use egui::{include_image, vec2, ImageSource};
use egui_widget_texicon::Texicon;
use egui_widget_themenator::ThemeName;

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
        let y = center_y - TEXI_HEIGHT / 2.0; // perfect vertical centering

        // ------------------------
        // Timing the Texicons loop
        // ------------------------
        #[cfg(not(target_arch = "wasm32"))]
        let start_time = std::time::Instant::now();

        // -----------------
        // Draw the Texicons
        // -----------------
        // === TEXICON #1 ===
        let mut idx = 0;
        let texicon = &TEXICONS[idx];
        let pos = egui::pos2(x, y);
        let texi_rect = egui::Rect::from_min_size(pos, texi_size);

        let resp = ui.put(
            texi_rect,
            Texicon::new(texicon.img.clone())
                .enabled(true)
                .selected(self.selected[idx])
                .img_size(vec2(80., 80.))
                .img_scale_hov(1.1)
                .text_size(13.)
                .sense(egui_widget_texicon::TexiSense::ImageAndText)
                .bkgnd_col(palette.red)
                .bkgnd_col_sel(palette.mauve)
                .bkgnd_col_hov(palette.mauve)
                .img_tint_col(palette.base)
                .img_tint_col_sel(palette.base)
                .img_tint_col_hov(palette.base)
                .text_col(palette.base)
                .text_col_sel(palette.base)
                .text_col_hov(palette.base)
                .frame_col(palette.base)
                .frame_col_sel(palette.teal)
                .frame_col_hov(palette.teal)
                .frame_size(vec2(100., 150.))
                .frame_width(4.)
                .tooltip_text(texicon.tooltip.to_string())
                .tooltip_gap(40.)
                .tooltip_position(egui::RectAlign::BOTTOM),
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
                .enabled(true)
                .selected(self.selected[idx])
                .img_size(vec2(48., 48.))
                .img_scale_hov(1.3)
                .sense(egui_widget_texicon::TexiSense::Frame)
                .bkgnd_col(palette.red.gamma_multiply_u8(8))
                .bkgnd_col_sel(palette.red.gamma_multiply_u8(24))
                .bkgnd_col_hov(palette.red.gamma_multiply_u8(64))
                .img_tint_col(palette.text)
                .img_tint_col_sel(palette.text)
                .img_tint_col_hov(palette.text)
                .frame_col(palette.crust)
                .frame_col_sel(palette.text)
                .frame_col_hov(palette.text)
                .frame_size(texi_size)
                .frame_width(2.)
                .radius(0)
                .tooltip_text(texicon.tooltip.to_string())
                .tooltip_gap(20.)
                .tooltip_position(egui::RectAlign::BOTTOM),
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
                .enabled(true)
                .selected(self.selected[idx])
                .img_size(vec2(40., 40.))
                .img_scale_hov(1.15)
                .text(texicon.text.to_string())
                .text_size(15.)
                .img_text_gap(10.)
                .bkgnd_col(palette.base)
                .bkgnd_col_sel(palette.mantle)
                .bkgnd_col_hov(palette.crust)
                .img_tint_col(palette.blue)
                .img_tint_col_sel(palette.blue)
                .img_tint_col_hov(palette.blue)
                .text_col(palette.teal)
                .text_col_sel(palette.teal)
                .text_col_hov(palette.teal)
                .frame_col(palette.crust)
                .frame_col_sel(palette.subtext0)
                .frame_col_hov(palette.subtext0)
                .frame_size(texi_size)
                .frame_width(2.)
                .radius(20)
                .tooltip_text(texicon.tooltip.to_string())
                .tooltip_gap(20.)
                .tooltip_position(egui::RectAlign::BOTTOM),
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
                .enabled(true)
                .selected(self.selected[idx])
                .img_size(vec2(50., 50.))
                .img_scale_hov(1.1)
                .text(texicon.text.to_string())
                .text_size(17.)
                .img_text_gap(6.)
                .bkgnd_col(palette.base)
                .bkgnd_col_sel(palette.mantle)
                .bkgnd_col_hov(palette.crust)
                .img_tint_col(palette.green)
                .img_tint_col_sel(palette.yellow)
                .img_tint_col_hov(palette.yellow)
                .text_col(palette.yellow)
                .text_col_sel(palette.green)
                .text_col_hov(palette.green)
                .frame_col(palette.crust)
                .frame_col_sel(palette.subtext0)
                .frame_col_hov(palette.subtext0)
                .frame_size(vec2(120., 100.))
                .frame_width(4.)
                .tooltip_text(texicon.tooltip.to_string())
                .tooltip_gap(20.)
                .tooltip_position(egui::RectAlign::BOTTOM),
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
                .enabled(false)
                .selected(self.selected[idx])
                .img_size(vec2(50., 50.))
                .text(texicon.text.to_string())
                .text_size(17.)
                .img_text_gap(0.)
                .bkgnd_col(palette.base)
                .bkgnd_col_sel(palette.mantle)
                .bkgnd_col_hov(palette.crust)
                .img_tint_col(palette.green)
                .img_tint_col_sel(palette.yellow)
                .img_tint_col_hov(palette.yellow)
                .text_col(palette.yellow)
                .text_col_sel(palette.green)
                .text_col_hov(palette.green)
                .frame_col(palette.crust)
                .frame_col_sel(palette.crust)
                .frame_col_hov(palette.crust)
                .frame_size(texi_size)
                .frame_width(4.)
                .tooltip_text(texicon.tooltip.to_string())
                .tooltip_gap(20.)
                .tooltip_position(egui::RectAlign::BOTTOM),
        );
        // Click response
        if resp.clicked() {
            self.selected = [false; NUM_TEXICONS];
            self.selected[idx] = true;
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
