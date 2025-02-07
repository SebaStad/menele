use crate::styling::styles::{CssClass, StyleConfig};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct EinleitungProps {
    pub introduction_title: String,
    pub introduction_image_url: String,
    pub is_small_window: bool,
}

#[function_component(Einleitung)]
pub fn einleitung(props: &EinleitungProps) -> Html {
    let EinleitungProps {
        introduction_title,
        introduction_image_url,
        is_small_window,
    } = props;
    let style_lookup = StyleConfig::new();

    let style_string_kapitel_text_links = style_lookup
        .get_style(CssClass::KapitelTextLinks, *is_small_window)
        .unwrap()
        .clone();

    let style_string_einleitung_header = style_lookup
        .get_style(CssClass::EinleitungHeader, *is_small_window)
        .unwrap()
        .clone();

    let style_string_image_artikel_rechts = style_lookup
        .get_style(CssClass::ImageArtikelRechts, *is_small_window)
        .unwrap()
        .clone();

    html! {
        <div class = "text" style = "color:black">
            <div class = "kapitel-text-links" style = {style_string_kapitel_text_links}>
                <div class = "einleitung-header" style = {style_string_einleitung_header}>
                    <p>{introduction_title}</p>
                </div>
                <div class = "einleitung-text">
                    <p>{"${content.content}"}</p>
                </div>
            </div>
            <div class="image-artikel-rechts">
                <p>
                    <img src= {introduction_image_url.clone()}
                     style = {style_string_image_artikel_rechts}/>
                </p>
            </div>
        </div>
    }
}
