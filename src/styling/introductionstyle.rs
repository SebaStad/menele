use yew::prelude::*;

/// A reusable component that centers its child elements.
#[derive(Properties, PartialEq)]
pub struct IntroductionStyleProps {
    #[prop_or_default]
    pub children: Children, // This allows the component to accept child elements
}

#[function_component(IntroductionStyle)]
pub fn header_footer_style(props: &IntroductionStyleProps) -> Html {
    html! {
        <div style="
            font-size: 1.875em;
            text-align: center;"
        >
            <div>
                { for props.children.iter() }
            </div>
        </div>
    }
}