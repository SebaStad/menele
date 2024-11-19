use yew::prelude::*;
use crate::styling::styles::{StyleConfig, CssClass};

#[derive(PartialEq, Properties)]
pub struct HeaderImageProps {
    pub is_small_window: bool,
}

#[function_component(HeaderImage)]
pub fn header_image(props: &HeaderImageProps) -> Html {
    let HeaderImageProps {is_small_window, ..} = props;

    // log!("BSDF", !*is_small_window);
    let style_lookup = StyleConfig::new();
    let style_string = style_lookup.get_style(
        CssClass::ImageHeaderFooter,
        *is_small_window
    ).unwrap().clone();
    html! {
        <div class="image-header-footer">
            <img src="https://www.medius-fitness.de/wp-content/uploads/2022/02/Logo-rot-_medius_-2021-weiss.png"
            style = {style_string}/>
        </div>
    }
}