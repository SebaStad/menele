use yew::UseReducerHandle;
use crate::reducers::introductionstate::IntroductionState;
use crate::reducers::sectionstate::SectionState;
use crate::reducers::globaloptions::GlobalOptions;

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub section_state: UseReducerHandle<SectionState>,
    pub introduction_state: UseReducerHandle<IntroductionState>,
    pub start_left_state: UseReducerHandle<GlobalOptions>
}
