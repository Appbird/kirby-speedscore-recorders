use gloo_net::http::Request;
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
    let oninput = Callback::from(move |input_event: InputEvent| {
        let value = get_value_from_input_event(input_event);
        wasm_bindgen_futures::spawn_local(async move {
            send_api_request(&value).await.unwrap_throw();
        });
    });
    html!( 
        <input type={"text"}  {oninput}/>
    )
}
fn main() {
    yew::Renderer::<App>::new().render();
}