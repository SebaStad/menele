use std::fmt::Debug;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::testmod::MainPageRoute;
use crate::styling::centered_container::CenteredContainer;

use gloo_file::callbacks::{FileReader, read_as_text};
use gloo_file::{File, Blob};
use gloo::utils::format::JsValueSerdeExt;
use gloo::utils::document;
use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use html5ever::driver::{ParseOpts, Parser};
// use markup5ever_rcdom as rcdom;

use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::{TreeBuilderOpts,TreeSink};
use html5ever::{parse_document, serialize};
use std::cell::RefCell;
use std::rc::Rc;


#[function_component(LoadMenele)]
pub fn loadmenele() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&MainPageRoute::Home));

    // let onclick_read_newspaper: Callback<MouseEvent> = Callback::from(
    //     move |_| navigator.push(&MainPageRoute::NewMenele)
    // );
    html! {
        <div>
            <CenteredContainer>
                <h1>{ "Load old Newsletter" }</h1>
                <button {onclick}>{ "Go Home" }</button>
                <FileUpload/>
            </CenteredContainer>
        </div>
    }
}


#[function_component(FileUpload)]
fn file_upload() -> Html {
    let file_content = use_state(|| String::new());
    let reader_handle = use_state(|| None);

    let on_file_change = {
        let file_content = file_content.clone();
        Callback::from(move |event: Event| {
            let input: HtmlInputElement = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>();

            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let file_content = file_content.clone();
                    let file: Blob = File::from(file).into();
                    // log!("Selected file: {:?}", file.clone()); 
                    let _reader = read_as_text(&file, move |result| {
                        match result {
                            Ok(text) => {
                                log!("File content: {}", text.clone());
                                file_content.set(text);
                            }
                            Err(_err) => {
                                log!("Error reading file:");
                            }
                        }
                    });
                    reader_handle.set(Some(_reader));

                    // https://proxiesapi.com/articles/the-ultimate-html5ever-cheat-sheet-for-rust
                    // 2023 xD
                }
            }
            let html_opts = ParseOpts::default();
            // let html_doc = parse_document(*file_content.as_bytes());
        })
    };

    let on_button_click = Callback::from(move |_| {
        if let Some(input) = document().get_element_by_id("file-upload") {
            input.unchecked_into::<HtmlInputElement>().click();
        }
    });

    html! {
        <div>
            // Hidden file input
            <input id="file-upload" type="file" accept=".html" style="display: none;" onchange={on_file_change} />

            <button onclick={on_button_click}>{"Select HTML File"}</button>

            // <pre>{(*file_content).clone()}</pre>
        </div>
    }
}