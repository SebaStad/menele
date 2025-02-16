use yew::prelude::*;
use yew_router::prelude::*;
use gloo::events::EventListener;
use gloo::utils::window;
use crate::reducers::windowsize::{WindowSizeAction, WindowSizeState};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window, Selection};
use gloo_console::log;

// use web_sys::selection::Selection;

use crate::app::app::MainPageRoute;
use crate::meneleparts::newsletter::NewsLetterProps;
use crate::reducers::appstate::AppState;
use crate::routes::subroutes::coupled_sections::convert_sections;
use crate::styling::labels::{FrontendLabels, GLOBAL_LABELS};

#[function_component(HtmlMenele)]
pub fn htmlmenele() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |route: &MainPageRoute| navigator.push(route));

    let labels = GLOBAL_LABELS.read().unwrap();

    let on_select_all = Callback::from(move |_: MouseEvent| {
        let window: Window = window();
        let document = window.document().unwrap();
        let all_relevant_documents = document.get_elements_by_class_name("html-string");
    
        if let Some(pre_element) = all_relevant_documents.get_with_index(0) {
            let pre_html = pre_element.unchecked_into::<HtmlElement>();

            // Create a selection range
            // let window_2 = window.clone();
            let selection = window.get_selection().unwrap().unwrap();
            let range = document.create_range().unwrap();
            range.select_node_contents(&pre_html).unwrap();
            selection.remove_all_ranges().unwrap();
            selection.add_range(&range).unwrap();
        }
    });

    let on_copy = Callback::from(move |_: MouseEvent| {
        let window: Window = window();
        let document = window.document().unwrap();

        let all_relevant_documents = document.get_elements_by_class_name("html-string");
    
        if let Some(pre_element) = all_relevant_documents.get_with_index(0) {
            let pre_html = pre_element.unchecked_into::<HtmlElement>();
            let text_to_copy = pre_html.inner_text(); // Extract the text inside <pre>
    
            let thenavigator: web_sys::Navigator = window.navigator();
            let clipboard = thenavigator.clipboard();
            let promise = clipboard.write_text(&text_to_copy);
    
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = wasm_bindgen_futures::JsFuture::from(promise).await {
                    web_sys::console::error_1(&e);
                }
            });
        }
    });

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
            <div style="
            display: flex;
            flex-direction: column;
            align-items: center;
            ">
                <button onclick = {on_select_all}>
                {
                    "Alles auswählen"
                }
                </button>
                <br/>
                <button onclick = {on_copy}>
                {
                    "Alles kopieren"
                }
                </button>
            </div> 
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
