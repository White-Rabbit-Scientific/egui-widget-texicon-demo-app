use egui::{Color32, FontId};
use egui_widget_texicon::TexiState;
use smallvec::SmallVec;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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

            ctx.set_visuals(egui::Visuals::dark());

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
            .exact_width(150.)
            .resizable(false)
            .show(ctx, |ui| {
                crate::texi_sidebar::draw_texicons(ui, &mut self.texistate_vec_side);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let welcome = format!("Welcome to the Texicon Widget Demo (version {}) written in Rust and egui.", VERSION);
            print_heading(ui, &welcome);
            print_bullets(ui, "-- Texicons are highly configurable, e.g. images (svg, png), text, colors, fonts, sizes, spacings etc.");
            print_bullets(ui, "-- Mouse hover and click support dynamic behavior");
            print_bullets(ui, "-- Mouse hover and click support dynamic behavior");
            print_bullets(ui, "-- Tooltips are supported with configurable positioning and gap");
            print_bullets(ui, "-- Texicons provide selected / non-selected state outside the widget");
            print_bullets(ui, "-- Text will wrap, or split into two lines while maintaining centering");
            print_bullets(ui, "-- Click / hover can be over the entire widget area or the image + text area");

            crate::texi_centralbar::draw_texicons(ui, &mut self.texistate_vec_central);
        });
    }
}

fn print_heading(ui: &mut egui::Ui, s: &str) {
    ui.add(egui::Label::new(
        egui::RichText::new(s)
            .color(Color32::WHITE)
            .font(FontId::new(24., egui::FontFamily::Proportional)),
    ));
    ui.add_space(10.);
}

fn print_bullets(ui: &mut egui::Ui, s: &str) {
    ui.add(egui::Label::new(
        egui::RichText::new(s)
            .color(Color32::WHITE.gamma_multiply_u8(200))
            .font(FontId::new(18., egui::FontFamily::Proportional)),
    ));
    ui.add_space(4.);
}
