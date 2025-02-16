use yew::prelude::*;
use yew_router::prelude::*;
// use gloo::events::EventListener;
// use gloo::utils::window;
// use crate::reducers::windowsize::{WindowSizeAction, WindowSizeState};
// use wasm_bindgen::JsCast;
// use web_sys::{Document, Element, HtmlElement};

use crate::app::testmod::MainPageRoute;
use crate::meneleparts::newsletter::NewsLetterProps;
use crate::reducers::appstate::AppState;
use crate::routes::subroutes::coupled_sections::convert_sections;
use crate::styling::labels::{FrontendLabels, GLOBAL_LABELS};

#[function_component(HtmlMenele)]
pub fn htmlmenele() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |route: &MainPageRoute| navigator.push(route));

    let labels = GLOBAL_LABELS.read().unwrap();

    // let on_select_all = Callback::from(move |_| {
    //     let window = window();
    //     let document = window.document().unwrap();

    //     if let Some(pre_element) = document.get_element_by_id("html-string") {
    //         let pre_html = pre_element.unchecked_into::<HtmlElement>();

    //         // Create a selection range
    //         let selection = window.get_selection().unwrap();
    //         let range = document.create_range().unwrap();
    //         range.select_node_contents(&pre_html).unwrap();
    //         selection.remove_all_ranges().unwrap();
    //         selection.add_range(&range).unwrap();
    //     }
    // });

    // let on_copy = Callback::from(move |_| {
    //     let window = window();
    //     let document = window.document().unwrap();

    //     if let Some(pre_element) = document.get_element_by_id("html-string") {
    //         let pre_html = pre_element.unchecked_into::<HtmlElement>();

    //         let selection = window.;
    //         let range = document.create_range().unwrap();
    //         range.select_node_contents(&pre_html).unwrap();
    //         selection.remove_all_ranges().unwrap();
    //         selection.add_range(&range).unwrap();

    //         // Copy to clipboard
    //         document.exec_command("copy").unwrap();
    //     }
    // });

    html! {

        <div>
            <div style="
            display: flex;
            flex-direction: column;
            align-items: center;
            ">
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::Home)
                }>{ 
                    labels
                        .get_label(
                            FrontendLabels::MainPage
                        )
                        .unwrap()
                 }</button>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::NewMenele)
                }>{                     
                    labels
                        .get_label(
                            FrontendLabels::Edit
                        )
                        .unwrap()
                 }</button>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::LoadMenele)
                }>{                     
                    labels
                        .get_label(
                            FrontendLabels::Load
                        )
                        .unwrap()
                 }</button>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::PreviewMenele)
                }>{ 
                    labels
                        .get_label(
                            FrontendLabels::Preview
                        )
                        .unwrap()
                 }</button>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::Settings)
                }>{ 
                    labels
                        .get_label(
                            FrontendLabels::Settings
                        )
                        .unwrap()
                 }</button>
                </div>
            <br/>
            <hr/>
            <br/>
            <HtmlLayout/>
        </div>
    }
}

#[function_component(HtmlLayout)]
pub fn html_layout() -> Html {
    let appstate = use_context::<AppState>().expect("AppState context not found");
    // let state = use_context::<UseReducerHandle<SectionState>>().expect("AppState context not found");
    let state = &appstate.section_state;
    let introductionstate = &appstate.introduction_state;

    let current_newsletter = NewsLetterProps {
        is_small_window: true,
        einleitung: introductionstate.clone(),
        dynamic_sections: convert_sections(&state.clone().sections),
    };
    let html_content = current_newsletter.to_html(); // Render HTML as a string

    html! {
        <div class = "html-string">
            <pre style="white-space: pre-wrap; word-wrap: break-word;">
                { html_content.clone() }
            </pre>
        </div>
    }
}
