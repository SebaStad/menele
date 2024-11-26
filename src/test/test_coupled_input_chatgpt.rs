use yew::{function_component, html, Callback, Html, Properties, Reducible, UseReducerHandle, use_reducer, TargetCast};

#[derive(Clone, Debug, PartialEq)]
struct Section {
    id: usize,
    text: String,
}

#[derive(Clone, Debug, PartialEq)]
struct AppState {
    sections: Vec<Section>,
}

enum AppAction {
    AddSection,
    RemoveSection,
    UpdateText { id: usize, text: String },
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AppAction::AddSection => {
                let mut sections = self.sections.clone();
                sections.push(Section {
                    id: sections.len(),
                    text: String::new(),
                });
                Self { sections }.into()
            }
            AppAction::RemoveSection => {
                let mut sections = self.sections.clone();
                sections.pop();
                Self {sections}.into()
            }
            AppAction::UpdateText { id, text } => {
                let mut sections = self.sections.clone();
                if let Some(section) = sections.iter_mut().find(|s| s.id == id) {
                    section.text = text;
                }
                Self { sections }.into()
            }
        }
    }
}

#[function_component(CoupledApp)]
pub fn coupled_app() -> Html {
    let state = use_reducer(|| AppState { sections: vec![] });

    let add_section = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(AppAction::AddSection);
        })
    };

    let remove_section = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(AppAction::RemoveSection);
        })
    };

    html! {
        <div style="display: flex; height: 100vh;">
            // Left-hand side: Input fields
            <div style="width: 50%; padding: 16px; border-right: 1px solid #ccc;">
                <button onclick={add_section}>{ "Add Section" }</button>
                <button onclick={remove_section}>{ "Remove Section" }</button>
                <div>
                    { for state.sections.iter().map(|section| {
                        let state = state.clone();
                        let id = section.id;
                        let oninput = Callback::from(move |e: yew::events::InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap() ;
                            state.dispatch(AppAction::UpdateText { id, text: input.value() });
                        });
                        html! {
                            <div style="margin-top: 16px;">
                                <input
                                    type="text"
                                    placeholder={format!("Section {}", id + 1)}
                                    value={section.text.clone()}
                                    oninput={oninput}
                                />
                            </div>
                        }
                    })}
                </div>
            </div>

            // Right-hand side: Display sections
            <div style="width: 50%; padding: 16px;">
                { for state.sections.iter().map(|section| {
                    html! {
                        <div
                            style="background: #e3e3e3; padding: 16px; margin-top: 16px; border-radius: 4px;">
                            { &section.text }
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
