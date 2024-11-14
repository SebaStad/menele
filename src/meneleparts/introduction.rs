use yew::prelude::*;

use crate::styling::introductionstyle::IntroductionStyle;

#[function_component(IntroductionHeader)]
pub fn introduction_header() -> Html {
    html! {
        <>
            <div class="main-image">
                <img src="https://www.medius-fitness.de/wp-content/uploads/2022/02/2022_RiciRing.jpg"/>
            </div>
            <IntroductionStyle>
                <div class="einleitung-header">
                    <p>{"EINLEITUNG"}</p>
                </div>
            </IntroductionStyle>
        </>
    }
}