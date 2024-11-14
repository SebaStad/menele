use yew::prelude::*;

use crate::styling::headerfooterstyle::HeaderFooterStyle;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <HeaderFooterStyle>
            <div class="image-header-footer">
                <img src="https://www.medius-fitness.de/wp-content/uploads/2022/02/Logo-rot-_medius_-2021-weiss.png"/>
            </div>
            <div class="news-text">
                <p>{"medius news"}</p>
            </div>
        </HeaderFooterStyle>
    }
}