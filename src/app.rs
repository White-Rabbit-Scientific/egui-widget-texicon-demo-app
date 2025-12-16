use egui::FontId;
use egui_themes::ThemeWidget;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[rustfmt::skip]
pub struct TexiconDemoApp {
    top_menu: crate::texi_top_menu::TexiState,
    side_menu: crate::texi_side_menu::TexiState,
    central_menu: crate::texi_central_menu::TexiState,
}

impl TexiconDemoApp {
    pub fn new() -> Self {
        let mut app = Self {
            top_menu: crate::texi_top_menu::TexiState::new(),
            side_menu: crate::texi_side_menu::TexiState::new(),
            central_menu: crate::texi_central_menu::TexiState::new(),
        };
        app.top_menu.set_selected_texicon(0);
        app.side_menu.set_selected_texicon(0);
        app.central_menu.set_selected_texicon(0);
        app
    }
}

impl eframe::App for TexiconDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .exact_height(150.)
            .resizable(false)
            .show(ctx, |ui| {
                self.top_menu.draw_texicons(ui);
            });

        egui::SidePanel::left("left_panel")
            .exact_width(150.)
            .resizable(false)
            .show(ctx, |ui| {
                self.side_menu.draw_texicons(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Theme selector
            ui.add(ThemeWidget::new().label("Theme:").show_labels(false));
            ui.add_space(10.);
            print_text(ui);
            self.central_menu.draw_texicons(ui);
        });
    }
}

#[rustfmt::skip]
static BULLETS: &[&str] = &[
    "-- Texicon = Text + icon",
    "-- Texicons are highly configurable: images (svg, png), text, colors, fonts, sizes, spacings, scaling etc.",
    "-- Mouse hover and click support dynamic behavior",
    "-- Tooltips are supported with configurable positioning and gap",
    "-- Texicons provide selected / non-selected state outside the widget",
    "-- Text will wrap, or split into two lines while maintaining centering",
    "-- Click / hover can be over the entire widget area or the image + text area",
    "-- Try hovering and clicking on the Texicons to see their behavior",
];

#[rustfmt::skip]
static RED_BULLETS: &[&str] = &[
    "-- Try hovering and clicking on the Texicons to see their behavior",
];

enum TextStyle {
    Normal,
    Warning,
}

fn print_text(ui: &mut egui::Ui) {
    print_heading(
        ui,
        &format!(
            "Texicon Widget Demo (version {}) written in Rust and egui.",
            VERSION
        ),
    );

    for bullet in BULLETS {
        print_bullets(ui, bullet, TextStyle::Normal);
    }

    print_bullets(ui, RED_BULLETS[0], TextStyle::Warning);
}

fn print_heading(ui: &mut egui::Ui, s: &str) {
    ui.add(egui::Label::new(
        egui::RichText::new(s)
            .color(ui.visuals().error_fg_color)
            .font(FontId::new(24., egui::FontFamily::Proportional)),
    ));
    ui.add_space(10.);
}

fn print_bullets(ui: &mut egui::Ui, s: &str, text_style: TextStyle) {
    let color = match text_style {
        TextStyle::Normal => ui.visuals().strong_text_color(),
        TextStyle::Warning => ui.visuals().warn_fg_color,
    };
    ui.add(egui::Label::new(
        egui::RichText::new(s)
            .color(color)
            .font(FontId::new(18., egui::FontFamily::Proportional)),
    ));
    ui.add_space(4.);
}
