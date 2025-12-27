use super::messages::Message;
use super::state::App;
use crate::image::loader;
use iced::widget::scrollable::{Direction, Properties};
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            self.build_top_bar(),
            self.build_main_content(),
            self.build_bottom_bar()
        ]
        .into()
    }

    fn build_top_bar(&self) -> Element<'_, Message> {
        let mut top_bar_buttons = row![].spacing(10).padding(10);

        for (i, each_parent) in loader::get_parents().unwrap().into_iter().enumerate() {
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

        row![
            childs_buttons,
            scrollable(top_bar_buttons).direction(Direction::Horizontal(Properties::new()))
        ]
        .into()
    }

    fn build_main_content(&self) -> Element<'_, Message> {
        let result_preview = container(
            iced::widget::image(iced::widget::image::Handle::from_pixels(
                self.result.width(),
                self.result.height(),
                self.result.clone().into_raw(),
            ))
            .content_fit(iced::ContentFit::Contain)
            .filter_method(iced::widget::image::FilterMethod::Nearest),
        );

        let grid = self.build_image_grid();

        row![
            result_preview,
            container(scrollable(grid).width(Length::Fill).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .into()
    }

    fn build_image_grid(&self) -> Element<'_, Message> {
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

        grid.into()
    }

    fn build_bottom_bar(&self) -> Element<'_, Message> {
        row![button(text("Blend")).on_press(Message::ResultButtonClicked)]
            .spacing(10)
            .padding(10)
            .into()
    }
}
