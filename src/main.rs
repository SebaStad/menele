// mod app;
// mod tutorial_app;
// use yew::prelude::*;
// use yew_router::prelude::*;
// mod app;
// use app::app::App;
use app::testmod::Main;
// use test::test_add_section_chatgpt::AddSection;
// use test::test_textinput::StringApp;
// use test::test_coupled_input_chatgpt::CoupledApp;

mod app;
mod hooks;
mod meneleparts;
mod reducers;
mod routes;
mod styling;
mod templates;
mod test;

// use crate::app::router::{AppRoute, switch};
// use crate::routes::loadmenele::LoadNewsletter;
// use crate::routes::mainpage::Home;
// use crate::routes::newmenele::NewNewsletter;
// use crate::routes::settings::Settings;

// use app::App;
// use tutorial_app::app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<Main>::new().render();
}

// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <BrowserRouter>
//             <Switch<AppRoute> render={Switch::render(switch)} />
//         </BrowserRouter>
//     }
// }

// fn main() {
//     yew::Renderer::<App>::new().render();
// }
