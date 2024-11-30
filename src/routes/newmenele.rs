use yew::prelude::*;
use yew_router::prelude::*;
// use yew::{use_effect_with_deps};
use gloo::events::EventListener;
use gloo::utils::window;
use gloo_console::log;
use wasm_bindgen::JsValue;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::html::Scope;
use web_sys::{Blob, Url};
use std::rc::Rc;
use std::cell::Cell;

use crate::app::testmod::MainPageRoute;
use crate::meneleparts::header::Header;
use crate::meneleparts::newsletter::{NewsLetter, NewsLetterProps};

use crate::routes::subroutes::newmenele_right::{SectionProps, SectionData, Sections};
use crate::routes::subroutes::newmenele_left::{InputSectionData, InputSection, InputSections};
use crate::routes::subroutes::coupled_sections::{SectionRaw, convert_sections};
use crate::reducers::windowsize::{WindowSizeAction, WindowSizeState};
use crate::reducers::sectionstate::{SectionState, SectionAction};

use std::fs::File;
use std::io::Write;

#[function_component(NewMenele)]
pub fn newmenele() -> Html {

    html! {
        <div>
            <ResizableLayout/>
        </div>
    }
}

// https://yew.rs/docs/concepts/html/events#using-gloo-concise

#[function_component(ResizableLayout)]
pub fn resizable_layout() -> Html {
    let left_width = use_state(|| 50.0);
    let is_dragging = use_state(|| false);
    let is_small_window = use_state(|| false);
    let window_threshhold = 900.0;

    let window_size_state = use_reducer(
        || WindowSizeState {is_small_window: false}
    );
    let state = use_reducer(
        || SectionState { sections: vec![] }
    );

    use_effect_with(
        (
            is_dragging.clone(),
            left_width.clone(),
            is_small_window.clone(),
            window_size_state.clone(),
            state.clone()
        ),
        {
            let window_size_state = window_size_state.clone();
            let state = state.clone();
            let is_dragging = is_dragging.clone();
            let left_width = left_width.clone();
            let is_small_window = is_small_window.clone();
            move |_| {
                let is_dragging_2 = is_dragging.clone();

                let mouse_move_listener = EventListener::new(&window(), "mousemove", move |e| {
                    if *is_dragging_2 {
                        let mouse_event = e.dyn_ref::<web_sys::MouseEvent>().unwrap();
                        let window_width = window().inner_width().unwrap().as_f64().unwrap();
                        let new_width = (mouse_event.client_x() as f64 / window_width.clone()) * 100.0;
                        let right_window_size = window_width - mouse_event.client_x() as f64;
                        if right_window_size <= window_threshhold {
                            log!("ASDF", right_window_size);
                            is_small_window.set(true);
                            window_size_state.dispatch(WindowSizeAction::UpdateWindowSize {is_small_window: true});
                            state.dispatch(SectionAction::UpdateWindowSize { window_size: window_size_state.clone() })
                        }  else {
                            log!("Hello", right_window_size);
                            is_small_window.set(false);
                            window_size_state.dispatch(WindowSizeAction::UpdateWindowSize {is_small_window: false});
                            state.dispatch(SectionAction::UpdateWindowSize { window_size: window_size_state.clone() })
                        };
                        left_width.set(new_width.clamp(20.0, 80.0)); // Clamp to prevent overlap
                    }
                });

                let mouse_up_listener = EventListener::new(&window(), "mouseup", move |_| {
                    is_dragging.set(false);
                });

                // Cleanup listeners on unmount
                || {
                    drop(mouse_move_listener);
                    drop(mouse_up_listener);
                    // drop(mouse_up_listener_2);
                }
            }
        }
        // (), // No dependencies, only run once
    );


    let on_mouse_down = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |_| {
            is_dragging.set(true);
        })
    };

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&MainPageRoute::Home));

    let add_section = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(SectionAction::AddSection {window_size: window_size_state.clone()});
        })
    };

    let remove_section = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(SectionAction::RemoveSection);
        })
    };

    let print_html = {
        let state = state.clone();
        let is_small_window = {*is_small_window};
        Callback::from(move |_: yew::MouseEvent| {
            let current_newsletter = NewsLetterProps {
                is_small_window: is_small_window,
                dynamic_sections: convert_sections(&state.clone().sections)
            };
            log!("{}", current_newsletter.to_html());
            println!("{}", current_newsletter.to_html());
            // let mut file = File::create("newsletter.html").expect("Unable to create file");
            // file.write_all(current_newsletter.to_html().as_bytes()).expect("Unable to write data");
        })
    };

    let export_html = {
        let state = state.clone();
        let is_small_window = {*is_small_window};
        // https://www.reddit.com/r/rust/comments/15h064x/download_generated_csv_in_yew_app/
        Callback::from(move |_: yew::MouseEvent| {
            // Generate the HTML content
            let current_newsletter = NewsLetterProps {
                is_small_window,
                dynamic_sections: convert_sections(&state.clone().sections),
            };
            let html_content = current_newsletter.to_html(); // Render HTML as a string
    
            // Create a Blob with the HTML content
            let blob_parts = js_sys::Array::new();
            blob_parts.push(&html_content.into());
            let blob = Blob::new_with_str_sequence_and_options(
                &blob_parts,
                web_sys::BlobPropertyBag::new().type_("text/html"),
            ).expect("Failed to create Blob");
    
            // Generate a URL for the Blob
            let url = Url::create_object_url_with_blob(&blob).expect("Failed to create URL");
    
            // Trigger download
            let window = window();
            let document = window.document().expect("No document found");
            let a = document.create_element("a").expect("Failed to create anchor element");
            let a = a.dyn_into::<web_sys::HtmlElement>().expect("Not an anchor element");

            let _ = a.set_attribute("href", &url);
            let _ = a.set_attribute("download", "newsletter.html"); 
            a.click(); // Simulate click
    
            // Clean up the URL object after use
            Url::revoke_object_url(&url).expect("Failed to revoke object URL");
        })
    };

    let input_container_style = String::from(
        // "display: flex;
        // "align-items: center;
        // margin-top: 16px;"
        
        // "display: block; "
        ""
    );
    let input_box_style = String::from(
        // "width: 300px;
        // height: 100px;
        // padding: 8px;
        // font-size: 16px;"
        "display: block; "
    );


    html! {
        <div style="display: flex; height: 100vh; width: 100%;">
            <div style={format!("width: {}%;", *left_width)}>
                <h1>{ "Create new Newspaper" }</h1>
                <button {onclick}>{ "Go Home" }</button>
                <button onclick={add_section}>{ "Add Section" }</button>
                <button onclick={remove_section}>{ "Remove Section" }</button>
                <button onclick={print_html}> { "Print html"} </button>
                <button onclick={export_html}> { "Export html"} </button>
                <div>
                { for state.sections.iter().map(|section| {
                    let state = state.clone();
                    let state_2 = state.clone();
                    let state_3 = state.clone();
                    let id = section.id;
                    // let id2 = id.clone();
                    let oninput_chapter_title = Callback::from(move |e: yew::events::InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap() ;
                        state.dispatch(SectionAction::UpdateChapterTitle { id, text: input.value() } );
                    });

                    let oninput_text = Callback::from(move |e: yew::events::InputEvent| {
                        let input: web_sys::HtmlTextAreaElement = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap() ;
                        state_2.dispatch(SectionAction::UpdateText { id, text: input.value() });
                    });
                    
                    let on_input_image = Callback::from(move |e: yew::events::InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap() ;
                        state_3.dispatch(SectionAction::UpdateImage { id, text: input.value() });
                    });

                    html! {
                        <div style = {input_container_style.clone()}>
                            <label>{ format!("Kapitel Name {}", id + 1) }</label>
                            <input
                                type="text"
                                placeholder={format!("Section {}", id + 1)}
                                value={section.chapter_title.clone()}
                                oninput={oninput_chapter_title}
                            />
                            <br />
                            <label>{ format!("Text Kapitel {}", id + 1) }</label>
                            <textarea style = {input_box_style.clone()}
                                type="text"
                                placeholder={format!("Section {}", id + 1)}
                                value={section.text.clone()}
                                oninput={oninput_text}
                            />
                            <label>{ format!("Bild-Url Kapitel {}", id + 1) }</label>
                            <br />
                            <input
                                type="text"
                                placeholder={format!("Section {}", id + 1)}
                                value={section.image_url.clone()}
                                oninput={on_input_image}
                            />
                        </div>
                    }
                })}
                </div>
            </div>
            <div
                style="
                    width: 5px;
                    background-color: #e0e0e0;
                    cursor: ew-resize;
                "
                onmousedown={on_mouse_down}
            />
            <div style={format!("width: {}%; is_small: {}", 100.0 - *left_width, *is_small_window)}>
            {      
                html! {
                    <NewsLetter
                    is_small_window={*is_small_window}
                    dynamic_sections={convert_sections(&state.clone().sections)}
                    />
                }
            }
            </div>
        </div>
    }
}

