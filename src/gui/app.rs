use super::messages::Message;
use super::state::App;
use crate::image::cache::ImgCache;
use crate::image::loader;
use crate::image::processor;
use iced::{Sandbox, Settings, Size};

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let childs = loader::get_childrens().unwrap();
        let selected_child = childs.first().unwrap().clone();
        let img_cache = ImgCache::new(0, 1, 2, 4, 3);

        App {
            vec_of_widgets: Vec::new(),
            childs: childs.clone(),
            selected_child: (selected_child, 0),
            img_cache,
            vec_of_img: vec![None; loader::get_parents().unwrap().len()],
            selected_parent_id: 0,
            result: image::ImageBuffer::new(0, 0),
        }
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn title(&self) -> String {
        String::from("Liberty Sprite Creator V0.1.0")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ParentButtonClicked((name, id)) => {
                self.vec_of_widgets = self
                    .img_cache
                    .get_cells(name.as_str(), self.selected_child.0.as_str());
                self.selected_parent_id = id;
            }

            Message::ChildRadioClicked(data) => {
                self.selected_child = data;
            }

            Message::ImgClicked(path) => {
                if self.vec_of_img[self.selected_parent_id as usize] == Some(path.clone()) {
                    self.vec_of_img[self.selected_parent_id as usize] = None;
                } else {
                    self.vec_of_img[self.selected_parent_id as usize] = Some(path);
                }

                self.result = processor::render_combined_image(
                    self.vec_of_img
                        .clone()
                        .into_iter()
                        .filter_map(|x| x)
                        .collect::<Vec<_>>(),
                )
                .unwrap_or(image::ImageBuffer::new(0, 0));
            }

            Message::ResultButtonClicked => {
                processor::save_result(&self.result, "./src/Result/output.png");
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        App::view(self)
    }
}

pub fn run_gui() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            min_size: Some(Size::new(1280.0, 720.0)),
            position: iced::window::Position::Centered,
            ..Default::default()
        },
        antialiasing: false,
        ..Default::default()
    })
}
