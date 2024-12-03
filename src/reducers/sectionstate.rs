use yew::{Reducible, UseReducerHandle};
// use gloo_console::log;

// use crate::meneleparts::newsletter::NewsLetterProps;
use crate::routes::subroutes::coupled_sections::SectionRaw;
use crate::reducers::windowsize::WindowSizeState;

#[derive(Clone, Debug, PartialEq)]
pub struct SectionState {
    pub sections: Vec<SectionRaw>
}

pub enum SectionAction {
    AddSection {window_size: UseReducerHandle<WindowSizeState>},
    RemoveSection,
    UpdateText { id: usize, text: String},
    UpdateImage {id: usize, text: String},
    UpdateChapterTitle {id: usize, text: String},
    UpdateWindowSize {window_size: UseReducerHandle<WindowSizeState>},
    UpdateLinkButton {id: usize, text: String},
    // CreateHtml {newsletter: NewsLetterProps}
}

impl Reducible for SectionState {
    type Action = SectionAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            SectionAction::AddSection {window_size} => {
                let mut sections = self.sections.clone();
                sections.push(SectionRaw {
                    id: sections.len(),
                    chapter_title: String::new(),
                    text: String::new(),
                    image_url: String::new(),
                    button_url: String::new(),
                    is_left: sections.len() % 2 == 1,
                    is_small_window: window_size.is_small_window.clone()
                });
                Self { sections, ..*self}.into()
            }
            SectionAction::RemoveSection => {
                let mut sections = self.sections.clone();
                sections.pop();
                Self { sections, ..*self}.into()
            }
            SectionAction::UpdateText { id, text } => {
                let mut sections = self.sections.clone();
                if let Some(section) = sections.iter_mut().find(|s| s.id == id) {
                    section.text = text;
                }
                Self { sections, ..*self}.into()
            }
            SectionAction::UpdateImage { id, text } => {
                let mut sections = self.sections.clone();
                if let Some(section) = sections.iter_mut().find(|s| s.id == id) {
                    section.image_url = text;
                }
                Self { sections, ..*self}.into()
            }
            SectionAction::UpdateChapterTitle { id, text } => {
                let mut sections = self.sections.clone();
                if let Some(section) = sections.iter_mut().find(|s| s.id == id) {
                    section.chapter_title = text;
                }
                Self { sections, ..*self}.into()
            }
            SectionAction::UpdateWindowSize { window_size } => {
                let mut sections = self.sections.clone();

                // log!("Sectionslength: {}", sections.len());
                for section in sections.iter_mut() {
                    section.is_small_window = window_size.is_small_window.clone();
                }
                Self { sections, ..*self}.into()
            }

            SectionAction::UpdateLinkButton { id, text }  => {
                let mut sections = self.sections.clone();
                if let Some(section) = sections.iter_mut().find(|s| s.id == id) {
                    section.button_url = text;
                }
                Self { sections, ..*self}.into()
            }

            // SectionAction::UpdateLinkButton { newsletter } => {
            //     let somestring =newsletter.to_html();

            //     println!("{:?}", somestring);
            //     Self { }.into()

            // }

        }
    }
}