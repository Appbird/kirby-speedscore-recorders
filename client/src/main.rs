use gloo_net::http::Request;
use gloo_timers::callback::Interval;
use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use web_sys::Text;
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
    timeout: Option<Timeout>,
    input_value: String
}
enum Msg {
    UpdateText(String),
    DispatchEvent,
}

impl App {
    fn update_text(&mut self, ctx: &Context<Self>, s:String){
        self.input_value = s;
        self.timeout = Some(Timeout::new(600, || {
            ctx.link().callback(|_:InputEvent| Msg::DispatchEvent);
        }));
    }
    fn delay_dispatch(&self) {
        wasm_bindgen_futures::spawn_local(async move {
            send_api_request(&self.input_value).await.unwrap_throw();
        })
    }
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            timeout: None,
            input_value : String::new(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateText(x) => self.update_text(ctx, x),
            Msg::DispatchEvent => self.delay_dispatch()
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