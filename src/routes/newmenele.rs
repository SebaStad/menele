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
// use web_sys::{MouseEvent, window};
use std::rc::Rc;
use std::cell::Cell;

use crate::app::testmod::MainPageRoute;
use crate::meneleparts::header::Header;
use crate::meneleparts::newsletter::NewsLetter;


use crate::routes::subroutes::newmenele_right::{SectionProps, SectionData, Sections};
use crate::routes::subroutes::newmenele_left::{InputSectionData, InputSection, InputSections};
use crate::routes::subroutes::coupled_sections::{SectionState, SectionAction, SectionRaw, convert_sections};

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
    let left_width = use_state(|| 50.0); // Left container starts at 50% width
    let is_dragging = use_state(|| false);
    let is_small_window = use_state(|| false);
    let window_threshhold = 900.0;

    use_effect_with(
        (is_dragging.clone(), left_width.clone(), is_small_window.clone()),
        {
            let is_dragging = is_dragging.clone();
            let left_width = left_width.clone();
            let is_small_window = is_small_window.clone();
            move |_| {
                let is_dragging_2 = is_dragging.clone();
                // let is_small_window = is_small_window.clone();
                let mouse_move_listener = EventListener::new(&window(), "mousemove", move |e| {
                    if *is_dragging_2 {
                        let mouse_event = e.dyn_ref::<web_sys::MouseEvent>().unwrap();
                        let window_width = window().inner_width().unwrap().as_f64().unwrap();
                        let new_width = (mouse_event.client_x() as f64 / window_width.clone()) * 100.0;
                        // let mouse_position_x = mouse_event.client_x() as f64;
                        let right_window_size = window_width - mouse_event.client_x() as f64;
                        if right_window_size <= window_threshhold {
                            // log!("ASDF", right_window_size);
                            is_small_window.set(true)
                        }  else {
                            // log!("Hello", right_window_size);
                            is_small_window.set(false)
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

    // let sections = use_state(|| vec![]);
    // let input_sections = use_state(|| vec![]);
    // let input_text_fields = use_state(|| vec![]);

    // let add_section = {
    //     let sections = sections.clone();
    //     let input_sections = input_sections.clone();
    //     Callback::from(move |_| {
    //         let mut new_sections = (*sections).clone();
    //         let mut new_input_sections = (*input_sections).clone();
    //         // let mut input_text_fields = input_text_fields.clone();
    //         let is_left = new_sections.len() % 2 == 0;
            
    //         new_sections.push(SectionData {
    //             content: format!("New Section {}", new_sections.len() + 1),
    //             is_left,
    //             image_url: String::from("https://www.medius-fitness.de/wp-content/uploads/2021/06/medius-Logo-550x120-DSV.png")
    //         });
    //         sections.set(new_sections);
    //     })
    // };

    let state = use_reducer(|| SectionState { sections: vec![] });
    let small_window = is_small_window.clone();
    let add_section = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(SectionAction::AddSection {is_small_window: small_window.clone()});
        })
    };

    let remove_section = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(SectionAction::RemoveSection);
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
                <div>
                { for state.sections.iter().map(|section| {
                    let state = state.clone();
                    let state_2 = state.clone();
                    let id = section.id;
                    // let id2 = id.clone();
                    let oninput_text = Callback::from(move |e: yew::events::InputEvent| {
                        let input: web_sys::HtmlTextAreaElement = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap() ;
                        state.dispatch(SectionAction::UpdateText { id, text: input.value() });
                    });
                    
                    let on_input_image = Callback::from(move |e: yew::events::InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap() ;
                        state_2.dispatch(SectionAction::UpdateImage { id, text: input.value() });
                    });

                    html! {
                        <div style = {input_container_style.clone()}>
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



enum MeneleSection {
    left(MeneleSectionLeft),
    right(MeneleSectionRight)
}

struct MeneleSectionLeft {
}

struct MeneleSectionRight {
}


// https://github.com/yewstack/yew/discussions/2400
enum Msg {
    AddOne,
    RemoveOne,
}

#[derive(PartialEq, Properties)]
pub struct MeneleSectionsPropsProps {}

#[function_component]
pub fn MeneleSectionsProps(props: &MeneleSectionsPropsProps) -> Html {
    let MeneleSectionsPropsProps {} = props;
    html! {
        <div></div>
    }
}


struct MeneleSections {
    menele_sections: Vec<MeneleSection>
}

impl Component for MeneleSections {
    type Message = Msg;
    type Properties = MeneleSectionsPropsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            menele_sections: Vec::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {

        }
    }

}
