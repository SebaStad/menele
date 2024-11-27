use gloo::net::eventsource::State;
use yew::{function_component, html, use_reducer, Callback, Html, Properties, Reducible, TargetCast, UseReducerHandle, UseStateHandle};
use gloo_console::log;
use crate::routes::subroutes::newmenele_right::SectionData;
// use crate::routes::subroutes::windowsizestate::WindowSizeState;


#[derive(Clone, Debug, PartialEq)]
pub struct SectionRaw {
    pub id: usize,
    pub text: String,
    pub image_url: String,
    pub is_left: bool,
    pub is_small_window: UseStateHandle<bool>
}

impl SectionRaw {
    pub fn to_section_data(&self) -> SectionData {
        SectionData {
            content: self.text.clone(),
            image_url: self.image_url.clone(),
            is_left: self.is_left,
            is_small_window: self.is_small_window.clone()
        }
    }
}

pub fn convert_sections(raw_sections: &Vec<SectionRaw>) -> Vec<SectionData> {
    raw_sections.iter().map(|section| section.to_section_data()).collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct SectionState {
    pub sections: Vec<SectionRaw>,
}

pub enum SectionAction {
    AddSection {is_small_window: UseStateHandle<bool>},
    RemoveSection,
    UpdateText { id: usize, text: String},
    UpdateImage {id: usize, text: String}
}

impl Reducible for SectionState {
    type Action = SectionAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            SectionAction::AddSection {is_small_window} => {
                let mut sections = self.sections.clone();
                log!("Hello", sections.len() % 2 == 0);
                log!("asdasd", *is_small_window.clone());
                sections.push(SectionRaw {
                    id: sections.len(),
                    text: String::new(),
                    image_url: String::new(),
                    is_left: sections.len() % 2 == 0,
                    is_small_window: is_small_window
                });
                Self { sections }.into()
            }
            SectionAction::RemoveSection => {
                let mut sections = self.sections.clone();
                sections.pop();
                Self { sections }.into()
            }
            SectionAction::UpdateText { id, text } => {
                let mut sections = self.sections.clone();
                if let Some(section) = sections.iter_mut().find(|s| s.id == id) {
                    section.text = text;
                }
                Self { sections }.into()
            }
            SectionAction::UpdateImage { id, text } => {
                let mut sections = self.sections.clone();
                if let Some(section) = sections.iter_mut().find(|s| s.id == id) {
                    section.image_url = text;
                }
                Self { sections }.into()
            }

        }
    }
}