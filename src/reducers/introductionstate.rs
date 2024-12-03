// use yew::{function_component, html, use_reducer, Callback, Html, Properties, Reducible, TargetCast, UseReducerHandle, UseStateHandle};
use yew::Reducible;
// use gloo_console::log;


#[derive(Clone, Debug, PartialEq)]
pub struct IntroductionState {
    pub main_image_url: String,
    pub introduction_title: String,
    pub introduction_image_url: String
}

pub enum IntroductionAction {
    UpdateMainImage { url: String},
    UpdateTitle {text: String},
    UpdateIntroductionImage {url: String}
}

impl Reducible for IntroductionState {
    type Action = IntroductionAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            IntroductionAction::UpdateMainImage { url } => {
                Self {
                    main_image_url: url.clone(),
                    ..(*self).clone() // Retain the existing fields
                }
                .into()
            }
            IntroductionAction::UpdateTitle { text } => {
                Self {
                    introduction_title: text.clone(),
                    ..(*self).clone()
                }
                .into()
            }
            IntroductionAction::UpdateIntroductionImage { url } => {
                Self {
                    introduction_image_url: url.clone(),
                    ..(*self).clone()
                }
                .into()
            }

        }
    }
}