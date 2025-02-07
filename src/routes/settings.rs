use yew::prelude::*;
use yew_router::prelude::*;
// use yew_hooks::prelude::*;

use crate::app::testmod::MainPageRoute;
use crate::reducers::appstate::AppState;
use crate::reducers::globaloptions::GlobalOptionsActions;
use crate::styling::centered_container::CenteredContainer;

#[function_component(Settings)]
pub fn settings() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |route: &MainPageRoute| navigator.push(route));
    html! {
        <div>
            <CenteredContainer>
                <h1>{ "Settings Newsletter" }</h1>
                <SettingsPage/>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::Home)
                }>{ "Hauptseite" }</button>
                <br/>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::NewMenele)
                }>{ "Newsletter editieren" }</button>
            </CenteredContainer>
        </div>
    }
}

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    let appstate = use_context::<AppState>().expect("AppState context not found");
    // let state = use_context::<UseReducerHandle<SectionState>>().expect("AppState context not found");
    let global_options = &appstate.start_left_state;
    // let global_options = use_reducer(|| GlobalOptions { chapters_start_left: true });

    let on_toggle = {
        let global_options = global_options.clone();
        Callback::from(move |_| {
            global_options.dispatch(GlobalOptionsActions::SwitchChaptersStartLeft);
        })
    };

    html! {
        <div>
            <div style="display: flex; align-items: center; gap: 10px; margin-top: 10px;">
                <label>
                    <input type="checkbox"
                        checked={global_options.chapters_start_left}
                        onclick={on_toggle} />
                    {"Kapitel fangen links an"}
                </label>
            </div>
        </div>
    }
}
