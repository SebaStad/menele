use yew::prelude::*;
use crate::styling::styles::{StyleConfig, CssClass};
use crate::styling::headerfooterstyle::HeaderFooterStyle;

use crate::meneleparts::headerimage::HeaderImage;

#[derive(Clone, PartialEq, Properties)]
pub struct HeaderProps {
    pub is_small_window: bool
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps {is_small_window, ..} = props;

    let style_lookup = StyleConfig::new();
    let style_string = style_lookup.get_style(
        CssClass::NewsText,
        *is_small_window
    ).unwrap().clone();

    html! {
        <HeaderFooterStyle is_small_window={*is_small_window}>
            <HeaderImage is_small_window={*is_small_window}/>
            <div class="news-text" style={style_string}>
                <p>{"medius news"}</p>
            </div>
        </HeaderFooterStyle>
    }
}