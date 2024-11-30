use yew::prelude::*;
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

use crate::routes::subroutes::newmenele_right::{SectionData, Sections};

use crate::templates::menele_template::MeneleTemplate;
use askama::Template;

#[derive(PartialEq, Properties)]
pub struct NewsLetterProps {
    pub is_small_window: bool,
    pub dynamic_sections: Vec<SectionData>
}

#[function_component]
pub fn NewsLetter(props: &NewsLetterProps) -> Html {
    let NewsLetterProps {is_small_window, dynamic_sections} = props;
    html! {
        <div>
            <Header is_small_window={*is_small_window}/>
            <MainImage is_small_window={*is_small_window}/>
            <Einleitung is_small_window={*is_small_window}/>
            <TrennerMitte/>
            <Sections sections={(*dynamic_sections).clone()} />
            <Footer is_small_window={*is_small_window}/>
            <Impressum is_small_window={*is_small_window}/>
        </div>
    }
}

impl NewsLetterProps {

    pub fn to_html(&self) -> String {
        let template = MeneleTemplate{
            introduction_image: &String::from(
                "<img src=\"https://www.medius-fitness.de/wp-content/uploads/2022/02/2022_RiciRing.jpg\">"
            ),
            sections: &self.dynamic_sections.
                iter()
                .map(|section| section.to_html())
                .collect::<Vec<_>>()
                .join("\n")
        };

        template.render().unwrap()

    }

}