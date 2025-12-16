use egui::{include_image, ImageSource};
use egui_themes::ThemeName;
use egui_widget_texicon::Texicon;

// === Constants ===
#[rustfmt::skip] const TEXI_WIDTH: f32      = 70.0;
#[rustfmt::skip] const TEXI_HEIGHT: f32     = 80.0;
#[rustfmt::skip] const TEXI_GAP: f32        = 30.0;
#[rustfmt::skip] const IMG_SCALE_HOVER: f32 = 1.10;
#[rustfmt::skip] const FRAME_WIDTH: f32     = 2.0;
#[rustfmt::skip] const TOOLTIP_GAP: f32     = 20.0;

// === Texicon data ===
#[rustfmt::skip]
struct MyTexicon {
    img:     ImageSource<'static>,
    text:    &'static str,
    tooltip: &'static str,
    uid:     &'static str,
}

#[rustfmt::skip]
const TEXICONS: [MyTexicon; 4] = [
    MyTexicon {
        img:     include_image!("../assets/pics/testtube.svg"),
        text:    "Experiments",
        tooltip: "Text wrapping and centering for long words.",
        uid:     "ls_tt",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/clock.svg"),
        text:    "Timing Stuff",
        tooltip: "Text wrapping and centering for multiple words.",
        uid:     "ls_ck",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/waves.svg"),
        text:    "Filtering",
        tooltip: "This is a tooltip for the waves Texicon.",
        uid:     "ls_wv",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/gear-light.svg"),
        text:    "Settings",
        tooltip: "This is a tooltip for the gear Texicon.",
        uid:     "ls_gr",
    },
];

const NUM_TEXICONS: usize = TEXICONS.len();

#[derive(Clone, Default)]
pub struct TexiState {
    selected: [bool; NUM_TEXICONS],
    hovering: [bool; NUM_TEXICONS],
}

impl TexiState {
    pub fn new() -> Self {
        Self::default()
    }
    // Set a texicon to selected state (at startup)
    pub fn set_selected_texicon(&mut self, index: usize) {
        if index < NUM_TEXICONS {
            self.selected[index] = true;
        }
    }
    pub fn draw_texicons(&mut self, ui: &mut egui::Ui) {
        // Get current theme color palette
        let palette = ThemeName::current(ui.ctx()).palette();

        ui.add_space(20.);

        // Allocate space for the texicons
        let (_id, rect) =
            ui.allocate_space(egui::vec2(ui.available_width(), ui.available_height()));

        let texi_size = egui::vec2(TEXI_WIDTH, TEXI_HEIGHT);

        let x = rect.center().x - TEXI_WIDTH / 2.0;
        let mut y = rect.min.y;

        // Pre-calculate colors to avoid duplication
        let text_dim = palette.text.gamma_multiply(0.5);

        // Usage
        for (idx, texicon) in TEXICONS.iter().enumerate() {
            let pos = egui::pos2(x, y);
            let texi_rect = egui::Rect::from_min_size(pos, texi_size);

            let resp = ui.put(
                texi_rect,
                Texicon::new(texicon.uid)
                    .texi_enabled(true)
                    .texi_is_selected(self.selected[idx])
                    .texi_is_hovered(self.hovering[idx])
                    .texi_img(texicon.img.clone())
                    .texi_img_scale_hov(IMG_SCALE_HOVER)
                    .texi_text(Some(texicon.text.to_string()))
                    .texi_bkgnd_col(palette.base)
                    .texi_bkgnd_col_sel(palette.crust)
                    .texi_bkgnd_col_hov(palette.crust)
                    .texi_img_tint(text_dim)
                    .texi_img_tint_sel(palette.text)
                    .texi_img_tint_hov(palette.mauve)
                    .texi_text_col(text_dim)
                    .texi_text_col_sel(palette.text)
                    .texi_text_col_hov(palette.mauve)
                    .texi_frame_col(palette.surface0)
                    .texi_frame_col_sel(palette.overlay0)
                    .texi_frame_col_hov(palette.mauve)
                    .texi_frame_size(texi_size)
                    .texi_frame_width(FRAME_WIDTH)
                    .texi_tooltip_text(Some(texicon.tooltip.to_string()))
                    .texi_tooltip_gap(TOOLTIP_GAP),
            );
            // Click response
            if resp.clicked() {
                self.selected = [false; NUM_TEXICONS];
                self.selected[idx] = true;
            }
            // Hover response
            self.hovering[idx] = resp.hovered();

            y += TEXI_HEIGHT + TEXI_GAP;
        }
    }
}
