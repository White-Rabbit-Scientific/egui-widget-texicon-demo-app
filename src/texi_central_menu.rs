use egui::{include_image, vec2, ImageSource};
use egui_themes::ThemeName;
use egui_widget_texicon::Texicon;

// === Constants ===
#[rustfmt::skip] const TEXI_WIDTH: f32  = 100.0;
#[rustfmt::skip] const TEXI_HEIGHT: f32 = 100.0;
#[rustfmt::skip] const TEXI_GAP: f32    = 30.0;

// === Texicon data ===
#[rustfmt::skip]
struct MyTexicon {
    img:     ImageSource<'static>,
    text:    &'static str,
    tooltip: &'static str,
}

#[rustfmt::skip]
const TEXICONS: [MyTexicon; 5] = [
    MyTexicon {
        img:     include_image!("../assets/pics/testtube.svg"),
        text:    "Experiments",
        tooltip: "No image and text provided. Mouseover senses text and icon, not frame.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/clock.svg"),
        text:    "Timing Stuff",
        tooltip: "Image provided, text set to None.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/waves.svg"),
        text:    "FILTERING",
        tooltip: "This is a tooltip for the waves icon.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/gear.svg"),
        text:    "Settings",
        tooltip: "This is a tooltip for the gear icon.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/article.png"),
        text:    "Documents (disabled)",
        tooltip: "This is a tooltip for the documents icon.",
    },
];

const NUM_TEXICONS: usize = TEXICONS.len();

#[derive(Clone, Default)]
pub struct TexiState {
    selected: [bool; NUM_TEXICONS],
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
        let y = center_y - TEXI_HEIGHT / 2.0; // perfect vertical centering

        // === TEXICON #1 ===
        let mut idx = 0;
        let texicon = &TEXICONS[idx];
        let pos = egui::pos2(x, y);
        let texi_rect = egui::Rect::from_min_size(pos, texi_size);

        let resp = ui.put(
            texi_rect,
            Texicon::new(texicon.img.clone())
                .texi_enabled(true)
                .texi_is_selected(self.selected[idx])
                .texi_img_size(vec2(80., 80.))
                .texi_img_scale_hov(1.1)
                .texi_text_size(13.)
                .texi_sense(egui_widget_texicon::TexiSense::ImageAndText)
                .texi_bkgnd_col(palette.red)
                .texi_bkgnd_col_sel(palette.mauve)
                .texi_bkgnd_col_hov(palette.mauve)
                .texi_img_tint_col(palette.base)
                .texi_img_tint_col_sel(palette.base)
                .texi_img_tint_col_hov(palette.base)
                .texi_text_col(palette.base)
                .texi_text_col_sel(palette.base)
                .texi_text_col_hov(palette.base)
                .texi_frame_col(palette.base)
                .texi_frame_col_sel(palette.base)
                .texi_frame_col_hov(palette.teal)
                .texi_frame_size(vec2(100., 150.))
                .texi_frame_width(4.)
                .texi_tooltip_text(Some(texicon.tooltip.to_string()))
                .texi_tooltip_gap(40.)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );
        // Click response
        if resp.clicked() {
            self.selected = [false; NUM_TEXICONS];
            self.selected[idx] = true;
        }

        // === TEXICON #2 ===
        x += TEXI_WIDTH + TEXI_GAP;
        idx += 1;
        let texicon = &TEXICONS[idx];
        let pos = egui::pos2(x, y);
        let texi_rect = egui::Rect::from_min_size(pos, texi_size);

        let resp = ui.put(
            texi_rect,
            Texicon::new(texicon.img.clone())
                .texi_enabled(true)
                .texi_is_selected(self.selected[idx])
                // .texi_img(texicon.img.clone())
                .texi_img_size(vec2(48., 48.))
                .texi_img_scale_hov(1.3)
                .texi_text(None)
                .texi_sense(egui_widget_texicon::TexiSense::Frame)
                .texi_bkgnd_col(palette.red.gamma_multiply_u8(8))
                .texi_bkgnd_col_sel(palette.red.gamma_multiply_u8(24))
                .texi_bkgnd_col_hov(palette.red.gamma_multiply_u8(64))
                .texi_img_tint_col(palette.text)
                .texi_img_tint_col_sel(palette.text)
                .texi_img_tint_col_hov(palette.text)
                .texi_frame_col(palette.crust)
                .texi_frame_col_sel(palette.text)
                .texi_frame_col_hov(palette.text)
                .texi_frame_size(texi_size)
                .texi_frame_width(2.)
                .texi_radius(0)
                .texi_tooltip_text(Some(texicon.tooltip.to_string()))
                .texi_tooltip_gap(20.)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );
        // Click response
        if resp.clicked() {
            self.selected = [false; NUM_TEXICONS];
            self.selected[idx] = true;
        }

