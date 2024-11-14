use yew::prelude::*;
use yew_router::prelude::*;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::html::Scope;
use web_sys::{MouseEvent, window};
use std::rc::Rc;
use std::cell::Cell;

use crate::app::testmod::MainPageRoute;
use crate::meneleparts::header::Header;
use crate::meneleparts::newsletter::NewsLetter;

#[function_component(NewMenele)]
pub fn newmenele() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&MainPageRoute::Home));
    html! {
        <div>
            <h1>{ "Create new Newspaper" }</h1>
            <button {onclick}>{ "Go Home" }</button>
            <br/>
            <NewsLetter/>
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