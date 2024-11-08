// mod app;
// mod tutorial_app;

mod app;
use app::app::App;

// use app::App;
// use tutorial_app::app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
