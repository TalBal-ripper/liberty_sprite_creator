use crate::img_output::img_render::ImgCache;

pub struct App {
    pub vec_of_widgets: Vec<(iced::widget::image::Handle, String)>,
    pub childs: Vec<String>,
    pub selected_child: (String, usize),
    pub selected_parent_id: u8,
    pub img_cache: ImgCache,
    pub vec_of_img: Vec<Option<String>>,
    pub result: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ParentButtonClicked((String, u8)),
    ChildRadioClicked((String, usize)),
    ImgClicked(String),
    ResultButtonClicked(),
}
