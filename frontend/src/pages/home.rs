use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::HelloServer));

    html! {
        <>
            <div>{"Hello from Yew. this is the real deal"}</div>
            <button {onclick}>{"click to go hello"}</button>
        </>
    }
}
