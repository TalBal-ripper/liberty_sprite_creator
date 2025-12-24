use super::img_path;
use image as img;

pub fn get_rendered_images(
    parent: &str,
    child: &str,
) -> Result<Vec<Option<iced::advanced::image::Handle>>, Box<dyn std::error::Error>> {
    let data = img_path::get_path()?;
    let parent_dir = data
        .into_iter()
        .find(|p| p.name == parent)
        .ok_or(format!("Parent not found: {}", parent))?;

    let child_dir = parent_dir
        .children
        .into_iter()
        .find(|c| c.name == child)
        .ok_or(format!("Child not found in {}: {}", parent, child))?;

    let mut images = Vec::new();

    for each_val_of_path_of_img in child_dir.files {
        let img = img::open(&each_val_of_path_of_img)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let pixels = rgba.into_raw();
        images.push(Some(iced::widget::image::Handle::from_pixels(
            width, height, pixels,
        )));
    }

    Ok(images)
}

pub fn extract_cell(
    image_path: &str,
    rows: u32,
    cols: u32,
    row: u32,
    col: u32,
    scale: u32,
) -> Option<iced::widget::image::Handle> {
    if let Ok(loaded) = img::open(image_path) {
        let rgba = loaded.to_rgba8();
        let (width, height) = rgba.dimensions();

        let cell_width = width / cols;
        let cell_height = height / rows;

        // Проверка границ
        if row >= rows || col >= cols {
            return None;
        }

        let x = col * cell_width;
        let y = row * cell_height;

        let cell = img::imageops::crop_imm(&rgba, x, y, cell_width, cell_height).to_image();

        let scaled = img::imageops::resize(
            &cell,
            cell_width * scale,
            cell_height * scale,
            img::imageops::FilterType::Nearest,
        );

        let (sw, sh) = scaled.dimensions();
        Some(iced::widget::image::Handle::from_pixels(
            sw,
            sh,
            scaled.into_raw(),
        ))
    } else {
        None
    }
}
