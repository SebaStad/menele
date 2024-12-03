use yew::UseReducerHandle;
use crate::reducers::introductionstate::IntroductionState;
use crate::reducers::sectionstate::SectionState;

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub section_state: UseReducerHandle<SectionState>,
    pub introduction_state: UseReducerHandle<IntroductionState>,
}
