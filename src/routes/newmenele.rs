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

#[function_component(NewMenele)]
pub fn newmenele() -> Html {

    html! {
        <div>
            <ResizableLayout/>
        </div>
    }
}

// #[hook]
// pub fn use_is_small_screen(threshold: f64) -> bool {
//     let is_small_screen = use_state(|| {
//         let width = &window()
//             .inner_width()
//             .unwrap()
//             .as_f64()
//             .unwrap_or(1024.0);
//         *width < threshold
//     });

//     let is_small_screen = is_small_screen.clone();
//     let window2 = window();

//     {
//         use_effect_with(
//             (is_small_screen.clone(), window2.clone()),
//             {
//                 let is_small_screen = is_small_screen.clone();
//                 let window2 = window2.clone();
//                 move |_| {
//                     let window = window.clone();
//                     let is_small_screen = is_small_screen.clone();
        
//                     let on_resize = Closure::wrap(Box::new(move || {
//                         let window2 = window2.clone();
//                         if let Some(width) = &window2
//                             .inner_width()
//                             .ok()
//                             .and_then(|val| val.as_f64())
//                         {
//                             is_small_screen.set(*width < threshold);
//                         };
//                     }) as Box<dyn FnMut()>);
//                     let window2 = window();
//                     window2
//                         .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())
//                         .expect("should register resize event listener");
//                     on_resize.forget();
//                     || {}
//                 }
//         });
//     }

//     *is_small_screen
// }

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

                // let mouse_up_listener_2 = EventListener::new(&window(), "mouseup", move |_| {
                //     let window_width = window().inner_width().unwrap().as_f64().unwrap();
                //     log!("Hello", window_width);
                //     if window_width <= window_threshhold {
                //         is_small_window.set(true)
                //     }  else {
                //         is_small_window.set(false)
                //     };
                // });

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

    let sections = use_state(|| vec![]);

    let add_section = {
        let sections = sections.clone();
        Callback::from(move |_| {
            // Alternate between left and right sections
            let mut new_sections = (*sections).clone();
            let is_left = new_sections.len() % 2 == 0; // Alternate based on count
            new_sections.push(SectionData {
                content: format!("New Section {}", new_sections.len() + 1),
                is_left,
                image_url: String::from("https://www.medius-fitness.de/wp-content/uploads/2021/06/medius-Logo-550x120-DSV.png")
            });
            sections.set(new_sections);
        })
    };

    // html! {
    //     <div>
    //         <button onclick={add_section}>{ "Add Section" }</button>
    //         <Sections sections={(*sections).clone()} />
    //     </div>
    // }
// }

    html! {
        <div style="display: flex; height: 100vh; width: 100%;">
            <div style={format!("width: {}%;", *left_width)}>
                <h1>{ "Create new Newspaper" }</h1>
                <button {onclick}>{ "Go Home" }</button>
                <button onclick={add_section}>{ "Add Section" }</button>
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
                <NewsLetter is_small_window={*is_small_window} dynamic_sections={(*sections).clone()}/>
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

use yew::{function_component, html, Html, Properties, use_state, Callback};
// use yew::{function_component, html, use_state, Html};
// use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub text: String,
    pub image_url: String,
    pub is_left: bool,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let SectionProps {text, image_url, is_left} = props;
    html! {
        <div class={if *is_left { "section-left" } else { "section-right" }}>
            { &text }
            <img src={image_url.clone()}/>
        </div>
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct SectionData {
    pub content: String,
    pub image_url: String,
    pub is_left: bool,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SectionsProps {
    pub sections: Vec<SectionData>,
}

#[function_component(Sections)]
pub fn sections(props: &SectionsProps) -> Html {
    let SectionsProps {sections} = props;
    html! {
        <div class="sections-container">
            {
                for sections.iter().map(|section| {
                    html! {
                        <Section text={section.content.clone()} is_left={section.is_left} image_url = {section.image_url.clone()} />
                    }
                })
            }
        </div>
    }
}