        // === TEXICON #3 ===
        x += TEXI_WIDTH + TEXI_GAP;
        idx += 1;
        let texicon = &TEXICONS[idx];
        let pos = egui::pos2(x, y);
        let texi_rect = egui::Rect::from_min_size(pos, texi_size);

        let resp = ui.put(
            texi_rect,
            Texicon::new(texicon.img.clone())
                .texi_enabled(true)
                .texi_is_selected(self.selected[idx])
                // .texi_img(texicon.img.clone())
                .texi_img_size(vec2(40., 40.))
                .texi_img_scale_hov(1.15)
                .texi_text(Some(texicon.text.to_string()))
                .texi_text_size(15.)
                .texi_img_text_gap(10.)
                .texi_bkgnd_col(palette.base)
                .texi_bkgnd_col_sel(palette.mantle)
                .texi_bkgnd_col_hov(palette.crust)
                .texi_img_tint_col(palette.blue)
                .texi_img_tint_col_sel(palette.blue)
                .texi_img_tint_col_hov(palette.blue)
                .texi_text_col(palette.teal)
                .texi_text_col_sel(palette.teal)
                .texi_text_col_hov(palette.teal)
                .texi_frame_col(palette.crust)
                .texi_frame_col_sel(palette.subtext0)
                .texi_frame_col_hov(palette.subtext0)
                .texi_frame_size(texi_size)
                .texi_frame_width(2.)
                .texi_radius(20)
                .texi_tooltip_text(Some(texicon.tooltip.to_string()))
                .texi_tooltip_gap(20.)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );
        // Click response
        if resp.clicked() {
            self.selected = [false; NUM_TEXICONS];
            self.selected[idx] = true;
        }

        // === TEXICON #4 ===
        x += TEXI_WIDTH + TEXI_GAP;
        idx += 1;
        let texicon = &TEXICONS[idx];
        let pos = egui::pos2(x, y);
        let texi_rect = egui::Rect::from_min_size(pos, texi_size);

        let resp = ui.put(
            texi_rect,
            Texicon::new(texicon.img.clone())
                .texi_enabled(true)
                .texi_is_selected(self.selected[idx])
                // .texi_img(texicon.img.clone())
                .texi_img_size(vec2(50., 50.))
                .texi_img_scale_hov(1.1)
                .texi_text(Some(texicon.text.to_string()))
                .texi_text_size(17.)
                .texi_img_text_gap(6.)
                .texi_bkgnd_col(palette.base)
                .texi_bkgnd_col_sel(palette.mantle)
                .texi_bkgnd_col_hov(palette.crust)
                .texi_img_tint_col(palette.green)
                .texi_img_tint_col_sel(palette.yellow)
                .texi_img_tint_col_hov(palette.yellow)
                .texi_text_col(palette.yellow)
                .texi_text_col_sel(palette.green)
                .texi_text_col_hov(palette.green)
                .texi_frame_col(palette.crust)
                .texi_frame_col_sel(palette.subtext0)
                .texi_frame_col_hov(palette.subtext0)
                .texi_frame_size(vec2(120., 100.))
                .texi_frame_width(4.)
                .texi_tooltip_text(Some(texicon.tooltip.to_string()))
                .texi_tooltip_gap(20.)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );
        // Click response
        if resp.clicked() {
            self.selected = [false; NUM_TEXICONS];
            self.selected[idx] = true;
        }

        // === TEXICON #5 ===
        x += TEXI_WIDTH + TEXI_GAP;
        idx += 1;
        let texicon = &TEXICONS[idx];
        let pos = egui::pos2(x, y);
        let texi_rect = egui::Rect::from_min_size(pos, texi_size);

        let resp = ui.put(
            texi_rect,
            Texicon::new(texicon.img.clone())
                .texi_enabled(false)
                .texi_is_selected(self.selected[idx])
                // .texi_img(texicon.img.clone())
                .texi_img_size(vec2(50., 50.))
                .texi_text(Some(texicon.text.to_string()))
                .texi_text_size(17.)
                .texi_img_text_gap(0.)
                .texi_bkgnd_col(palette.base)
                .texi_bkgnd_col_sel(palette.mantle)
                .texi_bkgnd_col_hov(palette.crust)
                .texi_img_tint_col(palette.green)
                .texi_img_tint_col_sel(palette.yellow)
                .texi_img_tint_col_hov(palette.yellow)
                .texi_text_col(palette.yellow)
                .texi_text_col_sel(palette.green)
                .texi_text_col_hov(palette.green)
                .texi_frame_col(palette.crust)
                .texi_frame_col_sel(palette.crust)
                .texi_frame_col_hov(palette.crust)
                .texi_frame_size(texi_size)
                .texi_frame_width(4.)
                .texi_tooltip_text(Some(texicon.tooltip.to_string()))
                .texi_tooltip_gap(20.)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );
        // Click response
        if resp.clicked() {
            self.selected = [false; NUM_TEXICONS];
            self.selected[idx] = true;
        }
    }
}
