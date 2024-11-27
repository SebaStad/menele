use std::ops::Deref;
use yew::prelude::*;
use gloo::utils::window;
use gloo::events::EventListener;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::html::Scope;
use gloo_console::log;

#[hook]
pub fn use_window_size() -> bool
{
    let is_small_window = use_state(|| false);
    let window_threshhold = 900.0;
    {
        let is_small_window = is_small_window.clone();
        use_effect_with(
            is_small_window.clone(),
            {
                move |_| {
                    let mouse_move_listener = EventListener::new(&window(), "mousemove", move |e| {
                        let mouse_event = e.dyn_ref::<web_sys::MouseEvent>().unwrap();
                        let window_width = window().inner_width().unwrap().as_f64().unwrap();
                        let right_window_size = window_width - mouse_event.client_x() as f64;
                        if right_window_size <= window_threshhold {
                            // log!("ASDF", right_window_size);
                            is_small_window.set(true)
                        }  else {
                            // log!("Hello", right_window_size);
                            is_small_window.set(false)
                        };
                    });
                    
                    || {
                        drop(mouse_move_listener)
                    }
                }
            }
        );
    }

    log!("Mouse_Pos: {}", *is_small_window.clone());
    let is_small_window = is_small_window.clone();
    is_small_window.deref().clone()
}
