use yew::prelude::*;
use crate::styling::styles::{StyleConfig, CssClass};

/// A reusable component that centers its child elements.
#[derive(Properties, PartialEq)]
pub struct HeaderFooterStyleProps {
    pub is_small_window: bool,
    #[prop_or_default]
    pub children: Children, // This allows the component to accept child elements
}

#[function_component(HeaderFooterStyle)]
pub fn header_footer_style(props: &HeaderFooterStyleProps) -> Html {
    let HeaderFooterStyleProps {is_small_window, ..} = props;

    let style_lookup = StyleConfig::new();
    let style_string = style_lookup.get_style(
        CssClass::HeaderFooter,
        *is_small_window
    ).unwrap().clone();
    html! {
        <div style={style_string} class="header-footer" id="header-footer">
            <div>
                { for props.children.iter() }
            </div>
        </div>
    }
}