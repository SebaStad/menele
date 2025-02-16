use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::app::MainPageRoute;
use crate::styling::centered_container::CenteredContainer;
use crate::styling::labels::{FrontendLabels, GLOBAL_LABELS};

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |route: &MainPageRoute| navigator.push(route));
    let labels = GLOBAL_LABELS.read().expect("Global Lock");
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
                    }>{ 
                        labels
                            .get_label(FrontendLabels::MainPageEdit)
                            .expect("Mainpageedit")
                    }</button>
                    <br/>
                    <button onclick={
                        let onclick = onclick.clone();
                        move |_| onclick.emit(&MainPageRoute::LoadMenele)
                    }>{ 
                        labels
                            .get_label(FrontendLabels::Load)
                            .expect("Mainpageedit")
                     }</button>
                    <br/>
                    <button onclick={
                        let onclick = onclick.clone();
                        move |_| onclick.emit(&MainPageRoute::PreviewMenele)
                    }>{ 
                        labels
                            .get_label(FrontendLabels::Preview)
                            .expect("Mainpageedit")
                     }</button>
                    <br/>
                    <button onclick={
                        let onclick = onclick.clone();
                        move |_| onclick.emit(&MainPageRoute::HtmlMenele)
                    }>{ 
                        labels
                            .get_label(FrontendLabels::HtmlCode)
                            .expect("Mainpageedit")
                     }</button>
                    <br/>
                    <button onclick={
                        let onclick = onclick.clone();
                        move |_| onclick.emit(&MainPageRoute::Settings)
                    }>{ 
                        labels
                            .get_label(FrontendLabels::Settings)
                            .expect("Mainpageedit")
                     }</button>
                </div>
        </CenteredContainer>
    }
}
