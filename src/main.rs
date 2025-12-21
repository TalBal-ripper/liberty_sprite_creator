use iced::widget::{container, row, scrollable};
use iced::{Element, Length, Sandbox, Settings, Size};
use image as img;

struct ImageGallery {
    images: Vec<iced::widget::image::Handle>,
}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for ImageGallery {
    type Message = Message;

    fn new() -> Self {
        let mut images = Vec::new();

        for i in 1..=20 {
            let path = format!("image{}.png", i);
            if let Ok(loaded) = img::open(&path) {
                let rgba = loaded.to_rgba8();
                let (w, h) = rgba.dimensions();
                let scale = 3;
                let scaled = img::imageops::resize(
                    &rgba,
                    w * scale,
                    h * scale,
                    img::imageops::FilterType::Nearest,
                );
                let (sw, sh) = scaled.dimensions();
                images.push(iced::widget::image::Handle::from_pixels(
                    sw,
                    sh,
                    scaled.into_raw(),
                ));
            }
        }

        ImageGallery { images }
    }

    fn title(&self) -> String {
        String::from("Horizontal Scroll")
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        // Используем row для горизонтального расположения
        let mut content = row![].spacing(10).padding(20);

        for handle in &self.images {
            content = content.push(
                iced::widget::image(handle.clone())
                    .content_fit(iced::ContentFit::None)
                    .filter_method(iced::widget::image::FilterMethod::Nearest),
            );
        }

        // Горизонтальная прокрутка
        container(
            scrollable(content)
                .direction(scrollable::Direction::Horizontal(
                    scrollable::Properties::default(),
                ))
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

fn main() -> iced::Result {
    ImageGallery::run(Settings {
        window: iced::window::Settings {
            size: Size::new(1280.0, 720.0),
            resizable: false,
            position: iced::window::Position::Centered,
            ..Default::default()
        },
        antialiasing: false,
        ..Default::default()
    })
}
