use crate::gui::enum_and_struct::{App, Message};
use crate::img_output::img_path::{get_childrens, get_parents};
use crate::img_output::img_render::{ImgCache, render_result_in_gui};
use iced::widget::scrollable::{Direction, Properties};
use iced::widget::{self, Image, button, column, container, row, scrollable, text};
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
            vec_of_img: vec![None; get_parents().unwrap().len()],
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

                self.result = render_result_in_gui(
                    self.vec_of_img
                        .clone()
                        .into_iter()
                        .filter_map(|x| x)
                        .collect::<Vec<_>>(),
                )
                .unwrap_or(image::ImageBuffer::new(0, 0));
            }

            Message::ResultButtonClicked() => {
                let result = {
                    let (width, height) = self.result.dimensions();
                    let new_width = width / 2;
                    let new_height = height / 2;

                    let scaled_result = image::imageops::resize(
                        &self.result,
                        new_width,
                        new_height,
                        image::imageops::FilterType::Nearest,
                    );

                    scaled_result
                };
                result.save("./src/Result/output.png").unwrap()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut top_bar_buttons = row![].spacing(10).padding(10);
        for (i, each_parent) in get_parents().unwrap().into_iter().enumerate() {
            top_bar_buttons = top_bar_buttons.push(
                button(text(each_parent.clone()))
                    .on_press(Message::ParentButtonClicked((each_parent, i as u8))),
            );
        }

        let mut childs_buttons = row![].spacing(10).padding(10);
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

        let mut grid = column![].spacing(10).padding(20);

        for chunk in self.vec_of_widgets.chunks(10) {
            let mut image_row = row![].spacing(10);

            for (handle, name) in chunk {
                image_row = image_row.push(
                    button(
                        iced::widget::image(handle.clone())
                            .content_fit(iced::ContentFit::None)
                            .filter_method(iced::widget::image::FilterMethod::Nearest),
                    )
                    .on_press(Message::ImgClicked(name.clone())),
                );
            }

            grid = grid.push(image_row);
        }

        column![
            row![
                childs_buttons,
                scrollable(top_bar_buttons).direction(Direction::Horizontal(Properties::new()))
            ],
            row![
                container(
                    iced::widget::image(iced::widget::image::Handle::from_pixels(
                        self.result.width(),
                        self.result.height(),
                        self.result.clone().into_raw(),
                    ))
                    .content_fit(iced::ContentFit::Contain)
                    .filter_method(iced::widget::image::FilterMethod::Nearest)
                ),
                container(scrollable(grid).width(Length::Fill).height(Length::Fill))
                    .width(Length::Fill)
                    .height(Length::Fill)
            ],
            row![button(text("BLEND IT!!!!!!!!")).on_press(Message::ResultButtonClicked())]
                .spacing(10)
                .padding(10)
        ]
        .into()
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
