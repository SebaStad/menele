use yew::prelude::*;
use yew::{UseReducerHandle};
use gloo_console::log;
use wasm_bindgen::JsValue;

// use crate::meneleparts::header::Header;
use crate::meneleparts::introduction::IntroductionHeader;
use crate::styling::headerfooterstyle::HeaderFooterStyle;
use crate::styling::styles::{StyleConfig, CssClass};

use crate::meneleparts::header::Header;
use crate::meneleparts::headerimage::HeaderImage;
use crate::meneleparts::mainimage::MainImage;
use crate::meneleparts::einleitung::Einleitung;
use crate::meneleparts::trenner::{TrennerMitte};
use crate::meneleparts::footer::Footer;
use crate::meneleparts::impressum::Impressum;
use crate::reducers::introductionstate::IntroductionState;

use crate::routes::subroutes::newmenele_right::{SectionData, Sections};

use crate::templates::menele_template::MeneleTemplate;
use askama::Template;

#[derive(PartialEq, Properties)]
pub struct NewsLetterProps {
    pub is_small_window: bool,
    pub einleitung: UseReducerHandle<IntroductionState>,
    pub dynamic_sections: Vec<SectionData>
}

#[function_component]
pub fn NewsLetter(props: &NewsLetterProps) -> Html {
    let NewsLetterProps {is_small_window, einleitung, dynamic_sections} = props;

    let main_image_url = einleitung.main_image_url.clone();

    let introduction_title = einleitung.introduction_title.clone();
    let introduction_image_url = einleitung.introduction_image_url.clone();

    html! {
        <div style = {"background-color: #ffffff"}>
            <Header is_small_window={*is_small_window}/>
            <MainImage image_url = {main_image_url} is_small_window={*is_small_window}/>
            <Einleitung introduction_title={introduction_title} introduction_image_url={introduction_image_url} is_small_window={*is_small_window}/>
            <TrennerMitte/>
            <Sections sections={(*dynamic_sections).clone()} />
            <Footer is_small_window={*is_small_window}/>
            <Impressum is_small_window={*is_small_window}/>
        </div>
    }
}

impl NewsLetterProps {

    pub fn to_html(&self) -> String {
        let main_image_url = self.einleitung.main_image_url.clone();
        let main_image_element = format!(
            "<img src={main_image_url}>"
        );

        let introduction_image_url = self.einleitung.introduction_image_url.clone();
        let introduction_image_element = format!(
            "<img src={introduction_image_url}>"
        );

        let template = MeneleTemplate{
            main_image: &main_image_element,
            einleitung_title: &self.einleitung.introduction_title,
            einleitung_image: &introduction_image_element,
            sections: &self.dynamic_sections.
                iter()
                .map(|section| section.to_html())
                .collect::<Vec<_>>()
                .join("\n")
        };

        template.render().unwrap()

    }

}