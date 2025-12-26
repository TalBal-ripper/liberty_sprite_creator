use iced::widget::scrollable::{Direction, Properties};

use crate::gui::enum_and_struct::{App, Message};
use crate::img_output::img_path::{get_childrens, get_parents};
use crate::img_output::img_render::ImgCache;
use iced::widget::{Column, Row, Scrollable, Text, button, container};
use iced::{Element, Length, Sandbox, Settings, Size};

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let childs = get_childrens().unwrap();
        let selected_child = childs.first().unwrap().clone();
        let img_cache = ImgCache::new(0, 1, 2, 4, 3);

        App {
            vec_of_widgets: Vec::new(),
            childs: childs.clone(),
            selected_child: (selected_child, 0),
            img_cache,
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
            Message::ParentButtonClicked((name, _id)) => {
                self.vec_of_widgets = self
                    .img_cache
                    .get_cells(name.as_str(), self.selected_child.0.as_str());
            }
            Message::ChildRadioClicked(data) => {
                self.selected_child = data;
            }
            Message::ImgClicked(path) => {
                println!("path: {path}");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut top_bar_buttons: Row<Message> = Row::new().spacing(10).padding(10);
        for (i, each_parent) in get_parents().unwrap().into_iter().enumerate() {
            top_bar_buttons = top_bar_buttons.push(
                button(Text::new(each_parent.clone()))
                    .on_press(Message::ParentButtonClicked((each_parent, i as u8))),
            );
        }

        let mut childs_buttons: Row<Message> = Row::new().spacing(10).padding(10);
        for (i, child_label) in self.childs.iter().enumerate() {
            childs_buttons = childs_buttons.push(iced::widget::radio(
                child_label,
                i,
                Some(self.selected_child.1),
                |selected_index| {
                    Message::ChildRadioClicked((child_label.to_string(), selected_index))
                },
            ));
        }

        Column::new()
            .push(
                Row::new().push(childs_buttons).push(
                    Scrollable::new(top_bar_buttons)
                        .direction(Direction::Horizontal(Properties::new())),
                ),
            )
            .push(
                Row::new()
                    .push(container(
                        Scrollable::new({
                            let mut grid: Column<Message> = Column::new().spacing(10).padding(20);

                            for chunk in self.vec_of_widgets.chunks(10) {
                                let mut image_row = Row::new().spacing(10);

                                for (handle, name) in chunk {
                                    image_row = image_row.push(
                                        button(
                                            iced::widget::image(handle.clone())
                                                .content_fit(iced::ContentFit::None)
                                                .filter_method(
                                                    iced::widget::image::FilterMethod::Nearest,
                                                ),
                                        )
                                        .on_press(Message::ImgClicked(name.clone())),
                                    );
                                }

                                grid = grid.push(image_row);
                            }

                            grid
                        })
                        .width(Length::Fill)
                        .height(Length::Fill),
                    ))
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .into()
    }
}

pub fn run_gui() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            min_size: Some(Size::new(1280.0 as f32, 720.0 as f32)),
            position: iced::window::Position::Centered,
            ..Default::default()
        },
        antialiasing: false,
        ..Default::default()
    })
}
