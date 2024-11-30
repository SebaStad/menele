use yew::{function_component, html, Html, Properties, use_state, Callback, UseStateHandle};
use crate::styling::styles::{CssClass, StyleConfig};
use crate::meneleparts::trenner::TrennerArtikel;
use gloo_console::log;

use crate::templates::sectionleft_template::SectionleftTemplate;
use crate::templates::sectionright_template::SectionrightTemplate;
use crate::templates::section_enum::SectionTemplate;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub chapter_title: String,
    pub text: String,
    pub image_url: String,
    pub is_left: bool,
    pub is_small_window: bool
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let SectionProps {chapter_title, text, image_url, is_left, is_small_window} = props;

    let deref_is_left = is_left.clone();

    let style_lookup = StyleConfig::new();
    let mut style_string_text = style_lookup.get_style(
        CssClass::KapitelTextRechts,
        is_small_window.clone()
    ).unwrap().clone();

    if deref_is_left {
        style_string_text = style_lookup.get_style(
            CssClass::KapitelTextLinks,
            is_small_window.clone()
        ).unwrap().clone();
    }

    let mut style_string_image = style_lookup.get_style(
        CssClass::ImageArtikelLinks,
        is_small_window.clone()
    ).unwrap().clone();

    if deref_is_left {
        style_string_image = style_lookup.get_style(
            CssClass::ImageArtikelRechts,
            is_small_window.clone()
        ).unwrap().clone();
    }

    let style_string_header = style_lookup
        .get_style(CssClass::SonstigeHeader, is_small_window.clone())
        .unwrap()
        .clone();

    html! {
        <div class = "text">
            <div class={if *is_left { "kapitel-text-links" } else { "kapitel-text-rechts" }}
            style = {style_string_text}>
                <div class = "sonstige-header" style = {style_string_header}>
                    <p>
                        {chapter_title.clone()}
                    </p>
                </div>
                <div class = "kapitel-text">
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
    pub chapter_title: String,
    pub content: String,
    pub image_url: String,
    pub is_left: bool,
    pub is_small_window: bool
}


impl SectionData {
    pub fn to_html(&self) -> String {
        let image_url = &self.image_url;
        let image_string = format!(
            "<img src=\"{image_url}\">"
        );

        let template = if self.is_left {

            SectionTemplate::Left(SectionleftTemplate {
                headline: &self.chapter_title,
                text: &self.content,
                image: &image_string
            })

        } else {

            SectionTemplate::Right(SectionrightTemplate {
                headline: &self.chapter_title,
                text: &self.content,
                image: &image_string
            })
        };

        template.render().unwrap()
    }
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
                    log!("is small windiw", section.is_small_window.clone());
                    log!("is left", section.is_left);
                    html! {
                        <Section
                        chapter_title = {section.chapter_title.clone()} 
                        text = {section.content.clone()}
                        is_left = {section.is_left}
                        image_url = {section.image_url.clone()}
                        is_small_window = {section.is_small_window.clone()}
                        />
                    }
                })
            }
        </div>
    }
}
