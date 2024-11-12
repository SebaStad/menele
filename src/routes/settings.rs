use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::testmod::MainPageRoute;

#[function_component(Settings)]
pub fn settings() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&MainPageRoute::Home));
    html! {
        <div>
            <h1>{ "Settings Newspaper" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}