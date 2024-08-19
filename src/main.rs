#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod api;
mod utils;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/weather/:id")]
    Weather { id: usize, name_city: String },
}



fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app"); 

    let config = dioxus::desktop::Config::new().with_menu(None);

    LaunchBuilder::desktop().with_cfg(config).launch(app);

   
}

#[component]
fn app() -> Element {
    // let mut location_input_visible = use_signal(|| false);
    //let weathers = use_resource(move || async move { get_weather("Serra,BR").await });
    // let teset;
    // println!("{:?}", teset.name);

    rsx! {
    Router::<Route> {}

         // body {
         //      style {{include_str!("../assets/main.css")}}

         // match &*weathers.read() {
         //     Some(Ok(resp)) => rsx! {p {"{resp.name}"}},
         //     Some(Err(e)) => rsx! { p { "Loading weather failed, {e}" } },
         //     None => rsx! { p { "Loading..." } },
         // }

         //      div { class: "weather-card morning",
         //          div { class: "location-container",
         //            div { class:"location-icon", id:"locationIcon", "📍" }
         //            div { class: "location",id: "locationText",
         //              "teset.nam" }
         //            div { class:"search-icon", id:"searchIcon",onclick: move |_| location_input_visible.set(true), "🔍"}
         //          }

         //    form { class:if location_input_visible() {"location-input-show-display"} else {"location-input-display"} ,onsubmit: move |event| { println!("Submitted! {:?}", event.values()["city"]); location_input_visible.set(false) },
         //          input {r#type: "text",
         //            class: "location-input",
         //            id:"locationInput",
         //            name: "city",
         //            placeholder: "Digite sua localização",
         //            autofocus:true
         //          }
         //          label {"ex: Sao Paulo, BR"}
         //    }

         //        div {class:"temperature", "22°C"}
         //                      div{ class:"weather-icon", "☀️"}
         //                      div { class:"details",
         //                          div{ "4 MPH"}
         //                          div{ "20°C / 7°C"}
         //                          div{ "60%"}
         //                      }
         //      }
         //  }
        }
}

