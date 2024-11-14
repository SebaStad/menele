use yew::prelude::*;

/// A reusable component that centers its child elements.
#[derive(Properties, PartialEq)]
pub struct HeaderFooterStyleProps {
    #[prop_or_default]
    pub children: Children, // This allows the component to accept child elements
}

#[function_component(HeaderFooterStyle)]
pub fn header_footer_style(props: &HeaderFooterStyleProps) -> Html {
    html! {
        <div style="
            background-color: #e30613;
            padding-top: 45px;
            color: white;
            height: 350px;
            width: 100%;
            margin: 0;
            font-family: \"Carlito\";"
        >
            <div>
                { for props.children.iter() }
            </div>
        </div>
    }
}