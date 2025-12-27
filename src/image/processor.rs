use image::imageops::FilterType;
use image::{ImageBuffer, Rgba};

pub fn render_combined_image(images: Vec<String>) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut iter = images.iter();
    let first_path = iter.next()?;

    let mut result = image::open(first_path).ok()?.to_rgba8();

    for path in iter {
        if let Ok(img) = image::open(path) {
            image::imageops::overlay(&mut result, &img.to_rgba8(), 0, 0);
        }
    }

    let (width, height) = result.dimensions();
    let scaled = image::imageops::resize(&result, width * 2, height * 2, FilterType::Nearest);

    Some(scaled)
}

pub fn save_result(result: &ImageBuffer<Rgba<u8>, Vec<u8>>, path: &str) {
    let (width, height) = result.dimensions();
    let new_width = width / 2;
    let new_height = height / 2;

    let scaled_result = image::imageops::resize(
        result,
        new_width,
        new_height,
        image::imageops::FilterType::Nearest,
    );

    if let Err(e) = scaled_result.save(path) {
        eprintln!("Failed to save image: {}", e);
    }
}
