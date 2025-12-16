use egui::{include_image, vec2, ImageSource};
use egui_themes::ThemeName;
use egui_widget_texicon::Texicon;

// === Constants ===
#[rustfmt::skip] const TEXI_WIDTH: f32             = 140.0;
#[rustfmt::skip] const TEXI_HEIGHT: f32            = 120.0;
#[rustfmt::skip] const TEXI_GAP: f32               = 30.0;
#[rustfmt::skip] const IMG_SIZE: egui::Vec2        = vec2(70.0, 70.0);
#[rustfmt::skip] const BASE_TEXT_SIZE: f32         = 11.0;
#[rustfmt::skip] const TEXT_SIZE_INCREMENT: f32    = 2.0;
#[rustfmt::skip] const BASE_IMG_TEXT_GAP: f32      = 5.0;
#[rustfmt::skip] const IMG_TEXT_GAP_INCREMENT: f32 = 2.0;
#[rustfmt::skip] const IMG_SCALE_HOVER: f32        = 1.05;
#[rustfmt::skip] const FRAME_WIDTH: f32            = 2.0;
#[rustfmt::skip] const CORNER_RADIUS: u8           = 4;
#[rustfmt::skip] const TOOLTIP_GAP: f32            = 20.0;

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
        text:    "Undersized text",
        tooltip: "This is a tooltip for the test tube icon. Note the small font size.",
        uid:     "top_tt",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/clock.svg"),
        text:    "Normal text",
        tooltip: "This is a tooltip for the clock icon.",
        uid:     "top_ck",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/waves.svg"),
        text:    "Large text",
        tooltip: "This is a tooltip for the waves icon. Note the large font size.",
        uid:     "top_wv",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/gear-light.svg"),
        text:    "Extra large text",
        tooltip: "This is a tooltip for the gear icon. Note the extra large font size.",
        uid:     "top_gr",
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

        // Allocate space for the texicons
        let (_id, rect) =
            ui.allocate_space(egui::vec2(ui.available_width(), ui.available_height()));

        let center_x = rect.center().x;
        let center_y = rect.center().y;

        let texi_size = egui::vec2(TEXI_WIDTH, TEXI_HEIGHT);

        let total_width = TEXI_WIDTH * NUM_TEXICONS as f32 + TEXI_GAP * (NUM_TEXICONS as f32 - 1.0);
        let mut x = center_x - total_width / 2.0;
        let y = center_y - texi_size.y / 2.0; // perfect vertical centering

        // Pre-calculate colors to avoid duplication
        let green_dim = palette.green.gamma_multiply(0.5);

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
                    .texi_img_size(IMG_SIZE)
                    .texi_img_scale_hov(IMG_SCALE_HOVER)
                    .texi_text(Some(texicon.text.to_string()))
                    .texi_text_size(BASE_TEXT_SIZE + TEXT_SIZE_INCREMENT * idx as f32)
                    .texi_img_text_gap(BASE_IMG_TEXT_GAP + IMG_TEXT_GAP_INCREMENT * idx as f32)
                    .texi_top_gap(0.)
                    .texi_bottom_gap(0.)
                    .texi_bkgnd_col(palette.base)
                    .texi_bkgnd_col_sel(palette.mantle)
                    .texi_bkgnd_col_hov(palette.crust)
                    .texi_img_tint(green_dim)
                    .texi_img_tint_sel(palette.green)
                    .texi_img_tint_hov(palette.green)
                    .texi_text_col(green_dim)
                    .texi_text_col_sel(palette.green)
                    .texi_text_col_hov(palette.green)
                    .texi_frame_col(palette.base)
                    .texi_frame_col_sel(palette.base)
                    .texi_frame_col_hov(palette.surface2)
                    .texi_frame_size(texi_size)
                    .texi_frame_width(FRAME_WIDTH)
                    .texi_radius(CORNER_RADIUS)
                    .texi_tooltip_text(Some(texicon.tooltip.to_string()))
                    .texi_tooltip_gap(TOOLTIP_GAP)
                    .texi_tooltip_position(egui::RectAlign::BOTTOM),
            );
            // Click response
            if resp.clicked() {
                self.selected = [false; NUM_TEXICONS];
                self.selected[idx] = true;
            }
            // Hover response
            self.hovering[idx] = resp.hovered();

            x += TEXI_WIDTH + TEXI_GAP;
        }
    }
}
