use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: u8,
    name: String,
    monitor: String,
    windows: u32,

    #[serde(rename = "hasfullscreen")]
    has_fullscreen: bool,

    #[serde(rename = "lastwindow")]
    last_window: String,

    #[serde(rename = "lastwindowtitle")]
    last_window_title: String,
}
