use egui::FontId;
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
const BUILD_MODE: &str = "debug";
#[cfg(not(debug_assertions))]
const BUILD_MODE: &str = "release";

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
        let top_menu_benchmark = self.top_menu.get_benchmark();

        egui::SidePanel::left("left_panel")
            .exact_width(150.)
            .resizable(false)
            .show(ctx, |ui| {
                self.side_menu.draw_texicons(ui);
            });
        let side_menu_benchmark = self.side_menu.get_benchmark();

        egui::CentralPanel::default().show(ctx, |ui| {
            let central_menu_benchmark = self.central_menu.get_benchmark();
            ui.horizontal(|ui| {
                // Theme selector

                // let theme_config = egui_widget_themenator::ThemeConfig {
                //     variant: todo!(),
                //     title: todo!(),
                //     description: todo!(),
                //     icon: todo!(),
                // };

                const LATTE: egui_widget_themenator::ThemeConfig =
                    egui_widget_themenator::ThemeConfig {
                        variant: egui_widget_themenator::ThemeVariant::Latte,
                        title: "Latte:",
                        description: "A light theme",
                        icon: "\u{2600}",
                    };

                const FRAPPE: egui_widget_themenator::ThemeConfig =
                    egui_widget_themenator::ThemeConfig {
                        variant: egui_widget_themenator::ThemeVariant::Frappe,
                        title: "Frappe:",
                        description: "A dark theme",
                        icon: "\u{1F319}",
                    };

                const MACCHIATO: egui_widget_themenator::ThemeConfig =
                    egui_widget_themenator::ThemeConfig {
                        variant: egui_widget_themenator::ThemeVariant::Macchiato,
                        title: "Macchiato:",
                        description: "A darker theme",
                        icon: "\u{1F319}",
                    };

                const MOCHA: egui_widget_themenator::ThemeConfig =
                    egui_widget_themenator::ThemeConfig {
                        variant: egui_widget_themenator::ThemeVariant::Mocha,
                        title: "Mocha:",
                        description: "A darkerer theme",
                        icon: "\u{1F319}",
                    };

                let themenator = egui_widget_themenator::Themenator::new() //.default_themes_two();
                    .add(LATTE)
                    .add(FRAPPE)
                    .add(MACCHIATO)
                    .add(MOCHA);

                ui.add(themenator);

                // let themenator = egui_widget_themenator::Themenator::new().default_themes_four();
                // ui.add(themenator);

                ui.add_space(30.);
                ui.spacing_mut().item_spacing.x = 10.0;
                ui.label("Github repositories: ");
                ui.hyperlink_to(
                    "This demo app",
                    "https://github.com/White-Rabbit-Scientific/egui-widget-texicon-demo-app",
                );
                ui.label("  |  ");
                ui.hyperlink_to(
                    "Texicon widget",
                    "https://github.com/White-Rabbit-Scientific/egui-widget-texicon",
                );
                ui.label("  |  ");
                ui.hyperlink_to(
                    "Themenator theme widget",
                    "https://github.com/White-Rabbit-Scientific/egui-widget-themenator",
                );
                ui.label("  |  ");
                ui.hyperlink_to(
                    "egui demo app",
                    "https://github.com/White-Rabbit-Scientific/egui-demo-app",
                );
            });
            ui.add_space(10.);
            print_text(ui);
            print_benchmarks(
                ui,
                &top_menu_benchmark,
                &side_menu_benchmark,
                &central_menu_benchmark,
            );
            self.central_menu.draw_texicons(ui);
        });
    }
}

#[rustfmt::skip]
static BULLETS: &[&str] = &[
    "-- Texicons are highly configurable: images (svg, png), text, colors, fonts, sizes, spacings, scaling.",
    "-- Mouse hover and click support dynamic behavior, image enlargement on hover, tooltips.",
    "-- Click / hover response over the entire widget area or the image + text area.",
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
            "Texicon (Text + icon) Widget Demo (version {}) written in Rust and egui.",
            VERSION
        ),
    );

    for bullet in BULLETS {
        print_bullets(ui, bullet, TextStyle::Normal);
    }

    print_bullets(ui, RED_BULLETS[0], TextStyle::Warning);

    // Benchmarks
    print_heading(ui, "Timing benchmarks (for each group, non-wasm)");

    // ---------------
    // Print the stats
    // ---------------
    let s = get_os_info();
    let s = format!("{} compiled in {} mode.", s, BUILD_MODE);
    print_bullets(ui, &s, TextStyle::Normal);
}

fn print_heading(ui: &mut egui::Ui, s: &str) {
    ui.add_space(10.);
    ui.add(egui::Label::new(
        egui::RichText::new(s)
            .color(ui.visuals().error_fg_color)
            .font(FontId::new(24., egui::FontFamily::Proportional)),
    ));
    ui.add_space(4.);
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

fn print_benchmarks(
    ui: &mut egui::Ui,
    tm_bm: &crate::texi_top_menu::Benchmark,
    sm_bm: &crate::texi_side_menu::Benchmark,
    cm_bm: &crate::texi_central_menu::Benchmark,
) {
    let tm = format!(
        "> Top Menu       Count: {},  Sum: {} us,  Average: {:2.1} us",
        tm_bm.count, tm_bm.sum, tm_bm.average
    );
    let sm = format!(
        "> Side Menu      Count: {},  Sum: {} us,  Average: {:2.1} us",
        sm_bm.count, sm_bm.sum, sm_bm.average
    );
    let cm = format!(
        "> Central Menu   Count: {},  Sum: {} us,  Average: {:2.1} us",
        cm_bm.count, cm_bm.sum, cm_bm.average
    );
    ui.add(egui::Label::new(
        egui::RichText::new(tm)
            .color(ui.visuals().strong_text_color())
            .font(FontId::new(16., egui::FontFamily::Monospace)),
    ));
    ui.add_space(4.);
    ui.add(egui::Label::new(
        egui::RichText::new(sm)
            .color(ui.visuals().strong_text_color())
            .font(FontId::new(16., egui::FontFamily::Monospace)),
    ));
    ui.add_space(4.);
    ui.add(egui::Label::new(
        egui::RichText::new(cm)
            .color(ui.visuals().strong_text_color())
            .font(FontId::new(16., egui::FontFamily::Monospace)),
    ));
    ui.add_space(4.);
}

pub fn get_os_info() -> String {
    let info = os_info::get();
    format!(
        "{} {} ({} {:?})",
        info.os_type(),
        info.version(),
        info.bitness(),
        info.architecture().unwrap_or_default()
    )
}
