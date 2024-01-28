use self::{candidate::Candidate, meta::Meta};
use serde::Deserialize;

mod candidate;
mod meta;

#[derive(Debug, Deserialize)]
pub struct Resume {
    pub meta: Meta,
    pub candidate: Candidate,
}
