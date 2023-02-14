#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ByteEmuApp {
    cycle: u64,
    text: String,
}

impl ByteEmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for ByteEmuApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
