#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use reqwest;
use serde::{Deserialize, Serialize};
use chrono::{Local, TimeZone, Utc};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Weather { id: usize, name_city: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    lat: f64,
    lon: f64,
}

#[derive(Deserialize, Debug, Default)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
    sys: Sys,
}

#[derive(Deserialize, Debug, Default)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug, Default)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug, Default)]
struct Wind {
    speed: f64,
}

#[derive(Deserialize, Debug, Default)]
struct Sys {
    country: String,
    sunrise: i64,
    sunset: i64
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    launch(app)
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
    let weathers = use_resource(move || async move { get_weather("Serra,BR").await });

    rsx! {
    style {{include_str!("../assets/tailwind.css")}}

        // Link { to: Route::Home {}, "Go to counter" }
        // "Blog post { id} - and {name_city}"
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
                                div { class:"text-gray-500", "Monday, 16:00"}
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
                                        div{ "{converte_unix_time_in_hours(resp.sys.sunrise)} AM"}
                                        div{ "{converte_unix_time_in_hours(resp.sys.sunset)} PM"}
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
        body {


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


                       Link {
                         class:"text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800",

                           to: Route::Weather {
                               id: 22,
                               name_city: "name".to_string()
                           },
                           "Go to blog"
                       }

                 }

              // p {"Home"}
              // input {class:"bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white",r#type: "text", id: "name", name: "{name}", oninput: move |event| name.set(event.value()) }

              //}
              //}

    }
}

async fn get_weather(city_name: &str) -> reqwest::Result<WeatherResponse> {
    // let response = reqwest::get("https://api.ipify.org").await?;
    // let ip = response.text().await?;

    // let mut location_lat_long = Location { lat: 0.0, lon: 0.0 };

    // if city_name != "none" {
    //     location_lat_long = get_lat_lon(ip.clone()).await?;

    //     println!("{:?}", location_lat_long);
    // }

    // println!("{:?}", location_lat_long);

    let city: Vec<&str> = city_name.split(",").collect();
    let resp_weather = reqwest::get(
        format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid=11a70b5b6c6d329b4725068885de6f6d", city[0], city[1]),
    )
    .await?
    .json::<WeatherResponse>()
    .await?;

    // println!("{:?}", city);

    // if city.len() == 3 {
    //     resp_weather = reqwest::get(format!("https://api.openweathermap.org/data/2.5/weather?q={},{},{}&units=metric&appid=11a70b5b6c6d329b4725068885de6f6d", city[0], city[1], city[2])).await?.json::<WeatherResponse>().await?;
    //     println!("w -> {:?}", resp_weather);
    // }

    // if city.len() == 2 {
    //     resp_weather = reqwest::get(
    //         format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid=11a70b5b6c6d329b4725068885de6f6d", city[0], city[1]),
    //     )
    //     .await?
    //     .json::<WeatherResponse>()
    //     .await?;

    //     println!("w -> {:?}", resp_weather);
    // }
    //let url_weather = format!("https://api.openweathermap.org/data/2.5/weather?q={city_name},{state_code},{country_code}&appid={api_key}");

    Ok(resp_weather)
}

async fn get_lat_lon(ip: String) -> reqwest::Result<Location> {
    let url = format!("{}{}", "http://ip-api.com/json/", ip);

    let response = reqwest::get(&url).await?.json::<Location>().await?;
    let local: Location = Location {
        lat: response.lat,
        lon: response.lon,
    };

    Ok(local)
}

fn converte_unix_time_in_hours(time: i64) -> String {
        // O timestamp Unix em segundos (UTC)
        let timestamp_utc = time;

        // Converte o timestamp para um objeto DateTime no UTC
        let datetime_utc = Utc.timestamp_opt(timestamp_utc, 0).unwrap();
    
        // Converte para a hora no fuso horário UTC-3
        let datetime_utc_minus = datetime_utc.with_timezone(&Local);
    
        let hora_utc_minus = datetime_utc_minus.format("%H:%M:%S").to_string();

        return hora_utc_minus;    
}