use yew::prelude::*;
use crate::styling::styles::{StyleConfig, CssClass};

#[derive(PartialEq, Properties)]
pub struct MainImageProps {
    pub image_url: String,
    pub is_small_window: bool,
}

#[function_component(MainImage)]
pub fn main_image(props: &MainImageProps) -> Html {
    let MainImageProps {image_url,is_small_window} = props;
    let style_lookup = StyleConfig::new();
    let style_string = style_lookup.get_style(
        CssClass::MainImage,
        *is_small_window
    ).unwrap().clone();
    html! {
        <div class = "main-image">
            <p>
                <img src= {image_url.clone()}
                style = {style_string}/>
            </p>
        </div>
    }
}