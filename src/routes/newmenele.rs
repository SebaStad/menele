use yew::prelude::*;
use yew_router::prelude::*;
// use yew::{use_effect_with_deps};
use gloo::events::EventListener;
use gloo::utils::window;

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




// #[function_component(TwoPanes)]
// fn two_panes() -> Html {
//     let left_width = Rc::new(Cell::new(50.0));  // percentage width of the left pane
//     let left_width = Rc::clone(&left_width);
//     Callback::from(move |event: MouseEvent| {
//         let start_x = event.client_x();
//         let initial_width = left_width.get();
//         let left_width = Rc::clone(&left_width);

//         // Create a closure for the "mousemove" event
//         let onmousemove = Closure::wrap(Box::new(move |event: MouseEvent| {
//             let delta_x = event.client_x() - start_x;
//             let new_width = (initial_width + delta_x as f64 / window().unwrap().inner_width().unwrap().as_f64().unwrap() * 100.0)
//                 .clamp(10.0, 90.0);  // Clamp the width between 10% and 90%
//             left_width.set(new_width);
//         }) as Box<dyn FnMut(_)>);

//         // Attach the "mousemove" event listener
//         window()
//             .expect("window not available")
//             .add_event_listener_with_callback("mousemove", onmousemove.as_ref().unchecked_ref())
//             .unwrap();
//         onmousemove.forget();

//         // Create a closure for the "mouseup" event
//         let onmouseup = Closure::wrap(Box::new(move |_: MouseEvent| {  // Explicitly set the type here
//             // Remove the "mousemove" event listener
//             window()
//                 .expect("window not available")
//                 .remove_event_listener_with_callback("mousemove", onmousemove.as_ref().unchecked_ref())
//                 .unwrap();
//         }) as Box<dyn FnMut(_)>);

//         // Attach the "mouseup" event listener
//         window()
//             .expect("window not available")
//             .add_event_listener_with_callback("mouseup", onmouseup.as_ref().unchecked_ref())
//             .unwrap();
//         onmouseup.forget();
//     });

//     html! {

//     }
// }

// https://yew.rs/docs/concepts/html/events#using-gloo-concise

#[function_component(ResizableLayout)]
pub fn resizable_layout() -> Html {
    let left_width = use_state(|| 50.0); // Left container starts at 50% width
    let is_dragging = use_state(|| false);


    let is_dragging = is_dragging.clone();
    use_effect_with(
        (is_dragging.clone(), left_width.clone()),
        {
            let is_dragging = is_dragging.clone();
            let left_width = left_width.clone();
            move |_| {
                let is_dragging_2 = is_dragging.clone();
                let mouse_move_listener = EventListener::new(&window(), "mousemove", move |e| {
                    if *is_dragging_2 {
                        let mouse_event = e.dyn_ref::<web_sys::MouseEvent>().unwrap();
                        let window_width = window().inner_width().unwrap().as_f64().unwrap();
                        let new_width = (mouse_event.client_x() as f64 / window_width) * 100.0;
                        left_width.set(new_width.clamp(10.0, 90.0)); // Clamp to prevent overlap
                    }
                });

                let mouse_up_listener = EventListener::new(&window(), "mouseup", move |_| {
                    is_dragging.set(false);
                });

                // Cleanup listeners on unmount
                || {
                    drop(mouse_move_listener);
                    drop(mouse_up_listener);
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

    html! {
        <div style="display: flex; height: 100vh; width: 100%;">
            <div style={format!("width: {}%;", *left_width)}>
                <h1>{ "Create new Newspaper" }</h1>
                <button {onclick}>{ "Go Home" }</button>
            </div>
            <div
                style="
                    width: 5px;
                    background-color: #e0e0e0;
                    cursor: ew-resize;
                "
                onmousedown={on_mouse_down}
            />
            <div style={format!("width: {}%;", 100.0 - *left_width)}>
                <NewsLetter/>
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