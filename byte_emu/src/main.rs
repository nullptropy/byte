#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    eframe::run_native(
        "byte-emu",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(byte_emu::ByteEmuApp::new(cc))),
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
            Box::new(|cc| Box::new(byte_emu::ByteEmuApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}