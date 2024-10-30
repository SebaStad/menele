mod app;
mod tutorial_app;

// use app::App;
use tutorial_app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
