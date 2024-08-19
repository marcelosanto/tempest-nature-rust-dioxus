#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod components;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let config = dioxus::desktop::Config::new().with_menu(None);

    LaunchBuilder::desktop().with_cfg(config).launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        Router::<components::Route> {}
    }
}
