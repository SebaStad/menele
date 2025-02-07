use crate::meneleparts::headerimage::HeaderImage;
use crate::styling::headerfooterstyle::HeaderFooterStyle;
use crate::styling::styles::{CssClass, StyleConfig};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct FooterProps {
    pub is_small_window: bool,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let FooterProps {
        is_small_window, ..
    } = props;

    // let style_lookup = StyleConfig::new();
    // let style_string = style_lookup.get_style(
    //     CssClass::NewsText,
    //     *is_small_window
    // ).unwrap().clone();

    let style_lookup = StyleConfig::new();
    let style_string = style_lookup
        .get_style(CssClass::Links, *is_small_window)
        .unwrap()
        .clone();

    html! {
        <HeaderFooterStyle is_small_window={*is_small_window}>
            <HeaderImage is_small_window={*is_small_window}/>
            <div class = "links" style = {style_string}>
            <p>
                <a href="https://www.facebook.com/mediusTegernsee/" title="Besuchen Sie uns auf Facebook!" target="_blank">
                <img src="https://www.mail-signatures.com/wp-content/uploads/2014/08/Facebook.png" width="30" height="30"/>
                </a>
                <a href="https://www.instagram.com/medius_medizinischefitness/?hl=de" id="logo-insta" title="Besuchen Sie uns auf Instagram!" target="_blank">
                <img src="https://cdn.exclaimer.com/Handbook%20Images/instagram-icon_square_64x64.png?_ga=3D2.47096743.1462399753.1646042707-777455659.1646042707" width="30" height="30"/>
                </a>
                <a href="https://www.medius-fitness.de" target="_blank">
                {"www.medius-fitness.de"}
                </a>
            </p>
            </div>
        </HeaderFooterStyle>
    }
}
