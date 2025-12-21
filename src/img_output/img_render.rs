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
