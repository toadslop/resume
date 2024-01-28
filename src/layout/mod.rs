use serde::Deserialize;
use strum::Display;

mod basic;

pub use basic::Basic;

#[derive(Debug, Deserialize, Display)]
pub enum Layout {
    Basic,
}
