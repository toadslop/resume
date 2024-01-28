use itertools::Itertools;
use serde::Deserialize;
use std::fmt::Display;
use std::fmt::{Formatter, Result};

#[derive(Debug, Deserialize)]
pub struct Candidate {
    pub name: Name,
}

#[derive(Debug, Deserialize)]
pub struct Name {
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
    pub title: Option<String>,
    pub suffix: Option<String>,
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let name = [
            self.title.as_ref(),
            Some(&self.first),
            self.middle.as_ref(),
            Some(&self.last),
            self.suffix.as_ref(),
        ]
        .iter()
        .flatten()
        .join(" ");

        write!(f, "{name}")
    }
}
