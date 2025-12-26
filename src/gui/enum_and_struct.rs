use crate::img_output::img_render::ImgCache;

pub struct App {
    pub vec_of_widgets: Vec<(iced::widget::image::Handle, String)>,
    pub childs: Vec<String>,
    pub selected_child: (String, usize),
    pub img_cache: ImgCache,
}

#[derive(Debug, Clone)]
pub enum Message {
    ParentButtonClicked((String, u8)),
    ChildRadioClicked((String, usize)),
    ImgClicked(String),
}
