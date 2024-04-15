use gloo_net::http::Request;
use gloo_net::websocket;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen_futures;
use std::future::Future;


fn send_api_request(value:&str) -> impl Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> {
    Request::get("../api/search")
            .query([("query", value)])
            .send()
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}



#[function_component]
fn App() -> Html {
    let time = use_state(|| 0);
    let input_value = use_state(|| String::new());
    
    let time_ref = time.clone();
    let input_value_ref = input_value.clone();

    // FIXED: updateされるたびに複数回intervalが作られてしまって困る
    let interval = Interval::new(100, move || { 
        let input_value_cloned = input_value.clone();
        if *time > 0 { time.set(*time - 100); }
        if *time == 0 {
            wasm_bindgen_futures::spawn_local(async move {
                send_api_request(&*input_value_cloned).await.unwrap_throw();
            });
        }
        web_sys::console::log_1(&(*time).to_string().into());
    });
    interval.forget();

    // FIXME ここのムーブの問題どうする？
    let oninput = Callback::from(move |input_event: InputEvent| {
        let value = get_value_from_input_event(input_event);
        time_ref.set(600);
        input_value_ref.set(value.clone());
    });
    html!( 
        <input type={"text"}  {oninput}/>
    )
}
fn main() {
    yew::Renderer::<App>::new().render();
}