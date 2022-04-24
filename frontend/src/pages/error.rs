use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::Route;

#[function_component(Error)]
pub fn home() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));

    html! {
        <>
            <div>{"404 error page"}</div>
            <button {onclick}>{"click to go Home"}</button>
        </>
    }
}
