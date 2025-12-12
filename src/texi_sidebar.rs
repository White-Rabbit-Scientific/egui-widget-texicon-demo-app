use egui::{include_image, vec2, Color32, ImageSource};
use egui_themes::ThemeName;
use egui_widget_texicon::{TexiState, Texicon};
use smallvec::SmallVec;

// === Texicon images -- update as required ===
//
// Define the number of Texicons
pub const NUM_TEXICONS: usize = 4;

// Texicom images (stored in the binary file)
static TEXI_IMGS: &[ImageSource<'static>] = &[IMG_TESTTUBE, IMG_CLOCK, IMG_WAVES, IMG_GEAR];
#[rustfmt::skip] const IMG_TESTTUBE: ImageSource<'static> = include_image!("../assets/pics/testtube.svg");
#[rustfmt::skip] const IMG_CLOCK:    ImageSource<'static> = include_image!("../assets/pics/clock.svg");
#[rustfmt::skip] const IMG_WAVES:    ImageSource<'static> = include_image!("../assets/pics/waves.svg");
#[rustfmt::skip] const IMG_GEAR:     ImageSource<'static> = include_image!("../assets/pics/gear.svg");

// Texicon text
static TEXI_TEXT: &[&str] = &[TXT_TEXTTUBE, TXT_CLOCK, TXT_WAVES, TXT_GEAR];
#[rustfmt::skip] const TXT_TEXTTUBE: &'static str = "Experiments";
#[rustfmt::skip] const TXT_CLOCK:    &'static str = "Timing Stuff";
#[rustfmt::skip] const TXT_WAVES:    &'static str = "Filtering";
#[rustfmt::skip] const TXT_GEAR:     &'static str = "Settings";

// Texicon tooltips
static TOOLTIPS: &[&str] = &[TT_TEXTTUBE, TT_CLOCK, TT_WAVES, TT_GEAR];
#[rustfmt::skip] const TT_TEXTTUBE: &'static str = "Text wrapping and centering for long words.";
#[rustfmt::skip] const TT_CLOCK:    &'static str = "Text wrapping and centering for short words";
#[rustfmt::skip] const TT_WAVES:    &'static str = "Faint outline for non-selected Texicons.";
#[rustfmt::skip] const TT_GEAR:     &'static str = "This is a tooltip for the gear Texicon.";

pub fn init_texicons(texistates: &mut SmallVec<[TexiState; NUM_TEXICONS]>) {
    // Create shared state for each Texicon
    for _ in 0..NUM_TEXICONS {
        texistates.push(TexiState::default());
    }
    // Set Texicon index 0 to selected state
    texistates[0].texi_selected = true;
}

pub fn draw_texicons(ui: &mut egui::Ui, texistates: &mut SmallVec<[TexiState; NUM_TEXICONS]>) {
    // Get current theme color palette
    let palette = ThemeName::current(ui.ctx()).palette();

    ui.vertical_centered(|ui| {
        ui.add_space(20.);
        // Loop
        for (texi_vec_index, _img) in TEXI_IMGS.iter().enumerate() {
            if let Some(texi) = texistates.get_mut(texi_vec_index) {
                let resp = ui.add(
                    Texicon::new(texi)
                        .texi_frame_size(vec2(70., 80.))
                        .texi_img_scale_hov(1.05)
                        .texi_img(TEXI_IMGS[texi_vec_index].clone())
                        .texi_text(Some(TEXI_TEXT[texi_vec_index].to_string()))
                        .texi_img(TEXI_IMGS[texi_vec_index].clone())
                        .texi_bkgnd_col(palette.base)
                        .texi_bkgnd_col_sel(palette.crust)
                        .texi_bkgnd_col_hov(palette.crust)
                        .texi_img_tint(palette.text.gamma_multiply(0.5))
                        .texi_img_tint_sel(palette.text)
                        .texi_img_tint_hov(palette.text)
                        .texi_text_col(palette.text.gamma_multiply(0.5))
                        .texi_text_col_sel(palette.text)
                        .texi_text_col_hov(palette.text)
                        .texi_frame_col(palette.mantle)
                        .texi_frame_col_sel(palette.overlay0)
                        .texi_frame_col_hov(palette.overlay2)
                        .texi_frame_width(2.)
                        .texi_tooltip_text(Option::Some(TOOLTIPS[texi_vec_index].to_string()))
                        .texi_tooltip_gap(20.),
                );
                if resp.clicked() {
                    texistates.iter_mut().for_each(|t| t.texi_selected = false);
                    texistates[texi_vec_index].texi_selected = true;
                }
            }
        }
    });
}
