use std::u8;

use crate::gui::enum_and_struct::{App, Message};
use crate::img_output;
use crate::img_output::img_path::{get_childrens, get_parents};
use crate::img_output::img_render::{extract_cell, get_rendered_images};
use iced::widget::{
    Column, Row, Scrollable, Text, button, column, container, row,
    scrollable::{self, Direction, Properties},
    text,
};
use iced::{Element, Length, Sandbox, Settings, Size};
use image::imageops::FilterType::Nearest;

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        // Извлечь ячейку: ряд 1, столбец 2 (индексы начинаются с 0)
        let cell_handle = extract_cell(
            "image1.png",
            4, // rows
            3, // cols
            0, // row (первый ряд)
            1, // col (второй столбец)
            2, // scale
        );

        App {
            cell_handle: cell_handle,
            buttons_scroll: 50,
        }
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn title(&self) -> String {
        String::from("Extract Cell Function")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ParentButtonClicked((name, id)) => {
                println!("Button {name} with id {id} clicked!");
            }
            Message::ChildButtonClicked(id) => {
                println!("Button {id} clicked!");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        /*let content = if let Some(handle) = &self.cell_handle {
            column![
                iced::widget::image(handle.clone())
                    .content_fit(iced::ContentFit::None)
                    .filter_method(iced::widget::image::FilterMethod::Nearest)
            ]
            .spacing(20)
            .padding(20)
        } else {
            column![text("Ячейка не найдена")]
        };

        let mut col: Row<Message> = row(vec![]).padding(10);

        for i in 0..self.buttons_scroll {
            col = col.push(button(Text::new(format!("Button #{i}"))));
        }
        column![
            Scrollable::new(col).direction(Direction::Horizontal(Properties::new())),
            content
        ]
        .into()*/

        //let base_image;
        let mut top_bar_battons: Row<Message> = row(vec![]).spacing(10).padding(10);
        for (i, each_parent) in get_parents().unwrap().into_iter().enumerate() {
            top_bar_battons = top_bar_battons.push(
                button(Text::new(each_parent.clone()))
                    .on_press(Message::ParentButtonClicked((each_parent, i as u8))),
            );
        }

        let mut childs_buttons: Row<Message> = row(vec![]).spacing(10).padding(10);
        for (i, each_child) in get_childrens().unwrap().into_iter().enumerate() {
            childs_buttons = childs_buttons.push(
                button(Text::new(each_child.clone()))
                    .on_press(Message::ChildButtonClicked(each_child)),
            );
        }
        column![
            row![
                childs_buttons,
                Scrollable::new(top_bar_battons)
                    .direction(Direction::Horizontal(Properties::new()))
            ],
            row![]
        ]
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
