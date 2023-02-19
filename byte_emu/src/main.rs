#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod emu;

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
            let mut app = app::ByteEmuApp::new(cc);

            if let Some(path) = args().nth(1) {
                let mut data = Vec::new();
                let mut file = File::open(path).expect("failed to open the file");
                file.read_to_end(&mut data)
                    .expect("failed to read the file");

                app.emu.load_program(&data, 0x0000);
            }

            Box::new(app)
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "byte_emu_app", // hardcode it
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(app::ByteEmuApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
