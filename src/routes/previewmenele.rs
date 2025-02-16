use crate::reducers::windowsize::{WindowSizeAction, WindowSizeState};
use gloo::events::EventListener;
use gloo::utils::window;
use yew::prelude::*;
use yew_router::prelude::*;
// use wasm_bindgen::JsCast;

use crate::app::app::MainPageRoute;
use crate::meneleparts::newsletter::NewsLetter;
use crate::reducers::appstate::AppState;
use crate::routes::subroutes::coupled_sections::convert_sections;
use crate::styling::labels::{FrontendLabels, GLOBAL_LABELS};
use crate::reducers::sectionstate::SectionAction;

#[function_component(PreviewMenele)]
pub fn previewmenele() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |route: &MainPageRoute| navigator.push(route));

    let labels = GLOBAL_LABELS
        .read()
        .expect("Expect global labels");

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
                        .get_label(FrontendLabels::MainPage)
                        .expect("Another label")
                 }</button>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::NewMenele)
                }>{ 
                    labels
                        .get_label(FrontendLabels::Edit)
                        .expect("Another label")
                 }</button>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::LoadMenele)
                }>{ 
                    labels
                        .get_label(FrontendLabels::Load)
                        .expect("Another label")
                 }</button>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::HtmlMenele)
                }>{ 
                    labels
                        .get_label(FrontendLabels::HtmlCode)
                        .expect("Another label")
                 }</button>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::Settings)
                }>{ 
                    labels
                        .get_label(FrontendLabels::Settings)
                        .expect("Another label")
                 }</button>
                </div>
            <br/>
            <hr/>
            <br/>
            <PreviewLayout/>
        </div>
    }
}

#[function_component(PreviewLayout)]
pub fn preview_layout() -> Html {
    let is_small_window = use_state(|| false);
    let is_dragging = use_state(|| false);
    let window_size_state = use_reducer(|| WindowSizeState {
        is_small_window: false,
    });
    let window_threshhold = 900.0;

    let appstate = use_context::<AppState>().expect("AppState context not found");
    // let state = use_context::<UseReducerHandle<SectionState>>().expect("AppState context not found");
    let state = &appstate.section_state;
    let introductionstate = &appstate.introduction_state;

    use_effect_with(
        (
            is_dragging.clone(),
            is_small_window.clone(),
            window_size_state.clone(),
            state.clone(),
        ),
        {
            let window_size_state = window_size_state.clone();
            let is_dragging = is_dragging.clone();
            let is_small_window = is_small_window.clone();
            let state = state.clone();
            move |_| {
                let is_dragging_2 = is_dragging.clone();

                let mouse_move_listener = EventListener::new(&window(), "mousemove", move |e| {
                    if *is_dragging_2 {
                        // let mouse_event = e.dyn_ref::<web_sys::MouseEvent>().unwrap();
                        let window_width = window().inner_width().unwrap().as_f64().unwrap();
                        // let new_width = (mouse_event.client_x() as f64 / window_width.clone()) * 100.0;
                        let right_window_size = window_width - 0.0 as f64;
                        if right_window_size <= window_threshhold {
                            // log!("ASDF", right_window_size);
                            is_small_window.set(true);
                            window_size_state.dispatch(WindowSizeAction::UpdateWindowSize {
                                is_small_window: true,
                            });
                            state.dispatch(SectionAction::UpdateWindowSize {
                                window_size: window_size_state.clone(),
                            })
                        } else {
                            // log!("Hello", right_window_size);
                            is_small_window.set(false);
                            window_size_state.dispatch(WindowSizeAction::UpdateWindowSize {
                                is_small_window: false,
                            });
                            state.dispatch(SectionAction::UpdateWindowSize {
                                window_size: window_size_state.clone(),
                            })
                        };
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
        },
    );

    use_effect_with(is_dragging.clone(), move |_| {
        let listener = EventListener::new(&window(), "resize", move |_| {
            is_dragging.set(true); // Set is_dragging to true on window resize
        });

        // Cleanup on unmount
        move || drop(listener)
    });

    html! {
        <div style={format!("width: {}%; is_small: {}, background-color: #ffffff", 100.0, *is_small_window)}>
        {
            html! {
                <NewsLetter
                is_small_window={*is_small_window}
                einleitung={introductionstate.clone()}
                dynamic_sections={convert_sections(&state.clone().sections)}
                />
            }
        }
        </div>

    }
}
