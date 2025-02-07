use yew::{function_component, html, use_state, Callback, Html, InputEvent, TargetCast};

// fn main() {
//     yew::Renderer::<App>::new().render();
// }
#[function_component(StringApp)]
pub fn string_app() -> Html {
    let input_value = use_state(|| String::from(""));
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input_value.set(input.value())
            }
        })
    };
    html!(
        <>
            <h1>{(*input_value).clone()}</h1>
            <input type="text"  value={(*input_value).clone()} oninput={on_input} />
        </>
    )
}
