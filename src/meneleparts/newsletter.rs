use yew::prelude::*;

use crate::meneleparts::header::Header;
use crate::meneleparts::introduction::IntroductionHeader;

#[derive(PartialEq, Properties)]
pub struct NewsLetterProps {}

#[function_component]
pub fn NewsLetter(props: &NewsLetterProps) -> Html {
    let NewsLetterProps {} = props;
    html! {
        <div>
            <Header/>
            <IntroductionHeader/>
            // <Einleitung/>
            // <Trenner/>
            // <Sectionlist/>
            // <Footer/>
            // <Impressum/>
        </div>
    }
}