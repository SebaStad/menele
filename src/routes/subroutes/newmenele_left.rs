use yew::{function_component, html, Callback, Html, InputEvent, Properties};
// use yew::{function_component, html, use_state, Html};
// use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InputSectionProps {
    pub text: String,
    pub image_url: String,
    pub on_input: Callback<InputEvent>,
}

#[function_component(InputSection)]
pub fn input_section(props: &InputSectionProps) -> Html {
    let InputSectionProps {
        text,
        image_url,
        on_input,
    } = props;
    html! {
        <div>
            <input type="text"  value={text.clone()} oninput={on_input} />
            <img src={image_url.clone()}/>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputSectionData {
    pub content: String,
    pub image_url: String,
    pub on_input: Callback<InputEvent>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputSectionsProps {
    pub input_sections: Vec<InputSectionData>,
}

#[function_component(InputSections)]
pub fn input_sections(props: &InputSectionsProps) -> Html {
    let InputSectionsProps { input_sections } = props;
    html! {
        <div class="input-sections-container">
            {
                for input_sections.iter().map(|section| {
                    html! {
                        <InputSection text={section.content.clone()} image_url = {section.image_url.clone()} on_input = {section.on_input.clone()}/>
                    }
                })
            }
        </div>
    }
}
