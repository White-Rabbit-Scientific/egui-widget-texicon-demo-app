use egui::{include_image, ImageSource};
use egui_widget_texicon::Texicon;
use egui_widget_themenator::ThemeName;

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
}

#[rustfmt::skip]
const TEXICONS: [MyTexicon; 4] = [
    MyTexicon {
        img:     include_image!("../assets/pics/testtube.svg"),
        text:    "Experiments",
        tooltip: "Text wrapping and centering for long words.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/clock.svg"),
        text:    "Timing Stuff",
        tooltip: "Text wrapping and centering for multiple words.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/waves.svg"),
        text:    "Filtering",
        tooltip: "This is a tooltip for the waves Texicon.",
    },
    MyTexicon {
        img:     include_image!("../assets/pics/gear-light.svg"),
        text:    "Settings",
        tooltip: "This is a tooltip for the gear Texicon.",
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

        ui.add_space(20.);

        // Allocate space for the texicons
        let (_id, rect) =
            ui.allocate_space(egui::vec2(ui.available_width(), ui.available_height()));

        let texi_size = egui::vec2(TEXI_WIDTH, TEXI_HEIGHT);

        let x = rect.center().x - TEXI_WIDTH / 2.0;
        let mut y = rect.min.y;

        // Pre-calculate colors to avoid duplication
        let text_dim = palette.text.gamma_multiply(0.5);

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
                    .img_scale_hov(IMG_SCALE_HOVER)
                    .text(texicon.text.to_string())
                    .bkgnd_col(palette.base)
                    .bkgnd_col_sel(palette.crust)
                    .bkgnd_col_hov(palette.crust)
                    .img_tint_col(text_dim)
                    .img_tint_col_sel(palette.text)
                    .img_tint_col_hov(palette.mauve)
                    .text_col(text_dim)
                    .text_col_sel(palette.text)
                    .text_col_hov(palette.mauve)
                    .frame_col(palette.surface0)
                    .frame_col_sel(palette.overlay0)
                    .frame_col_hov(palette.mauve)
                    .frame_size(texi_size)
                    .frame_width(FRAME_WIDTH)
                    .tooltip_text(texicon.tooltip.to_string())
                    .tooltip_gap(TOOLTIP_GAP),
            );
            // Click response
            if resp.clicked() {
                self.selected = [false; NUM_TEXICONS];
                self.selected[idx] = true;
            }

            y += TEXI_HEIGHT + TEXI_GAP;
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
