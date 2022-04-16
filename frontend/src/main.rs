use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);

    let increase = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let decrease = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1>{"Counter Frontend"}</h1>
            <button onclick={increase}>{ "+1" }</button>
            <button onclick={decrease}>{ "-1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
