use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::testmod::MainPageRoute;
use crate::styling::centered_container::CenteredContainer;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |route: &MainPageRoute| navigator.push(route));
    // https://stackoverflow.com/questions/77426942/how-to-reuse-a-yew-callback-for-multiple-elements
    // offical way of doing:
    // https://yew.rs/docs/0.20/concepts/router#function-components
    html! {
        <CenteredContainer>
                <h1>{ "Home" }</h1>
                <div style="
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                ">
                    <button onclick={
                        let onclick = onclick.clone();
                        move |_| onclick.emit(&MainPageRoute::NewMenele)
                    }>{ "Newsletter editieren" }</button>
                    <br/>
                    <button onclick={
                        let onclick = onclick.clone();
                        move |_| onclick.emit(&MainPageRoute::LoadMenele)
                    }>{ "Newsletter Laden" }</button>
                    <br/>
                    <button onclick={
                        let onclick = onclick.clone();
                        move |_| onclick.emit(&MainPageRoute::PreviewMenele)
                    }>{ "Newsletter Vorschau" }</button>
                    <br/>
                    <button onclick={
                        let onclick = onclick.clone();
                        move |_| onclick.emit(&MainPageRoute::HtmlMenele)
                    }>{ "Newsletter Html-Code" }</button>
                    <br/>
                    <button onclick={
                        let onclick = onclick.clone();
                        move |_| onclick.emit(&MainPageRoute::Settings)
                    }>{ "Einstellungen" }</button>
                </div>
        </CenteredContainer>
    }
}
