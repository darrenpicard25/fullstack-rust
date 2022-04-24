use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

use pages::{HelloServer, Home};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/error")]
    Error,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::HelloServer => html! { <HelloServer /> },
        _ => html! { <h1>{"404 Not found"}</h1>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}
