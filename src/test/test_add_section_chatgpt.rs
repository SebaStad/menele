use yew::{function_component, html, Html, Properties, use_state, Callback};
// use yew::{function_component, html, use_state, Html};
// use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub content: String,
    pub is_left: bool,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <div class={if props.is_left { "section-left" } else { "section-right" }}>
            { &props.content }
        </div>
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct SectionData {
    pub content: String,
    pub is_left: bool,
}

#[derive(Properties, PartialEq)]
pub struct SectionsProps {
    pub sections: Vec<SectionData>,
}

#[function_component(Sections)]
pub fn sections(props: &SectionsProps) -> Html {
    html! {
        <div class="sections-container">
            {
                for props.sections.iter().map(|section| {
                    html! {
                        <Section content={section.content.clone()} is_left={section.is_left} />
                    }
                })
            }
        </div>
    }
}


// use crate::components::sections::{Sections, SectionData};

#[function_component(AddSection)]
pub fn add_section() -> Html {
    let sections = use_state(|| vec![]);

    let add_section = {
        let sections = sections.clone();
        Callback::from(move |_| {
            // Alternate between left and right sections
            let mut new_sections = (*sections).clone();
            let is_left = new_sections.len() % 2 == 0; // Alternate based on count
            new_sections.push(SectionData {
                content: format!("New Section {}", new_sections.len() + 1),
                is_left,
            });
            sections.set(new_sections);
        })
    };

    html! {
        <div>
            <button onclick={add_section}>{ "Add Section" }</button>
            <Sections sections={(*sections).clone()} />
        </div>
    }
}

