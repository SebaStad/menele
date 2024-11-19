use yew::prelude::*;

use crate::meneleparts::section::MeneleSection;

// https://github.com/yewstack/yew/discussions/2400
enum Msg {
    AddOne,
    RemoveOne,
}

#[derive(PartialEq, Properties)]
pub struct MeneleSectionsProps {
    pub is_small_window: bool,
    pub sections: Vec<MeneleSection>,
    pub is_next_left: bool
}

// impl MeneleSectionsProps {

//     pub fn new() -> Self {
//         Self {
//             sections: Vec::new(),
//             is_next_left: true,
//             is_small_window: false
//         }
//     }

//     pub fn add_section(&mut self, new_section: MeneleSection) {
//         self.sections.push(new_section)
//     }
// }

struct MeneleSections ;

// struct MeneleSections {
//     menele_sections: Vec<MeneleSection>
// }

impl Component for MeneleSections {
    type Message = Msg;
    type Properties = MeneleSectionsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {

        }
    }

}