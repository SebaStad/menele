use yew::prelude::*;
use crate::styling::styles::{StyleConfig, CssClass};

#[function_component(TrennerMitte)]
pub fn trenner_mitte() -> Html {
    let style_lookup = StyleConfig::new();

    let style_string = style_lookup.get_style(
        CssClass::TrennerMitte,
        true
    ).unwrap().clone();
    html! {
        <div class = "trenner-mitte" style = {style_string}> 
        </div>
    }
}