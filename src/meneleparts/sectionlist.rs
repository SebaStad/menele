use yew::prelude::*;
use yew_router::prelude::*;

use crate::meneleparts::section::MeneleSection;

// https://github.com/yewstack/yew/discussions/2400
enum Msg {
    AddOne,
    RemoveOne,
}

#[derive(PartialEq, Properties)]
pub struct MeneleSectionsPropsProps {}

#[function_component]
pub fn MeneleSectionsProps(props: &MeneleSectionsPropsProps) -> Html {
    let MeneleSectionsPropsProps {} = props;
    html! {
        <div></div>
    }
}


struct MeneleSections {
    menele_sections: Vec<MeneleSection>
}

impl Component for MeneleSections {
    type Message = Msg;
    type Properties = MeneleSectionsPropsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            menele_sections: Vec::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {

        }
    }

}