#[component]
fn Weather(id: usize, name_city: String) -> Element {
    let weathers = use_resource(move || async move { api::get_weather("Serra,BR").await });
    
    rsx! {
    style {{include_str!("../assets/tailwind.css")}}
        body { class:"bg-gray-100 p-8 flex justify-center items-center min-h-screen",

          match &*weathers.read() {
              Some(Ok(resp)) => rsx! {
                div {class:"bg-white rounded-3xl shadow-lg p-8 max-w-5xl w-full",
                div {class:"flex",
                        div {class:"flex flex-col w-1/3",
                            div {class:"relative mb-8",
                               input { r#type:"text", placeholder:"Search for places...", class:"w-full bg-gray-100 p-4 rounded-full pl-12 focus:outline-none" }
                               svg {
                                           class: "absolute top-1/2 left-4 transform -translate-y-1/2 w-6 h-6 text-gray-500",
                                           xmlns: "http://www.w3.org/2000/svg",
                                           view_box: "0 0 20 20",
                                           fill: "currentColor",
                                           path {
                                               fill_rule: "evenodd",
                                               d: "M12.9 14.32a8 8 0 111.414-1.414l4.387 4.386a1 1 0 01-1.415 1.415l-4.386-4.387zM8 14A6 6 0 108 2a6 6 0 000 12z",
                                               clip_rule: "evenodd",
                                           }
                                       }
          }


                            div {class:"bg-yellow-100 rounded-3xl p-6 flex flex-col items-center mb-8",
                                div { class:"text-6xl font-bold text-yellow-500 mb-4", "{resp.main.temp}°C"}
                                div { class:"text-gray-500", "{utils::get_day_and_hours_now()}"}
                                div { class:"flex items-center justify-center mt-8",
                                span {class:"text-lg font-medium text-gray-600", "{resp.weather[0].description}"}
                                }
                                div {class:"mt-4",
                                    p {class:"text-gray-600", "Rain - 30%"}
                                }
                            }

                            div { class:"flex items-center mt-auto",
                              img {src:"https://via.placeholder.com/80x80", alt:"Location", class:"w-12 h-12 rounded-full mr-4"}
                                div {
                                    p {class:"text-gray-600", "{resp.name}, {resp.sys.country}"}
                                }
                                }
                        }

                        div {
                          class:"w-2/3 pl-8 flex flex-col justify-center",
                            div {class:"grid grid-cols-3 gap-6",
                                div {class:"bg-gray-100 p-6 rounded-lg",
                                h3 {class:"text-gray-500 mb-2", "UV Index"}
                                div {class:"flex items-center",
                                span {class:"text-4xl font-bold text-yellow-500","5"}
                                div {class:"ml-4 text-gray-600", "Normal"}
                                    }
                        }

                                div {class:"bg-gray-100 p-6 rounded-lg",
                                    h3 {class:"text-gray-500 mb-2", "Wind Status"}
                                    div {class:"flex items-center",
                                    span {class:"text-4xl font-bold text-gray-700","{resp.wind.speed}"}
                                    span {class:"ml-2 text-lg text-gray-500", "km/h"}
                                    }
                                }

                                div {class:"bg-gray-100 p-6 rounded-lg",
                                    h3 {class:"text-gray-500 mb-2", "Sunrise & Sunset"}
                                    div { class:"text-gray-700",
                                        div{ "{utils::converte_unix_time_in_hours(resp.sys.sunrise)} AM"}
                                        div{ "{utils::converte_unix_time_in_hours(resp.sys.sunset)} PM"}
                                        }
                                        }

                                div {class:"bg-gray-100 p-6 rounded-lg",
                                    h3 {class:"text-gray-500 mb-2", "Humidity"}
                                    div {class:"flex items-center",
                                      span { class:"text-4xl font-bold text-gray-700", "{resp.main.humidity}%"}
                                    div {class:"ml-4 text-gray-600", "Normal"}
      }
      }

                                div {class:"bg-gray-100 p-6 rounded-lg",
                                    h3 {class:"text-gray-500 mb-2", "Visibility"}
                                    div {class:"flex items-center",
                                    span{ class:"text-4xl font-bold text-gray-700","5.2"}
                                    span {class:"ml-2 text-lg text-gray-500","km"}
                                }
                                }

                                div { class:"bg-gray-100 p-6 rounded-lg",
                                    h3 {class:"text-gray-500 mb-2", "Air Quality"}
                                    div { class:"flex items-center",
                                        span{ class:"text-4xl font-bold text-red-500", "105"}
                                        div { class:"ml-4 text-gray-600", "Unhealthy"}
                                    }
          }
          }
          }
          }


                }
              },
              Some(Err(e)) => rsx! { p { "Loading weather failed, {e}" } },
              None => rsx! { p { "Loading..." } },
          }



        }
    }
}

#[component]
fn Home() -> Element {
    // let mut name = use_signal(|| String::from(""));
    //const SVG_FILE: &str = manganis::mg!(file("assets/file.svg"));
    rsx! {
        style {{include_str!("../assets/tailwind.css")}}
        body { class:"bg-gray-100 flex items-center justify-center min-h-screen",
            div { class:"bg-white p-8 rounded-lg shadow-lg max-w-lg w-full",
            h1 { class:"text-4xl font-bold text-gray-800 text-center mb-6", "Tempest Nature"}
            
            div { class:"flex items-center space-x-3",
            input {
                    r#type:"text", 
                    placeholder:"DIGITE SUA CIDADE, EX: São Paulo, BR" ,
                    class:"w-full p-3 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500"}
               
        Link {
            class:"text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800",

              to: Route::Weather {
                  id: 22,
                  name_city: "name".to_string()
              },
              "➔"
          }
    }
    }


            //body {
            //   class: "bg-gray-100 flex justify-center items-center min-h-screen",
            //   style {{include_str!("../assets/tailwind.css")}}
            // div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4",
            //   div { class:"bg-orange-500 text-white p-6 rounded-lg shadow-lg",
            //               div { class:"flex flex-col items-center",
            //               h2 {class:"text-lg font-semibold", "New York"}
            //               div { class:"flex items-center justify-center mt-4",
            //               span{ class:"text-5xl font-bold", "22°"}

            //   }
            //                   div { class:"mt-4 text-sm",
            //                       p {"MPH"}
            //                       p{"20°/17°"}
            //                       p{"40%"}
            //   }
            //   }
            //          }

              //    form { class:"location-input-show-display",
              //          input {r#type: "text",
              //            class: "location-input",
              //            id:"locationInput",
              //            name: "city",
              //            placeholder: "Digite sua localização",
              //            autofocus:true
              //          }
              //          label {"ex: Sao Paulo, BR"}


                      

                 }

              // p {"Home"}
              // input {class:"bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white",r#type: "text", id: "name", name: "{name}", oninput: move |event| name.set(event.value()) }

              //}
              //}

    }
}


