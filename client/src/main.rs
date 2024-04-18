use gloo_net::http::Request;
use gloo_timers::callback::Timeout;
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

struct App {
    timeout: Option<Timeout>
}
enum Msg {
    UpdateText(String),
    DispatchEvent(String),
}


impl App {
    fn update_text(&mut self, ctx: &Context<Self>, s:String){
        let link = ctx.link().clone();
        self.timeout = Some(Timeout::new(600, move || {
            link.send_message(Msg::DispatchEvent(s));
        }));
    }
    fn delay_dispatch(&self, x:String) {
        wasm_bindgen_futures::spawn_local(async move {
            send_api_request(&x).await.unwrap_throw();
        })
    }
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            timeout: None
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        match msg {
            Msg::UpdateText(x) => self.update_text(ctx, x),
            Msg::DispatchEvent(x) => {
                self.delay_dispatch(x)
            }
        };
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let input = |input_event| {
            let s = get_value_from_input_event(input_event);
            Msg::UpdateText(s)
        };
        html!( 
            <input type={"text"}  oninput={ctx.link().callback(input)}/>
        )
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}