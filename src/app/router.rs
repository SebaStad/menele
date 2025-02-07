use yew::prelude::*;
use yew_router::prelude::*;

// use crate::routes::mainpage::Home;
use crate::routes::htmlmenele::HtmlMenele;
use crate::routes::loadmenele::LoadMenele;
use crate::routes::newmenele::NewMenele;
use crate::routes::previewmenele::PreviewMenele;
use crate::routes::settings::Settings;

// Define the routes for the app
#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum AppRoute {
    // #[at("/")]
    // Home,
    #[at("/new-newsletter")]
    NewNewsletter,
    #[at("/load-newsletter")]
    LoadNewsletter,
    #[at("/preview-newsletter")]
    PreviewNewsletter,
    #[at("/Html-newsletter")]
    HtmlNewsletter,
    #[at("/settings")]
    Settings,
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes {
        // AppRoute::Home => html! { <Home /> },
        AppRoute::NewNewsletter => html! { <NewMenele /> },
        AppRoute::LoadNewsletter => html! { <LoadMenele /> },
        AppRoute::PreviewNewsletter => html! { <PreviewMenele />},
        AppRoute::HtmlNewsletter => html! { <HtmlMenele />},
        AppRoute::Settings => html! { <Settings /> },
    }
}
