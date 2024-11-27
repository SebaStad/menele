use yew::{function_component, html, Callback, Html, Properties, Reducible, UseReducerHandle, use_reducer, TargetCast};
use yew::prelude::*;
use gloo_console::log;
use std::rc::Rc;
use crate::routes::subroutes::newmenele_right::SectionData;


#[derive(Clone, Debug, PartialEq)]
pub struct WindowSizeState {
    pub is_small_window: bool,
}

pub enum WindowSizeAction {
    UpdateWindowSize {is_small_window: bool},
}

impl Reducible for WindowSizeState {
    type Action = WindowSizeAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            WindowSizeAction::UpdateWindowSize {is_small_window} => {
                log!("Reducer called: UpdateWindowSize -> {:?}", is_small_window);
                let is_small_window = is_small_window;
                Self { is_small_window }.into()
            }
        }
    }
}
