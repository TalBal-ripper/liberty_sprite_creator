#[derive(Debug, Clone)]
pub enum Message {
    ParentButtonClicked((String, u8)),
    ChildRadioClicked((String, usize)),
    ImgClicked(String),
    ResultButtonClicked,
}
