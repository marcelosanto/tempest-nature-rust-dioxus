#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    let mut location_input_visible = use_signal(|| false);

    rsx! {
     body {
          style {{include_str!("../assets/main.css")}}

          div { class: "weather-card morning",
              div { class: "location-container",
                div { class:"location-icon", id:"locationIcon", "📍" }
                div { class: "location",id: "locationText",
                    "New York"}
                div { class:"search-icon", id:"searchIcon",onclick: move |_| location_input_visible.set(true), "🔍"}
              }

              input {r#type: "text",
               class:if location_input_visible() {"location-input"} else {"location-input-display"} ,
                id:"locationInput",
                placeholder: "Digite sua localização"
              }

            div {class:"temperature", "22°C"}
                          div{ class:"weather-icon", "☀️"}
                          div { class:"details",
                              div{ "4 MPH"}
                              div{ "20°C / 7°C"}
                              div{ "60%"}
                          }
          }
      }
    }
}
