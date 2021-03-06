use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::Route;

#[function_component(HelloServer)]
pub fn hello_server() -> Html {
    let data = use_state(|| None);
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));

    // Request `/api/hello` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/hello").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <>
                    <div>{"No server response"}</div>
                    <button {onclick}>{"click to go hello"}</button>
                </>
            }
        }
        Some(Ok(data)) => {
            html! {
                <>
                    <div>{"Got server response: "}{data}</div>
                    <button {onclick}>{"click to go hello"}</button>
                </>
            }
        }
        Some(Err(err)) => {
            html! {
                <>
                    <div>{"Error requesting data from server: "}{err}</div>
                    <button {onclick}>{"click to go hello"}</button>
                </>
            }
        }
    }
}
