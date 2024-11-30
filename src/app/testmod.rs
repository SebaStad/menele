use yew_router::prelude::*;
use yew::prelude::*;

use crate::routes::mainpage::Home;
use crate::routes::newmenele::NewMenele;
use crate::routes::loadmenele::LoadMenele;
use crate::routes::settings::Settings;

use crate::reducers::sectionstate::SectionState;
use std::rc::Rc;

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


    html! {
        <ContextProvider<UseReducerHandle<SectionState>> context={state.clone()}>
            <BrowserRouter>
                <Switch<MainPageRoute> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </ContextProvider<UseReducerHandle<SectionState>>>
    }
}