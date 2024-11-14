use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::testmod::MainPageRoute;
use crate::styling::centered_container::CenteredContainer;

#[function_component(LoadMenele)]
pub fn loadmenele() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&MainPageRoute::Home));
    html! {
        <div>
            <CenteredContainer>
                <h1>{ "Load old Newspaper" }</h1>
                <button {onclick}>{ "Go Home" }</button>
            </CenteredContainer>
        </div>
    }
}