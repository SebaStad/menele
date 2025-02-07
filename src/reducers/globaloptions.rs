use std::rc::Rc;
use yew::Reducible;
// use gloo_console::log;

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalOptions {
    pub chapters_start_left: bool,
}

pub enum GlobalOptionsActions {
    SwitchChaptersStartLeft,
}

impl Reducible for GlobalOptions {
    type Action = GlobalOptionsActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            GlobalOptionsActions::SwitchChaptersStartLeft => Rc::new(Self {
                chapters_start_left: !self.chapters_start_left,
            }),
        }
    }
}
