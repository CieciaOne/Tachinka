use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}



#[function_component(App)]
pub fn app() -> Html {
    let welcome = use_state_eq(|| "".to_string());

    // {
    //     let welcome = welcome.clone();
    //     use_effect_with_deps(
    //         move |name| {
    //             update_welcome_message(welcome, name.clone());
    //             || ()
    //         },
    //         (*name).clone(),
    //     );
    // }
    let scale = "1";
    let arr = ["A","B","C","D","E","F","G","H","A","B","C","D","E","F","G","H"];
    html! {
        <div class="screen">
            <div class="topbar">
                <h2 class={"heading"}>{"Tachinka"}</h2>
                <input type="text" class="search-box"/>
            </div>

            <div class="main">
                {
                    arr.iter().map(|i| {
                        html!{
                            <div id={*i} class="item" style="color:white;">{format!("Object: {}",i)}</div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="botbar">
                <p>{"bot"}</p>
            </div>
        </div>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        match hello(name).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap());
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
}
