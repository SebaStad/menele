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
            // <Impressum is_small_window={*is_small_window}/>
        </div>
    }
}

