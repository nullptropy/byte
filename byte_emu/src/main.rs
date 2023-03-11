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

    tracing_subscriber::fmt::init();

    eframe::run_native(
        "byte-emu",
        eframe::NativeOptions::default(),
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

            Box::new(app::ByteEmuApp::new(cc, program))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "byte_emu_app",
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(app::ByteEmuApp::new(cc, None))),
        )
        .await
        .expect("failed to start eframe");
    });
}
