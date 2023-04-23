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
        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let layout_job = highlight(ui.ctx(), string);
            ui.fonts(|f| f.layout_job(layout_job))
        };

        egui::ScrollArea::both().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.state.text)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(1)
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

        let font_id = egui::FontId::monospace(10.0);
        let mut prev_loc: Option<Location> = None;

        loop {
            match lexer.scan_token() {
                Ok(token) => {
                    if token.kind == TokenType::EndOfFile {
                        break;
                    }

                    if let Some(ref prev) = prev_loc {
                        if token.location.start - prev.end > 0 {
                            layout_job.append(
                                &string[prev.end..token.location.start],
                                0.0,
                                egui::TextFormat::simple(font_id.clone(), Color32::TRANSPARENT),
                            );
                        }
                    }

                    layout_job.append(
                        token.text,
                        0.0,
                        egui::TextFormat::simple(font_id.clone(), self.token_color(token.kind)),
                    );

                    prev_loc = Some(token.location);
                }
                Err(why) => println!("Syntax Error: {why}"),
            }
        }

        layout_job
    }

    // TODO: do this based on actual color themes
    // and add a mechanism to the highlighter
    // to be able to pick different color schemes
    fn token_color(&self, token_type: TokenType) -> Color32 {
        use TokenType::*;

        match token_type {
            DWDirective | DBDirective | OrgDirective | Include | Equ => {
                Color32::from_rgb(0x63, 0xaa, 0xcf)
            }

            Identifier => Color32::from_rgb(0x3d, 0xc1, 0xac),
            Number => Color32::from_rgb(0xd8, 0x98, 0xa4),
            String => Color32::from_rgb(0x7b, 0xaf, 0x95),
            Comment => Color32::from_rgb(0x6a, 0x6a, 0x69),

            _ => Color32::BLUE,
        }
    }
}
// LeftParen,
// RightParen,
// LeftBrace,
// RightBrace,
// Comma,
// Dot,
// Minus,
// Plus,
// Semicolon,
// Slash,
// Star,
// Hash,
// Bang,
// BangEqual,
// Equal,
// EqualEqual,
// Greater,
// GreaterEqual,
// Less,
// LessEqual,
// DollarSign,
// NumberSign,
// PercentSign,
// Colon,

// Identifier,
// OrgDirective,
// DBDirective,
// DWDirective,
// Include,

// String,
// Number,

// Comment,
// EndOfFile,
