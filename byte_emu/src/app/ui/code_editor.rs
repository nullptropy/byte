use egui::{text::LayoutJob, Color32};
use std::collections::HashSet;

use crate::app::ByteEmuApp;
use byte_asm::lex::{Lexer, Token, TokenType};
use byte_common::opcode::get_opcode;

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
        let mut selected = self.state.code_editor_theme;
        egui::ComboBox::from_label("Select Theme")
            .selected_text(format!("{selected:?}"))
            .show_ui(ui, |ui: &mut egui::Ui| {
                ui.selectable_value(&mut selected, Theme::Default, "Default");
                ui.selectable_value(&mut selected, Theme::EmbersLight, "EmbersLight");
            });
        self.state.code_editor_theme = selected;

        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job = highlight(ui.ctx(), string, self.state.code_editor_theme);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.separator();
        egui::ScrollArea::both().show(ui, |ui| {
            ui.style_mut().visuals.extreme_bg_color = self
                .state
                .code_editor_theme
                .colorize(HighlighterType::Background);
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
                    append(
                        &src[0..token.location.start],
                        theme.colorize(HighlighterType::Generic),
                    );
                }
                Some(prev) => {
                    if token.location.start - prev.location.end > 0 {
                        append(
                            &src[prev.location.end..token.location.start],
                            theme.colorize(HighlighterType::Generic),
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

pub enum HighlighterType {
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

#[rustfmt::skip]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Theme {
    Default,
    EmbersLight,
}

impl Theme {
    pub fn colorize(&self, kind: HighlighterType) -> Color32 {
        use Theme::*;

        match self {
            Default => self.default_theme(kind),
            EmbersLight => self.embers_light_theme(kind),
        }
    }
}

impl Theme {
    fn embers_light_theme(&self, kind: HighlighterType) -> Color32 {
        use HighlighterType::*;

        match kind {
            Background => self.color(0xdbd6d1),
            Comment => self.color(0xb19b90),
            Generic => self.color(0x433b32),
            Instruction => self.color(0x648a77),
            Keyword => self.color(0x6d638c),
            Label => self.color(0x6d8257),
            Number => self.color(0x8b7586),
            String => self.color(0x68858a),
            Variable => self.color(0x8e8a70),
        }
    }

    fn default_theme(&self, kind: HighlighterType) -> Color32 {
        use HighlighterType::*;

        match kind {
            Background => self.color(0x0a0a0a),
            Comment => self.color(0x6a6a69),
            Generic => self.color(0xffffff),
            Instruction => self.color(0xffc591),
            Keyword => self.color(0x63aacf),
            Label => self.color(0x72975f),
            Number => self.color(0xd898a4),
            String => self.color(0x7baf95),
            Variable => self.color(0x96ced8),
        }
    }

    fn color(&self, color: usize) -> Color32 {
        Color32::from_rgb((color >> 16) as u8, (color >> 8) as u8, color as u8)
    }
}
