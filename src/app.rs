use egui::{Color32, FontId};
use egui_widget_texicon::TexiState;
use smallvec::SmallVec;

pub struct TexiconDemoApp {
    run_once: bool,
    texistate_vec_side: SmallVec<[TexiState; crate::texi_sidebar::NUM_TEXICONS]>,
    texistate_vec_top: SmallVec<[TexiState; crate::texi_sidebar::NUM_TEXICONS]>,
    texistate_vec_central: SmallVec<[TexiState; crate::texi_sidebar::NUM_TEXICONS]>,
}

impl Default for TexiconDemoApp {
    fn default() -> Self {
        Self {
            run_once: false,
            texistate_vec_side: SmallVec::new(),
            texistate_vec_top: SmallVec::new(),
            texistate_vec_central: SmallVec::new(),
        }
    }
}

impl TexiconDemoApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TexiconDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.run_once == false {
            self.run_once = true;

            crate::texi_sidebar::init_texicons(&mut self.texistate_vec_side);
            crate::texi_topbar::init_texicons(&mut self.texistate_vec_top);
            crate::texi_centralbar::init_texicons(&mut self.texistate_vec_central);
        }

        egui::TopBottomPanel::top("top_panel")
            .exact_height(150.)
            .resizable(false)
            .show(ctx, |ui| {
                crate::texi_topbar::draw_texicons(ui, &mut self.texistate_vec_top);
            });

        egui::SidePanel::left("left_panel")
            .default_width(300.)
            .resizable(false)
            .show(ctx, |ui| {
                crate::texi_sidebar::draw_texicons(ui, &mut self.texistate_vec_side);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Label::new(
                egui::RichText::new("Welcome to the Texicon Widget demo written in Rust and egui.")
                    .color(Color32::WHITE)
                    .font(FontId::new(24., egui::FontFamily::Proportional)),
            ));
            ui.add_space(10.);
            ui.add(egui::Label::new(
                egui::RichText::new(
                    "-- Texicons are highly configurable, e.g. images (svg, png), text, colors, fonts, sizes, spacings etc.",
                )
                .color(Color32::WHITE.gamma_multiply_u8(200))
                .font(FontId::new(18., egui::FontFamily::Proportional)),
            ));
            ui.add_space(4.);
            ui.add(egui::Label::new(
                egui::RichText::new(
                    "-- Mouse hover and click support dynamic behavior",
                )
                .color(Color32::WHITE.gamma_multiply_u8(200))
                .font(FontId::new(18., egui::FontFamily::Proportional)),
            ));
            ui.add_space(4.);
            ui.add(egui::Label::new(
                egui::RichText::new(
                    "-- Tooltips are supported with configurable positioning and gap",
                )
                .color(Color32::WHITE.gamma_multiply_u8(200))
                .font(FontId::new(18., egui::FontFamily::Proportional)),
            ));
            ui.add_space(4.);
            ui.add(egui::Label::new(
                egui::RichText::new(
                    "-- Texicons provide selected / non-selected state outside the widget",
                )
                .color(Color32::WHITE.gamma_multiply_u8(200))
                .font(FontId::new(18., egui::FontFamily::Proportional)),
            ));
            ui.add_space(4.);
            ui.add(egui::Label::new(
                egui::RichText::new(
                    "-- Text will wrap, or split into two lines while maintaining centering",
                )
                .color(Color32::WHITE.gamma_multiply_u8(200))
                .font(FontId::new(18., egui::FontFamily::Proportional)),
            ));

            ui.add_space(4.);
            ui.add(egui::Label::new(
                egui::RichText::new(
                    "-- Click / hover can be over the entire widget area or the image + text area",
                )
                .color(Color32::WHITE.gamma_multiply_u8(200))
                .font(FontId::new(18., egui::FontFamily::Proportional)),
            ));

            crate::texi_centralbar::draw_texicons(ui, &mut self.texistate_vec_central);
        });
    }
}
