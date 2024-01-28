use crate::model::Resume;
use leptos::{component, view, IntoView};

#[component]
pub fn Basic(config: Resume) -> impl IntoView {
    let name = config.candidate.name.to_string();
    view! {
        <h1>{name}</h1>
    }
}
