use super::img_path;
use iced::widget::image::Handle;
use image as img;
use image::imageops::FilterType;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ImgCache {
    pub cached_cells: Arc<HashMap<(String, String), Vec<(Handle, String)>>>,
}

impl ImgCache {
    pub fn new(row: u32, col: u32, scale: u32, rows: u32, cols: u32) -> Self {
        let mut cache: HashMap<(String, String), Vec<(Handle, String)>> = HashMap::new();

        if let Ok(parents) = img_path::get_path() {
            for parent in parents {
                for child in parent.children {
                    let mut cells: Vec<(Handle, String)> = Vec::new();

                    for path in &child.files {
                        let widget_name = {
                            "./src/img/".to_string()
                                + parent.name.as_str()
                                + "/"
                                + child.name.as_str()
                                + "/"
                                + path.file_name().unwrap_or_default().to_str().unwrap()
                        };

                        if let Ok(image) = img::open(path) {
                            let rgba = image.to_rgba8();
                            let (width, height) = rgba.dimensions();
                            let cell_width = width / cols;
                            let cell_height = height / rows;

                            if row < rows && col < cols {
                                let x = col * cell_width;
                                let y = row * cell_height;

                                let cell =
                                    img::imageops::crop_imm(&rgba, x, y, cell_width, cell_height)
                                        .to_image();

                                let scaled = img::imageops::resize(
                                    &cell,
                                    cell_width * scale,
                                    cell_height * scale,
                                    img::imageops::FilterType::Nearest,
                                );

                                let handle = Handle::from_pixels(
                                    scaled.width(),
                                    scaled.height(),
                                    scaled.into_raw(),
                                );

                                cells.push((handle, widget_name));
                            }
                        }
                    }

                    cache.insert((parent.name.clone(), child.name.clone()), cells);
                }
            }
        }

        ImgCache {
            cached_cells: Arc::new(cache),
        }
    }

    pub fn get_cells(&self, parent: &str, child: &str) -> Vec<(Handle, String)> {
        self.cached_cells
            .get(&(parent.to_string(), child.to_string()))
            .cloned()
            .unwrap_or_default()
    }
}

pub fn render_result_in_gui(
    images: Vec<String>,
) -> Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
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
