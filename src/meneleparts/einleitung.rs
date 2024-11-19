use yew::prelude::*;
use crate::styling::styles::{StyleConfig, CssClass};

#[derive(PartialEq, Properties)]
pub struct EinleitungProps {
    pub is_small_window: bool,
}

#[function_component(Einleitung)]
pub fn einleitung(props: &EinleitungProps) -> Html {
    let EinleitungProps {is_small_window} = props;
    let style_lookup = StyleConfig::new();

    let style_string_kapitel_text_links = style_lookup.get_style(
        CssClass::KapitelTextLinks,
        *is_small_window
    ).unwrap().clone();

    let style_string_einleitung_header = style_lookup.get_style(
        CssClass::EinleitungHeader,
        *is_small_window
    ).unwrap().clone();

    let style_string_image_artikel_rechts = style_lookup.get_style(
        CssClass::ImageArtikelRechts,
        *is_small_window
    ).unwrap().clone();
    html! {
        <div class = "text">
            <div class = "kapitel-text-links" style = {style_string_kapitel_text_links}>
                <div class = "einleitung-header" style = {style_string_einleitung_header}>
                    <p>{"EINLEITUNG"}</p>
                </div>
                <div class = "einleitung-text">
                    <p>{"${content.content}"}</p>
                </div>
            </div>
            <div class="image-artikel-rechts">
                <p>
                    <img src="https://www.medius-fitness.de/wp-content/uploads/2022/02/Ric-2022.jpg"
                    style = {style_string_image_artikel_rechts}/>
                </p>
            </div>
        </div>
    }
}