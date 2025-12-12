use egui::{include_image, Color32, ImageSource};
use egui_themes::ThemeName;
use egui_widget_texicon::{TexiState, Texicon};
use smallvec::SmallVec;

// === Texicon images -- update as required ===
//
// Define the number of Texicons
pub const NUM_TEXICONS: usize = 5;

// Texicom images (stored in the binary file)
static TEXI_IMGS: &[ImageSource<'static>] =
    &[IMG_TESTTUBE, IMG_CLOCK, IMG_WAVES, IMG_GEAR, IMG_ARTICLE];
#[rustfmt::skip] const IMG_TESTTUBE: ImageSource<'static> = include_image!("../assets/pics/testtube.svg");
#[rustfmt::skip] const IMG_CLOCK:    ImageSource<'static> = include_image!("../assets/pics/clock.svg");
#[rustfmt::skip] const IMG_WAVES:    ImageSource<'static> = include_image!("../assets/pics/waves.svg");
#[rustfmt::skip] const IMG_GEAR:     ImageSource<'static> = include_image!("../assets/pics/gear.svg");
#[rustfmt::skip] const IMG_ARTICLE:  ImageSource<'static> = include_image!("../assets/pics/article.png");

// Texicon text
static TEXI_TEXT: &[&str] = &[TXT_TEXTTUBE, TXT_CLOCK, TXT_WAVES, TXT_GEAR, TXT_ARTICLE];
#[rustfmt::skip] const TXT_TEXTTUBE: &'static str = "Experiments";
#[rustfmt::skip] const TXT_CLOCK:    &'static str = "Timing Stuff";
#[rustfmt::skip] const TXT_WAVES:    &'static str = "FILTERING";
#[rustfmt::skip] const TXT_GEAR:     &'static str = "Settings";
#[rustfmt::skip] const TXT_ARTICLE:  &'static str = "Documents (disabled)";

// Texicon tooltips
static TOOLTIPS: &[&str] = &[TT_TEXTTUBE, TT_CLOCK, TT_WAVES, TT_GEAR, TT_ARTICLE];
#[rustfmt::skip] const TT_TEXTTUBE: &'static str = "No image and text provided. Mouseover senses text and icon, not frame.";
#[rustfmt::skip] const TT_CLOCK:    &'static str = "Image provided, text set to None.";
#[rustfmt::skip] const TT_WAVES:    &'static str = "This is a tooltip for the waves icon.";
#[rustfmt::skip] const TT_GEAR:     &'static str = "This is a tooltip for the gear icon.";
#[rustfmt::skip] const TT_ARTICLE:  &'static str = "This is a tooltip for the ducuments icon.";

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

    let (_id, rect) = ui.allocate_space(egui::vec2(ui.available_width(), ui.available_height()));

    let center_x = rect.center().x;
    let center_y = rect.center().y;

    let texi_size = egui::vec2(100.0, 100.0);
    let gap = 30.0;

    let total_width = texi_size.x * NUM_TEXICONS as f32 + gap * (NUM_TEXICONS as f32 - 1.0);
    let mut x = center_x - total_width / 2.0;
    let y = center_y - texi_size.y / 2.0; // perfect vertical centering

    // Texicon 1
    let pos = egui::pos2(x, y);
    let texi_rect = egui::Rect::from_min_size(pos, texi_size);
    let mut texi_vec_index = 0;

    if let Some(texi) = texistates.get_mut(texi_vec_index) {
        let resp = ui.put(
            texi_rect,
            Texicon::new(texi)
                // .texi_img(TEXI_IMGS[texi_vec_index].clone())
                // .texi_text(None)
                // .texi_text_size(13.)
                .texi_frame_size(texi_size)
                .texi_top_gap(0.)
                .texi_img_size(egui::vec2(80., 80.))
                .texi_img_scale_hov(1.1)
                .texi_frame_size(egui::vec2(100., 150.))
                .texi_bkgnd_col(palette.red)
                .texi_bkgnd_col_sel(palette.maroon)
                .texi_bkgnd_col_hov(palette.yellow)
                .texi_img_tint(palette.base)
                .texi_img_tint_sel(palette.base)
                .texi_img_tint_hov(palette.base)
                .texi_text_col(palette.base)
                .texi_text_col_sel(palette.base)
                .texi_text_col_hov(palette.base)
                .texi_frame_col(palette.base)
                .texi_frame_col_sel(palette.base)
                .texi_frame_col_hov(palette.surface2)
                .texi_sense(egui_widget_texicon::TexiSense::ImageAndText)
                .texi_tooltip_text(Some(TOOLTIPS[texi_vec_index].to_string()))
                .texi_tooltip_gap(20.)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );

        if resp.clicked() {
            texistates.iter_mut().for_each(|t| t.texi_selected = false);
            texistates[texi_vec_index].texi_selected = true;
        }
    }
    x += texi_size.x + gap;

    // Texicon 2
    let pos = egui::pos2(x, y);
    let texi_rect = egui::Rect::from_min_size(pos, texi_size);
    texi_vec_index += 1;

    if let Some(texi) = texistates.get_mut(texi_vec_index) {
        let resp = ui.put(
            texi_rect,
            Texicon::new(texi)
                .texi_img(TEXI_IMGS[texi_vec_index].clone())
                .texi_text(None)
                .texi_frame_size(texi_size)
                .texi_top_gap(0.)
                .texi_bottom_gap(0.)
                .texi_img_size(egui::vec2(48., 48.))
                .texi_img_scale_hov(1.3)
                .texi_bkgnd_col(palette.red.gamma_multiply_u8(8))
                .texi_bkgnd_col_sel(palette.red.gamma_multiply_u8(24))
                .texi_bkgnd_col_hov(palette.red.gamma_multiply_u8(64))
                .texi_img_tint(palette.text)
                .texi_img_tint_sel(palette.text)
                .texi_img_tint_hov(palette.text)
                .texi_frame_col(palette.crust)
                .texi_frame_col_sel(palette.text)
                .texi_frame_col_hov(palette.text)
                .texi_sense(egui_widget_texicon::TexiSense::Frame)
                .texi_tooltip_text(Some(TOOLTIPS[texi_vec_index].to_string()))
                .texi_frame_width(2.)
                .texi_tooltip_gap(20.)
                .texi_radius(0)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );

        if resp.clicked() {
            texistates.iter_mut().for_each(|t| t.texi_selected = false);
            texistates[texi_vec_index].texi_selected = true;
        }
    }
    x += texi_size.x + gap;

    // Texicon 3
    let pos = egui::pos2(x, y);
    let texi_rect = egui::Rect::from_min_size(pos, texi_size);
    texi_vec_index += 1;

    if let Some(texi) = texistates.get_mut(texi_vec_index) {
        let resp = ui.put(
            texi_rect,
            Texicon::new(texi)
                .texi_img(TEXI_IMGS[texi_vec_index].clone())
                .texi_text(Some(TEXI_TEXT[texi_vec_index].to_string()))
                .texi_text_size(15.)
                .texi_frame_size(texi_size)
                .texi_top_gap(0.)
                .texi_bottom_gap(0.)
                .texi_frame_width(2.)
                .texi_img_text_gap(10.)
                .texi_img_size(egui::vec2(40., 40.))
                .texi_img_scale_hov(1.05)
                .texi_bkgnd_col(palette.base)
                .texi_bkgnd_col_sel(palette.mantle)
                .texi_bkgnd_col_hov(palette.crust)
                .texi_img_tint(palette.blue)
                .texi_img_tint_sel(palette.blue)
                .texi_img_tint_hov(palette.blue)
                .texi_text_col(palette.teal)
                .texi_text_col_sel(palette.teal)
                .texi_text_col_hov(palette.teal)
                .texi_frame_col(palette.crust)
                .texi_frame_col_sel(palette.crust)
                .texi_frame_col_hov(palette.surface2)
                .texi_tooltip_text(Some(TOOLTIPS[texi_vec_index].to_string()))
                .texi_tooltip_gap(20.)
                .texi_radius(20)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );

        if resp.clicked() {
            texistates.iter_mut().for_each(|t| t.texi_selected = false);
            texistates[texi_vec_index].texi_selected = true;
        }
    }
    x += texi_size.x + gap;

    // Texicon 4
    let pos = egui::pos2(x, y);
    let texi_rect = egui::Rect::from_min_size(pos, texi_size);
    texi_vec_index += 1;

    if let Some(texi) = texistates.get_mut(texi_vec_index) {
        let resp = ui.put(
            texi_rect,
            Texicon::new(texi)
                .texi_img(TEXI_IMGS[texi_vec_index].clone())
                .texi_text(Some(TEXI_TEXT[texi_vec_index].to_string()))
                .texi_text_size(17.)
                .texi_frame_size(texi_size)
                .texi_top_gap(0.)
                .texi_bottom_gap(0.)
                .texi_img_text_gap(16.)
                .texi_img_size(egui::vec2(50., 50.))
                .texi_img_scale_hov(1.1)
                .texi_frame_size(egui::vec2(120., 100.))
                .texi_bkgnd_col(palette.base)
                .texi_bkgnd_col_sel(palette.mantle)
                .texi_bkgnd_col_hov(palette.crust)
                .texi_img_tint(palette.green)
                .texi_img_tint_sel(palette.yellow)
                .texi_img_tint_hov(palette.yellow)
                .texi_text_col(palette.yellow)
                .texi_text_col_sel(palette.green)
                .texi_text_col_hov(palette.green)
                .texi_frame_col(palette.crust)
                .texi_frame_col_sel(palette.crust)
                .texi_frame_col_hov(palette.crust)
                .texi_frame_width(4.)
                .texi_tooltip_text(Some(TOOLTIPS[texi_vec_index].to_string()))
                .texi_tooltip_gap(20.)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );

        if resp.clicked() {
            texistates.iter_mut().for_each(|t| t.texi_selected = false);
            texistates[texi_vec_index].texi_selected = true;
        }
    }
    x += texi_size.x + gap;

    // Texicon 5
    let pos = egui::pos2(x, y);
    let texi_rect = egui::Rect::from_min_size(pos, texi_size);
    texi_vec_index += 1;

    if let Some(texi) = texistates.get_mut(texi_vec_index) {
        let resp = ui.put(
            texi_rect,
            Texicon::new(texi)
                .texi_enabled(false)
                .texi_img(TEXI_IMGS[texi_vec_index].clone())
                .texi_text(Some(TEXI_TEXT[texi_vec_index].to_string()))
                .texi_text_size(17.)
                .texi_frame_size(texi_size)
                .texi_top_gap(0.)
                .texi_bottom_gap(0.)
                .texi_frame_width(4.)
                .texi_img_text_gap(0.)
                .texi_img_size(egui::vec2(50., 50.))
                .texi_bkgnd_col(palette.base)
                .texi_bkgnd_col_sel(palette.mantle)
                .texi_bkgnd_col_hov(palette.crust)
                .texi_img_tint(palette.green)
                .texi_img_tint_sel(palette.yellow)
                .texi_img_tint_hov(palette.yellow)
                .texi_text_col(palette.yellow)
                .texi_text_col_sel(palette.green)
                .texi_text_col_hov(palette.green)
                .texi_frame_col(palette.crust)
                .texi_frame_col_sel(palette.crust)
                .texi_frame_col_hov(palette.crust)
                .texi_tooltip_text(Some(TOOLTIPS[texi_vec_index].to_string()))
                .texi_tooltip_gap(20.)
                .texi_tooltip_position(egui::RectAlign::BOTTOM),
        );

        if resp.clicked() {
            texistates.iter_mut().for_each(|t| t.texi_selected = false);
            texistates[texi_vec_index].texi_selected = true;
        }
    }
    // x += texi_size.x + gap;
}
