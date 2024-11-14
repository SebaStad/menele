use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::testmod::MainPageRoute;
use crate::styling::centered_container::CenteredContainer;

#[function_component(Settings)]
pub fn settings() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&MainPageRoute::Home));
    html! {
        <div>
            <CenteredContainer>
                <h1>{ "Settings Newspaper" }</h1>
                <button {onclick}>{ "Go Home" }</button>
            </CenteredContainer>
        </div>
    }
}