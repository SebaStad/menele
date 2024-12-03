use yew::prelude::*;
use crate::styling::styles::{StyleConfig, CssClass};

#[derive(PartialEq, Properties)]
pub struct MoreButtonProps {
    pub url_string: String,
    pub is_small_window: bool
}

#[function_component(MoreButton)]
pub fn more_button(props: &MoreButtonProps) -> Html {
    let MoreButtonProps {url_string, is_small_window} = props;

    let style_lookup = StyleConfig::new();
    let style_string = style_lookup.get_style(
        CssClass::ButtonStack,
        *is_small_window
    ).unwrap().clone();

    if url_string.len() > 0 {
        html!{
            <div>
                <link href="https://fonts.googleapis.com/css?family=Oswald:400"
                rel="stylesheet" type="text/css"/>
                <a href= {url_string.clone()} class="btnStack" target="_blank" style = {style_string}>
                {"mehr dazu >"}
                </a>
            </div>
        }

    } else {
        html! {
            <div>
            </div>
        }
    }

}