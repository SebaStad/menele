use yew_router::prelude::*;
use yew::prelude::*;

use crate::routes::mainpage::Home;
use crate::routes::newmenele::NewMenele;
use crate::routes::loadmenele::LoadMenele;
use crate::routes::settings::Settings;
use crate::routes::previewmenele::PreviewMenele;
use crate::routes::htmlmenele::HtmlMenele;

use crate::reducers::sectionstate::SectionState;
use crate::reducers::introductionstate::IntroductionState;
use crate::reducers::appstate::AppState;
use crate::reducers::globaloptions::GlobalOptions;
// use std::rc::Rc;

#[derive(PartialEq, Properties)]
pub struct HelloWorldProps {}

#[function_component]
pub fn HelloWorld(props: &HelloWorldProps) -> Html {
    let HelloWorldProps {} = props;
    html! {
        <div></div>
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum MainPageRoute {
    #[at("/")]
    Home,
    #[at("/newmenele")]
    NewMenele,
    #[at("/loadmenele")]
    PreviewMenele,
    #[at("/previewmenele")]
    HtmlMenele,
    #[at("/htmlmenele")]
    LoadMenele,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: MainPageRoute) -> Html {
    match routes {
        MainPageRoute::Home => html! {
            <Home />
        },
        MainPageRoute::NewMenele => html! {
            <NewMenele />
        },
        MainPageRoute::LoadMenele => html! {
            <LoadMenele />
        },
        MainPageRoute::PreviewMenele => html! {
            <PreviewMenele />
        },
        MainPageRoute::HtmlMenele => html! {
            <HtmlMenele />
        },
        MainPageRoute::Settings => html! {
            <Settings />
        },
        MainPageRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    let state = use_reducer(
        || SectionState { sections: vec![] }
    );

    let introductionstate = use_reducer(
        || IntroductionState {
            main_image_url: String::from("https://www.medius-fitness.de/wp-content/uploads/2022/02/2022_RiciRing.jpg"),
            introduction_title: String::from("Einleitung"),
            introduction_image_url: String::from("https://www.medius-fitness.de/wp-content/uploads/2022/02/Ric-2022.jpg")
        }
    );

    let start_left_state = use_reducer(
        || GlobalOptions {
            chapters_start_left: true
        }
    );

    let app_state = AppState {
        section_state: state,
        introduction_state: introductionstate,
        start_left_state: start_left_state
    };

    // Context provider gives me information about the state inside the browserrouter
    // also, it is saved outside, so when i change it, it stays and is not overwritten
    // when i go into "new" again :)
    // This way, i should also be able to save settings?
    // But settings only work per session i assume...
    html! {
        <ContextProvider<AppState> context={app_state}>
            <BrowserRouter>
                <Switch<MainPageRoute> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </ContextProvider<AppState>>
    }
}