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

#[derive(PartialEq, Properties)]
pub struct NewsLetterProps {
    pub is_small_window: bool,
}

#[function_component]
pub fn NewsLetter(props: &NewsLetterProps) -> Html {
    let NewsLetterProps {is_small_window} = props;
    html! {
        <div>
            <Header is_small_window={*is_small_window}/>
            <MainImage is_small_window={*is_small_window}/>
            <Einleitung is_small_window={*is_small_window}/>
            <TrennerMitte/>
            // <Sectionlist/>
            // <Footer/>
            // <Impressum/>
        </div>
    }
}

// #[derive(Properties, PartialEq)]
// pub struct HeaderImageProps {
//     #[prop_or_default]
//     pub children: Children, // This allows the component to accept child elements
// }





