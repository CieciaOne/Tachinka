use std::fmt::format;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_logger;
use web_sys::window;
use yew::prelude::*;
use yew::{html, Component, Context, Html, Properties};
fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default());
}
// This here is start of for now unused section
pub enum Msg {
    Initialize,
    Update,
    Open,
}
#[derive(PartialEq, Properties)]
pub struct Props;
pub struct MangaList {
    elements: Vec<String>,
}

impl Component for MangaList {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            elements: Vec::<String>::new(),
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        self.elements
            .iter()
            .map(|i| {
                html! {
                    <div id={i.clone()} class="item" style="color:white;">{format!("Object: {}",i)}</div>
                }
            })
            .collect::<Html>()
    }
    // fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    //     self.elements.append(other)
    // }
}
// That is the end of it

#[derive(Deserialize, Serialize, Debug)]
struct Manga {
    id: String,
    // title: String,
    // description: Value,
    attributes: Value,
}
#[function_component(DynamicGrid)]
pub fn dynamic_grid() -> Html {
    let content_string = use_state_eq(|| "".to_string());
    {
        let content_string = content_string.clone();
        use_effect(|| {
            display_dynamic_content_grid(content_string);
            || ()
        });
    }

    let content: Result<Value, serde_json::Error> =
        serde_json::from_str(&(*content_string).clone());
    match content {
        Ok(resp) => {
            log(&resp.to_string());
            let data: Vec<Manga> =
                serde_json::from_value(resp.get("data").unwrap().clone()).unwrap();
            html! {
                data.iter().map(|i| {
                    html!{
                        <div class="item" style="color:white;">
                            <h3>{format!("{}",i.attributes["title"]["en"].to_string())}</h3>

                        </div>
                    }
                }).collect::<Html>()
            }
        }
        Err(e) => {
            html! {
                <h1>{format!("\nError: {}",e)}</h1>
            }
        }
    }
}
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="screen">
            <div class="topbar">
                <h2 class={"heading"}>{"Tachinka"}</h2>
                <input type="text" class="search-box"/>
            </div>

            <div class="main">

                    <DynamicGrid/>

            </div>
            <div class="botbar">
                <p>{"bot"}</p>
            </div>
        </div>
    }
}

fn display_dynamic_content_grid(content: UseStateHandle<String>) {
    spawn_local(async move {
        match get_manga_list("".to_string(), 0).await {
            Ok(message) => {
                content.set(message.as_string().unwrap());
            }
            Err(e) => {
                let window = window().unwrap();
                window
                    .alert_with_message(&format!("Error: {:?}", e))
                    .unwrap();
            }
        }
    });
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invokeGetMangaList, catch)]
    pub async fn get_manga_list(tags: String, offset: i32) -> Result<JsValue, JsValue>;

}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
