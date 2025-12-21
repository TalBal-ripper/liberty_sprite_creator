/*use iced::widget::{container, row, scrollable};
use iced::{Element, Length, Sandbox, Settings, Size};

struct ImageGallery {
    images: Vec<iced::widget::image::Handle>,
}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for ImageGallery {
    type Message = Message;

    fn new() -> Self {
        let image_paths = vec!["image1.png", "image2.png", "image3.png", "image4.png"];

        let images = image_paths
            .iter()
            .filter_map(|path| {
                image::open(path).ok().map(|loaded| {
                    let rgba = loaded.to_rgba8();
                    let (w, h) = rgba.dimensions();
                    let scale = 3;
                    let scaled = image::imageops::resize(
                        &rgba,
                        w * scale,
                        h * scale,
                        image::imageops::FilterType::Nearest,
                    );
                    let (sw, sh) = scaled.dimensions();
                    iced::widget::image::Handle::from_pixels(sw, sh, scaled.into_raw())
                })
            })
            .collect();

        ImageGallery { images }
    }

    fn title(&self) -> String {
        String::from("Image Gallery")
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        let images: Vec<_> = self
            .images
            .iter()
            .map(|handle| {
                iced::widget::image(handle.clone())
                    .content_fit(iced::ContentFit::None)
                    .filter_method(iced::widget::image::FilterMethod::Nearest)
                    .into()
            })
            .collect();

        let image_row = row(images).spacing(10).padding(20);

        container(scrollable(image_row))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    ImageGallery::run(Settings {
        window: iced::window::Settings {
            //size: Size::new(1280.0, 720.0), // Размер окна
            min_size: Some(Size::new(1280.0, 720.0)),
            //resizable: false,               // Запретить изменение размера
            ..Default::default()
        },
        antialiasing: false,
        ..Default::default()
    })
}*/

use super::messages;
use iced::widget::{container, image as iced_image};
use iced::{Element, Length, Sandbox, Settings};
use image as img;

impl Sandbox for messages::ImageViewer {
    type Message = messages::Message;

    fn new() -> Self {
        let image_handle = if let Ok(img) = img::open("image.png") {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            let pixels = rgba.into_raw();

            Some(iced::widget::image::Handle::from_pixels(
                width, height, pixels,
            ))
        } else {
            None
        };

        messages::ImageViewer { image_handle }
    }

    fn title(&self) -> String {
        String::from("Sharp Image")
    }

    fn update(&mut self, _message: messages::Message) {}

    fn view(&self) -> Element<messages::Message> {
        let content = if let Some(handle) = &self.image_handle {
            iced_image(handle.clone())
                .content_fit(iced::ContentFit::None)
                .filter_method(iced::widget::image::FilterMethod::Nearest)
        } else {
            iced_image(iced::widget::image::Handle::from_path("placeholder.png"))
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn app_run() -> iced::Result {
    messages::ImageViewer::run(Settings::default())
}
