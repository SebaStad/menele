use gloo::net::eventsource::State;
use yew::{function_component, html, use_reducer, Callback, Html, Properties, Reducible, TargetCast, UseReducerHandle, UseStateHandle};
use gloo_console::log;
use crate::{reducers::windowsize::WindowSizeState, routes::subroutes::newmenele_right::SectionData};
// use crate::routes::subroutes::windowsizestate::WindowSizeState;


#[derive(Clone, Debug, PartialEq)]
pub struct SectionRaw {
    pub id: usize,
    pub chapter_title: String,
    pub text: String,
    pub image_url: String,
    pub is_left: bool,
    pub is_small_window: bool
}

impl SectionRaw {
    pub fn to_section_data(&self) -> SectionData {
        SectionData {
            chapter_title: self.chapter_title.clone(),
            content: self.text.clone(),
            image_url: self.image_url.clone(),
            is_left: self.is_left,
            is_small_window: self.is_small_window
        }
    }
}

pub fn convert_sections(raw_sections: &Vec<SectionRaw>) -> Vec<SectionData> {
    raw_sections.iter().map(|section| section.to_section_data()).collect()
}
