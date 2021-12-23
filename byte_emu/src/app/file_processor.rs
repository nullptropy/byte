use std::sync::mpsc;

#[derive(Debug)]
pub struct FileProcesser<T> {
    tx: mpsc::Sender<T>,
    rx: mpsc::Receiver<T>,
}

impl<T> FileProcesser<T>
where
    T: Send + 'static,
{
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }

    pub fn consume_messages(&self) -> Vec<T> {
        let mut messages = Vec::new();

        while let Ok(fm) = self.rx.try_recv() {
            messages.push(fm);
        }

        messages
    }

    pub fn read<F>(&mut self, message_fn: F)
    where
        F: FnOnce(String, Vec<u8>) -> T,
        F: Send + 'static,
    {
        let tx = self.tx.clone();

        execute(async move {
            if let Some(file) = rfd::AsyncFileDialog::new().pick_file().await {
                let name = file.file_name();
                let data = file.read().await;

                // ignore the error
                tx.send(message_fn(name, data)).ok();
            }
        });
    }
}

use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    use pollster::FutureExt as _;
    std::thread::spawn(move || f.block_on());
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
