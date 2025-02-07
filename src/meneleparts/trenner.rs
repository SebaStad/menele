use crate::styling::styles::{CssClass, StyleConfig};
use yew::prelude::*;

#[function_component(TrennerMitte)]
pub fn trenner_mitte() -> Html {
    let style_lookup = StyleConfig::new();

    let style_string = style_lookup
        .get_style(CssClass::TrennerMitte, true)
        .unwrap()
        .clone();
    html! {
        <div class = "trenner-mitte" style = {style_string}>
        </div>
    }
}

#[function_component(TrennerArtikel)]
pub fn trenner_artikel() -> Html {
    let style_lookup = StyleConfig::new();

    let style_string = style_lookup
        .get_style(CssClass::TrennerArtikel, true)
        .unwrap()
        .clone();
    html! {
        <div class = "trenner-artikel" style = {style_string}>
        </div>
    }
}
