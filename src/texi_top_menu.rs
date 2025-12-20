use egui::{include_image, vec2, ImageSource};
use egui_widget_texicon::Texicon;
use egui_widget_themenator::ThemeName;

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
}

#[rustfmt::skip]
const TEXICONS: [MyTexicon; 4] = [
    MyTexicon {
        img:     include_image!("../assets/pics/testtube.svg"),
        text:    "Undersized text",
        tooltip: "This is a tooltip for the test tube icon. Note the small font size.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/clock.svg"),
        text:    "Normal text",
        tooltip: "This is a tooltip for the clock icon.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/waves.svg"),
        text:    "Large text",
        tooltip: "This is a tooltip for the waves icon. Note the large font size.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/gear-light.svg"),
        text:    "Extra large text",
        tooltip: "This is a tooltip for the gear icon. Note the extra large font size.",
    },
];

const NUM_TEXICONS: usize = TEXICONS.len();

#[derive(Default, Clone)]
pub struct Benchmark {
    pub count: f32,
    pub sum: f32,
    pub average: f32,
}

#[derive(Clone, Default)]
pub struct TexiState {
    selected: [bool; NUM_TEXICONS],
    benchmark: Benchmark,
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

        // ------------------------
        // Timing the Texicons loop
        // ------------------------
        #[cfg(not(target_arch = "wasm32"))]
        let start_time = std::time::Instant::now();

        // -----------------
        // Draw the Texicons
        // -----------------
        for (idx, texicon) in TEXICONS.iter().enumerate() {
            let pos = egui::pos2(x, y);
            let texi_rect = egui::Rect::from_min_size(pos, texi_size);

            let resp = ui.put(
                texi_rect,
                Texicon::new(texicon.img.clone())
                    .enabled(true)
                    .selected(self.selected[idx])
                    .img_size(IMG_SIZE)
                    .img_scale_hov(IMG_SCALE_HOVER)
                    .text(texicon.text.to_string())
                    .text_size(BASE_TEXT_SIZE + TEXT_SIZE_INCREMENT * idx as f32)
                    .img_text_gap(BASE_IMG_TEXT_GAP + IMG_TEXT_GAP_INCREMENT * idx as f32)
                    .bkgnd_col(palette.base)
                    .bkgnd_col_sel(palette.mantle)
                    .bkgnd_col_hov(palette.crust)
                    .img_tint_col(green_dim)
                    .img_tint_col_sel(palette.green)
                    .img_tint_col_hov(palette.green)
                    .text_col(green_dim)
                    .text_col_sel(palette.green)
                    .text_col_hov(palette.green)
                    .frame_col(palette.base)
                    .frame_col_sel(palette.base)
                    .frame_col_hov(palette.surface2)
                    .frame_size(texi_size)
                    .frame_width(FRAME_WIDTH)
                    .radius(CORNER_RADIUS)
                    .tooltip_text(texicon.tooltip.to_string())
                    .tooltip_gap(TOOLTIP_GAP)
                    .tooltip_position(egui::RectAlign::BOTTOM),
            );
            // Click response
            if resp.clicked() {
                self.selected = [false; NUM_TEXICONS];
                self.selected[idx] = true;
            }

            x += TEXI_WIDTH + TEXI_GAP;
        }

        // ------------------------
        // Timing the Texicons loop
        // ------------------------
        #[cfg(not(target_arch = "wasm32"))]
        {
            let duration = start_time.elapsed();
            self.benchmark.count += 1.0;
            self.benchmark.sum += duration.as_micros() as f32;
            self.benchmark.average = self.benchmark.sum / self.benchmark.count;
        }
    }

    pub fn get_benchmark(&self) -> Benchmark {
        self.benchmark.clone()
    }
}
