use leptos::{mount_to_body, view};
use model::Resume;

mod layout;
mod model;

fn main() {
    console_error_panic_hook::set_once();
    let config = include_str!("../config.yaml");
    let config: Resume = serde_yaml::from_str(config).expect("Failed to parse resume");

    #[allow(non_snake_case)]
    let Layout = match config.meta.layout {
        layout::Layout::Basic => layout::Basic,
    };

    let css = format!("themes/{}.css", config.meta.theme);

    mount_to_body(move || {
        view! {<div>
        <link rel="stylesheet" type="text/css" href={css} />
        <Layout config={config}></Layout></div>}
    });
}
