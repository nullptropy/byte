#![allow(unused)]

use crate::app::ByteEmuApp;
use byte_asm::lex::{Lexer, Location, Token, TokenType};
use egui::{text::LayoutJob, Color32};

impl ByteEmuApp {
    pub fn show_code_editor(&mut self, ctx: &egui::Context) {
        let mut open = self.state.is_code_editor_open;
        egui::Window::new("code editor")
            .open(&mut open)
            .default_size(egui::vec2(666.0, 588.0))
            .default_pos(egui::pos2(693.0, 216.0))
            .show(ctx, |ui| {
                self.ui_code_editor(ui);
            });
        self.state.is_code_editor_open = open;
    }

    fn ui_code_editor(&mut self, ui: &mut egui::Ui) {
        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job = highlight(ui.ctx(), string);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        egui::ScrollArea::both().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.state.text)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );
        });
    }
}

fn highlight(ctx: &egui::Context, string: &str) -> LayoutJob {
    type HighlightCache = egui::util::cache::FrameCache<LayoutJob, Highlighter>;

    impl egui::util::cache::ComputerMut<&str, LayoutJob> for Highlighter {
        fn compute(&mut self, string: &str) -> LayoutJob {
            self.highlight(string)
        }
    }

    ctx.memory_mut(|mem| mem.caches.cache::<HighlightCache>().get(string))
}

#[derive(Default)]
struct Highlighter {}

impl Highlighter {
    fn highlight(&self, string: &str) -> LayoutJob {
        let mut layout_job = LayoutJob::default();
        let mut lexer = Lexer::new(string);
        let mut prev_loc: Option<Location> = None;

        let mut append = |text: &str, color: Color32| {
            layout_job.append(
                text,
                0.0,
                egui::TextFormat::simple(egui::FontId::monospace(10.0), color),
            );
        };

        loop {
            match lexer.scan_token() {
                Ok(token) => {
                    if token.kind == TokenType::EndOfFile {
                        break;
                    }

                    match prev_loc {
                        None => {
                            append(&string[0..token.location.start], Color32::WHITE);
                        }
                        Some(ref prev) => {
                            if token.location.start - prev.end > 0 {
                                append(&string[prev.end..token.location.start], Color32::WHITE);
                            }
                        }
                    }

                    append(token.text, self.token_color(token.kind));
                    prev_loc = Some(token.location);
                }
                Err(why) => println!("Syntax Error: {why}"),
            }
        }

        layout_job
    }

    #[rustfmt::skip]
    fn token_color(&self, token_type: TokenType) -> Color32 {
        use TokenType::*;

        match token_type {
            Number     => Color32::from_rgb(0xd8, 0x98, 0xa4),
            String     => Color32::from_rgb(0x7b, 0xaf, 0x95),
            Comment    => Color32::from_rgb(0x6a, 0x6a, 0x69),

            DWDirective | DBDirective | OrgDirective | Include | Equ | Label => {
                Color32::from_rgb(0x63, 0xaa, 0xcf)
            }

            _ => Color32::WHITE,
        }
    }
}
