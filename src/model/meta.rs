use crate::layout::Layout;
use serde::Deserialize;
use strum::Display;

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub layout: Layout,
    pub theme: ThemeName,
}

#[derive(Debug, Deserialize, Display)]
pub enum ThemeName {
    Basic,
}
