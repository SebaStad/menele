use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::testmod::MainPageRoute;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |route: &MainPageRoute| navigator.push(route));
    // https://stackoverflow.com/questions/77426942/how-to-reuse-a-yew-callback-for-multiple-elements
    // offical way of doing:
    // https://yew.rs/docs/0.20/concepts/router#function-components
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button onclick={
                let onclick = onclick.clone();
                move |_| onclick.emit(&MainPageRoute::NewMenele)
            }>{ "Go New" }</button>
            <br/>
            <button onclick={
                let onclick = onclick.clone();
                move |_| onclick.emit(&MainPageRoute::LoadMenele)
            }>{ "Go Load" }</button>
            <br/>
            <button onclick={
                let onclick = onclick.clone();
                move |_| onclick.emit(&MainPageRoute::Settings)
            }>{ "Go Settings" }</button>
        </div>
    }
}
