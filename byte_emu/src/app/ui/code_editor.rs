#![allow(unused)]

use egui::{text::LayoutJob, Color32};
use std::collections::HashSet;

use crate::app::ByteEmuApp;
use byte_asm::lex::{Lexer, LexerResult, Location, Token, TokenType};
use byte_common::opcode::{get_opcode, OPCODE_MAP};

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
            let mut layout_job = highlight(ui.ctx(), string, Theme::Default);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        egui::ScrollArea::both().show(ui, |ui| {
            ui.style_mut().visuals.extreme_bg_color =
                Theme::Default.default_theme(HighlighterType::Background);
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

fn highlight(ctx: &egui::Context, string: &str, theme: Theme) -> LayoutJob {
    type HighlightCache = egui::util::cache::FrameCache<LayoutJob, Highlighter>;

    impl egui::util::cache::ComputerMut<(&str, Theme), LayoutJob> for Highlighter {
        fn compute(&mut self, data: (&str, Theme)) -> LayoutJob {
            let (string, theme) = data;
            self.highlight(string, theme)
        }
    }

    ctx.memory_mut(|mem| mem.caches.cache::<HighlightCache>().get((string, theme)))
}

#[derive(Default)]
struct Highlighter;

impl Highlighter {
    fn scan_tokens(&self, src: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(src);
        let mut tokens = Vec::new();

        loop {
            match lexer.scan_token() {
                Ok(token) if token.eof() => break,
                Ok(token) => tokens.push(token),
                // handle syntax errors here
                _ => (),
            }
        }

        tokens
    }

    fn process_tokens(&self, src: &str, tokens: Vec<Token>) -> Vec<(HighlighterType, Token)> {
        use TokenType::*;

        let mut tokens_iter = tokens.iter().peekable();
        let mut label_table = HashSet::new();
        let mut variable_table = HashSet::new();

        #[allow(clippy::while_let_on_iterator)]
        while let Some(token) = tokens_iter.next() {
            if token.kind != TokenType::Identifier {
                continue;
            }

            if let Some(next_token) = tokens_iter.peek() {
                if next_token.kind == TokenType::Colon {
                    label_table.insert(token.text(src));
                } else if next_token.kind == TokenType::Equ {
                    variable_table.insert(token.text(src));
                }
            }
        }

        let mut tokens_iter = tokens.into_iter().peekable();
        let mut result = Vec::new();

        #[allow(clippy::while_let_on_iterator)]
        while let Some(token) = tokens_iter.next() {
            let kind = match token.kind {
                Number => HighlighterType::Number,
                String => HighlighterType::String,
                Comment => HighlighterType::Comment,

                DWDirective | DBDirective | OrgDirective | Include | Equ => {
                    HighlighterType::Keyword
                }

                Identifier => {
                    if label_table.contains(token.text(src)) {
                        HighlighterType::Label
                    } else if variable_table.contains(token.text(src)) {
                        HighlighterType::Variable
                    } else {
                        get_opcode(token.text(src))
                            .map_or(HighlighterType::Generic, |_| HighlighterType::Instruction)
                    }
                }

                _ => HighlighterType::Generic,
            };

            result.push((kind, token));
        }

        result
    }

    fn highlight(&self, src: &str, theme: Theme) -> LayoutJob {
        let tokens = self.scan_tokens(src);
        let processed_tokens = self.process_tokens(src, tokens);

        let mut layout_job = LayoutJob::default();
        let mut prev: Option<Token> = None;

        let mut append = |text: &str, color: Color32| {
            layout_job.append(
                text,
                0.0,
                egui::TextFormat::simple(egui::FontId::monospace(10.0), color),
            );
        };

        for (kind, token) in processed_tokens {
            match prev {
                None => {
                    append(&src[0..token.location.start], Color32::WHITE);
                }
                Some(prev) => {
                    if token.location.start - prev.location.end > 0 {
                        append(
                            &src[prev.location.end..token.location.start],
                            Color32::WHITE,
                        );
                    }
                }
            }

            append(token.text(src), theme.colorize(kind));
            prev = Some(token);
        }

        layout_job
    }
}

enum HighlighterType {
    Background,
    Comment,
    // generic?
    Generic,
    Instruction,
    Keyword,
    Label,
    Number,
    String,
    Variable,
}

#[derive(Debug, Clone, Copy, Hash)]
enum Theme {
    Default,
    Base16,
}

impl Theme {
    fn colorize(&self, kind: HighlighterType) -> Color32 {
        use Theme::*;

        match self {
            Base16 => self.base16_theme(kind),
            Default => self.default_theme(kind),
        }
    }
}

impl Theme {
    pub fn base16_theme(&self, kind: HighlighterType) -> Color32 {
        todo!()
    }

    pub fn default_theme(&self, kind: HighlighterType) -> Color32 {
        use HighlighterType::*;

        match kind {
            Background => Color32::from_rgb(0x0a, 0x0a, 0x0a),
            Comment => Color32::from_rgb(0x6a, 0x6a, 0x69),
            Generic => Color32::from_rgb(0xff, 0xff, 0xff),
            Instruction => Color32::from_rgb(0xff, 0xc5, 0x91),
            Keyword => Color32::from_rgb(0x63, 0xaa, 0xcf),
            Label => Color32::from_rgb(0x72, 0x97, 0x5f),
            Number => Color32::from_rgb(0xd8, 0x98, 0xa4),
            String => Color32::from_rgb(0x7b, 0xaf, 0x95),
            Variable => Color32::from_rgb(0x96, 0xce, 0xd8),
        }
    }
}
