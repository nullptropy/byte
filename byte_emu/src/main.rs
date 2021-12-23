#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod emu;

const DEFAULT_BINARY: &[u8; 1 << 16] = include_bytes!("../assets/demo.bin");
const DEFAULT_SOURCE: &str = include_str!("../assets/demo.s");

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use std::env::args;
    use std::fs::File;
    use std::io::Read;

    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "byte-emu",
        native_options,
        Box::new(|cc| {
            let program = match args().nth(1) {
                Some(path) => {
                    let mut data = Vec::new();
                    let mut file = File::open(path).expect("failed to open the file");
                    file.read_to_end(&mut data)
                        .expect("failed to read the file");

                    Some((data, 0x000))
                }
                None => None,
            };

            Ok(Box::new(app::ByteEmuApp::new(cc, program)))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("byte_emu_app")
            .expect("Failed to find byte_emu_app")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("byte_emu_app was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(app::ByteEmuApp::new(cc, None)))),
            )
            .await;

        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p>The app has crashed. See the developer console for details.</p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
