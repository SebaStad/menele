use yew::prelude::*;
use crate::styling::styles::{StyleConfig, CssClass};

#[derive(Clone, PartialEq, Properties)]
pub struct ImpressumProps {
    pub is_small_window: bool
}

#[function_component(Impressum)]
pub fn mpressum(props: &ImpressumProps) -> Html {
    let ImpressumProps {is_small_window, ..} = props;

    let style_lookup = StyleConfig::new();
    let style_string_header = style_lookup
        .get_style(
            CssClass::Impressum,
            is_small_window.clone()
        )
        .unwrap()
        .clone();

    html! {
        <div class="impressum" style = {style_string_header}>
            <div class="sonstige-header">
                <p>{"IMPRESSUM"}</p>
            </div>
            <p>
                {"medius Fitness wird vertreten durch:"}
                <br/> 
                {"medius Schliersee GmbH & Co. KG"}
                <br/> 
                {"Perfallstrasse 4"}
                <br/> 
                {"83727 Schliersee"}
                <br/> 
                <br/> 
                {"Kontakt:"}
                <br/>
                {"Telefon 08026 – 92 93 841"}
                <br/> 
                {"Telefax 08026 – 92 93 842"}
                <br/> 
                {"E-Mail: "}
                <a href="mailto:info@medius-schliersee.de" class="email">
                    {"info@medius-schliersee.de"}
                </a>
                <br/>
                <br/>
                {"Unser vollständiges Impressum finden Sie"}
                <a href="https://www.medius-fitness.de/impressum/" target="_blank">
                    {"hier."}
                </a>
                <br/>
                <br/> 
            {"${footer.unsubscribeLink}"}</p>
        </div>

    }
}