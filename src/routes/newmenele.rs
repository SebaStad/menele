use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::testmod::MainPageRoute;
use crate::meneleparts::header::Header;
use crate::meneleparts::newsletter::NewsLetter;

#[function_component(NewMenele)]
pub fn newmenele() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&MainPageRoute::Home));
    html! {
        <div>
            <h1>{ "Create new Newspaper" }</h1>
            <button {onclick}>{ "Go Home" }</button>
            <br/>
            <NewsLetter/>
        </div>
    }
}



enum MeneleSection {
    left(MeneleSectionLeft),
    right(MeneleSectionRight)
}

struct MeneleSectionLeft {
}

struct MeneleSectionRight {
}


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