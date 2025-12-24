pub struct App {
    pub cell_handle: Option<iced::widget::image::Handle>,
    pub buttons_scroll: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    ParentButtonClicked((String, u8)),
    ChildButtonClicked(String),
}
