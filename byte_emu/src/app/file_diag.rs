use std::sync::mpsc;

#[derive(Debug)]
pub enum FileProcesserMessage {
    BinaryFileOpen(String),
    SourceFileOpen(String),
}

#[derive(Debug)]
pub struct FileProcesser {
    ch: (
        mpsc::Sender<FileProcesserMessage>,
        mpsc::Receiver<FileProcesserMessage>,
    ),
}

impl Default for FileProcesser {
    fn default() -> Self {
        Self {
            ch: mpsc::channel(),
        }
    }
}

impl FileProcesser {
    pub fn consume_messages(&self) -> Vec<FileProcesserMessage> {
        let mut messages = Vec::new();

        loop {
            match self.ch.1.try_recv() {
                Ok(fm) => messages.push(fm),
                Err(_) => break,
            }
        }

        messages
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // `false` for BinaryFileOpen
        //  `true` for SourceFileOpen
        let mut clicked_button: Option<bool> = None;
        let binary_open_button = ui.button("Load binary file");
        let source_open_button = ui.button("Load source file");

        if binary_open_button.clicked() {
            clicked_button = Some(false)
        }
        if source_open_button.clicked() {
            clicked_button = Some(true)
        }

        if let Some(clicked) = clicked_button {
            let tx = self.ch.0.clone();
            execute(async move {
                if let Some(file) = rfd::AsyncFileDialog::new().pick_file().await {
                    let message = if clicked {
                        FileProcesserMessage::SourceFileOpen(file.file_name())
                    } else {
                        FileProcesserMessage::BinaryFileOpen(file.file_name())
                    };

                    tx.send(message).ok();
                }
            });
        }
    }
}

use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
