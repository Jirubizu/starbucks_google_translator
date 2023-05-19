use crate::translator::language::Language;
use crate::translator::Translator;
use eframe::egui::{Context, FontDefinitions};
use eframe::{egui, CreationContext, Frame};
use egui::FontTweak;
use strum::IntoEnumIterator;

pub struct TranslateUi {
    translator: Translator,
}

impl TranslateUi {
    pub fn new(cc: &CreationContext) -> Self {
        let ctx = &cc.egui_ctx;
        let mut fonts = FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        let mut x =
            egui::FontData::from_static(include_bytes!("../../assets/NotoSansJP-Regular.ttf"));
        x.tweak = FontTweak {
            scale: 1.2,
            ..Default::default()
        };
        fonts.font_data.insert("my_font".to_owned(), x);

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());

        fonts.families.iter().for_each(|(family, fonts)| {
            println!("{}: {:?}", family, fonts);
        });

        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
        Self {
            translator: Translator::default(),
        }
    }
}

impl eframe::App for TranslateUi {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel")
            .min_height(270.0f32)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Translate From ComboBox
                    ui.horizontal(|ui| {
                        egui::ComboBox::new("from_combo", "")
                            .selected_text(self.translator.from.as_str())
                            .width(180.0)
                            .show_ui(ui, |ui| {
                                for local in Language::iter() {
                                    let text = local.as_str();
                                    if ui.selectable_label(false, text).clicked() {
                                        self.translator.from = local;
                                        self.translator.translate().unwrap();
                                    }
                                }
                            });
                    });
                    ui.add_space(5.0);
                    egui::ScrollArea::vertical()
                        .id_source("first")
                        .show(ui, |ui| {
                            // Input Text Area
                            let input_edit = ui.add_sized(
                                [960.0, 230.0],
                                egui::TextEdit::multiline(&mut self.translator.from_text),
                            );

                            // Event Handlers
                            if input_edit.changed() {
                                self.translator.translate().unwrap();
                            }
                        });
                });
            });

        egui::TopBottomPanel::bottom("bot_panel")
            .min_height(380.0f32)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    // Translate To ComboBox
                    ui.horizontal(|ui| {
                        egui::ComboBox::new("to_combo", "")
                            .width(180.0)
                            .selected_text(self.translator.to.as_str())
                            .show_ui(ui, |ui| {
                                for local in Language::iter() {
                                    let text = local.as_str();
                                    if ui.selectable_label(false, text).clicked() {
                                        self.translator.to = local;
                                        self.translator.translate().unwrap();
                                    }
                                }
                            });
                    });

                    egui::ScrollArea::vertical()
                        .max_width(900.0)
                        .show(ui, |ui| {
                            // Translated Text Area
                            let to_text_clone = self.translator.to_text.clone();
                            let res = to_text_clone.lock().unwrap();
                            ui.label(res.as_str());
                        });
                    ui.add_space(5.0);
                });
            });
    }
}
