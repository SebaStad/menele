use yew::{function_component, html, Html, Properties, use_state, Callback, UseStateHandle};
use crate::styling::styles::{CssClass, StyleConfig};
use crate::meneleparts::trenner::TrennerArtikel;
// use yew::{function_component, html, use_state, Html};
// use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub text: String,
    pub image_url: String,
    pub is_left: bool,
    pub is_small_window: bool
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let SectionProps {text, image_url, is_left, is_small_window} = props;

    let style_lookup = StyleConfig::new();
    let style_string_text = style_lookup.get_style(
        CssClass::KapitelTextRechts,
        *is_small_window
    ).unwrap().clone();

    if is_left.clone() {
        let style_string_text = style_lookup.get_style(
            CssClass::KapitelTextLinks,
            *is_small_window
        ).unwrap().clone();
    }

    let style_string_image = style_lookup.get_style(
        CssClass::ImageArtikelLinks,
        *is_small_window
    ).unwrap().clone();

    if is_left.clone() {
        let style_string_image = style_lookup.get_style(
            CssClass::ImageArtikelRechts,
            *is_small_window
        ).unwrap().clone();
    }
    html! {
        <div>
            <div class={if *is_left { "kapitel-text-links" } else { "kapitel-text-rechts" }}
            style = {style_string_text}>
                { 
                    text.split("\n").map(
                        |line|
                        html!{
                            <>
                                {line}
                                <br />
                            </>
                        }
                    ).collect::<Html>() 
                }
            </div>
            <div class={if *is_left { "image-artikel-rechts" } else { "image-artikel-links" }}
            style = {style_string_image}>
                <img src={image_url.clone()}/>
            </div>
            <TrennerArtikel />
        </div>
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct SectionData {
    pub content: String,
    pub image_url: String,
    pub is_left: bool,
    pub is_small_window: UseStateHandle<bool>
}

#[derive(Properties, PartialEq, Clone)]
pub struct SectionsProps {
    pub sections: Vec<SectionData>,
}

#[function_component(Sections)]
pub fn sections(props: &SectionsProps) -> Html {
    let SectionsProps {sections} = props;
    html! {
        <div class="sections-container">
            {
                for sections.iter().map(|section| {
                    html! {
                        <Section text={section.content.clone()} is_left={section.is_left} image_url = {section.image_url.clone()} is_small_window = {*section.is_small_window.clone()} />
                    }
                })
            }
        </div>
    }
}
