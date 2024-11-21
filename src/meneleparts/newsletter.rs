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

use crate::routes::newmenele::{SectionData, Sections};

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





// use yew::{function_component, html, Html, Properties, use_state, Callback};
// // use yew::{function_component, html, use_state, Html};
// // use yew::{function_component, html, Callback, Html, Properties};

// #[derive(Properties, PartialEq)]
// pub struct SectionProps {
//     pub text: String,
//     pub image_url: String,
//     pub is_left: bool,
// }

// #[function_component(Section)]
// pub fn section(props: &SectionProps) -> Html {
//     let SectionProps {text, image_url, is_left} = props;
//     html! {
//         <div class={if *is_left { "section-left" } else { "section-right" }}>
//             { &text }
//             <img src={image_url.clone()}/>
//         </div>
//     }
// }


// #[derive(Clone, Debug, PartialEq)]
// pub struct SectionData {
//     pub content: String,
//     pub image_url: String,
//     pub is_left: bool,
// }

// #[derive(Properties, PartialEq)]
// pub struct SectionsProps {
//     pub sections: Vec<SectionData>,
// }

// #[function_component(Sections)]
// pub fn sections(props: &SectionsProps) -> Html {
//     let SectionsProps {sections} = props;
//     html! {
//         <div class="sections-container">
//             {
//                 for sections.iter().map(|section| {
//                     html! {
//                         <Section text={section.content.clone()} is_left={section.is_left} image_url = {section.image_url.clone()} />
//                     }
//                 })
//             }
//         </div>
//     }
// }


// // use crate::components::sections::{Sections, SectionData};

// #[function_component(AddSection)]
// pub fn add_section() -> Html {
//     let sections = use_state(|| vec![]);

//     let add_section = {
//         let sections = sections.clone();
//         Callback::from(move |_| {
//             // Alternate between left and right sections
//             let mut new_sections = (*sections).clone();
//             let is_left = new_sections.len() % 2 == 0; // Alternate based on count
//             new_sections.push(SectionData {
//                 content: format!("New Section {}", new_sections.len() + 1),
//                 is_left,
//                 image_url: String::from("https://www.medius-fitness.de/wp-content/uploads/2021/06/medius-Logo-550x120-DSV.png")
//             });
//             sections.set(new_sections);
//         })
//     };

//     html! {
//         <div>
//             <button onclick={add_section}>{ "Add Section" }</button>
//             <Sections sections={(*sections).clone()} />
//         </div>
//     }
// }

