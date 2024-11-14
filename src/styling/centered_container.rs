use yew::prelude::*;

/// A reusable component that centers its child elements.
#[derive(Properties, PartialEq)]
pub struct CenteredContainerProps {
    #[prop_or_default]
    pub children: Children, // This allows the component to accept child elements
}

#[function_component(CenteredContainer)]
pub fn centered_container(props: &CenteredContainerProps) -> Html {
    html! {
        <div style="
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;">
            <div>
                { for props.children.iter() }
            </div>
        </div>
    }
}