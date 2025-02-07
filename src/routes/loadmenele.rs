use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::testmod::MainPageRoute;
use crate::reducers::appstate::AppState;
use crate::reducers::introductionstate::IntroductionAction;
use crate::reducers::sectionstate::SectionAction;
use crate::reducers::windowsize::WindowSizeState;
use crate::styling::centered_container::CenteredContainer;

use gloo::utils::document;
use gloo_console::log;
use gloo_file::callbacks::read_as_text;
use gloo_file::{Blob, File};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use select::document::Document;
use select::predicate::{Class, Name};

#[function_component(LoadMenele)]
pub fn loadmenele() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |route: &MainPageRoute| navigator.push(route));

    // let onclick_read_newspaper: Callback<MouseEvent> = Callback::from(
    //     move |_| navigator.push(&MainPageRoute::NewMenele)
    // );
    html! {
        <div>
            <CenteredContainer>
                <h1>{ "Html-Newsletter einlesen" }</h1>
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&MainPageRoute::Home)
                }>{ "Hauptseite" }</button>
                <FileUpload/>
            </CenteredContainer>
        </div>
    }
}

#[function_component(FileUpload)]
fn file_upload() -> Html {
    let file_content = use_state(|| String::new());
    let reader_handle = use_state(|| None);

    let window_size_state = use_reducer(|| WindowSizeState {
        is_small_window: false,
    });

    let navigator = use_navigator().unwrap();
    let change_szene = Callback::from(move |route: &MainPageRoute| navigator.push(route));

    let appstate = use_context::<AppState>().expect("AppState context not found");

    // yeah, idk if all this boilerplate is necessary...
    let section_state = &appstate.section_state;
    let introduction_state = &appstate.introduction_state;

    let introductionstate_mainimage = introduction_state.clone();
    let introduction_state_intro_title = introduction_state.clone();
    let introductionstate_intro_image = introduction_state.clone();

    let section_title_state = section_state.clone();
    let section_text_state = section_state.clone();
    let section_image_state = section_state.clone();
    let section_button_state = section_state.clone();

    let change_szene = change_szene.clone();

    let on_file_change = {
        let file_content = file_content.clone();
        let section_state = section_state.clone();

        let introductionstate_mainimage = introductionstate_mainimage.clone();
        let introduction_state_intro_title = introduction_state_intro_title.clone();
        let introductionstate_intro_image = introductionstate_intro_image.clone();

        let section_title_state = section_title_state.clone();
        let section_text_state = section_text_state.clone();
        let section_image_state = section_image_state.clone();
        let section_button_state = section_button_state.clone();

        let window_size_state = window_size_state.clone();

        let change_szene = change_szene.clone();

        Callback::from(move |event: Event| {
            let input: HtmlInputElement =
                event.target().unwrap().unchecked_into::<HtmlInputElement>();

            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let file_content = file_content.clone();
                    let section_state = section_state.clone();

                    let introductionstate_mainimage = introductionstate_mainimage.clone();
                    let introduction_state_intro_title = introduction_state_intro_title.clone();
                    let introductionstate_intro_image = introductionstate_intro_image.clone();

                    let section_title_state = section_title_state.clone();
                    let section_text_state = section_text_state.clone();
                    let section_image_state = section_image_state.clone();
                    let section_button_state = section_button_state.clone();

                    let window_size_state = window_size_state.clone();

                    let change_szene = change_szene.clone();

                    let file: Blob = File::from(file).into();
                    // log!("Selected file: {:?}", file.clone());
                    let _reader = read_as_text(&file, move |result| {
                        match result {
                            Ok(text) => {
                                // log!("File content: {}", text.clone());
                                file_content.set(text.clone());

                                let document = Document::from(text.as_str());

                                let main_image_div = document
                                    .find(Class("main-image"))
                                    .next()
                                    .expect("No Introduction found")
                                    .find(Name("img"))
                                    .next()
                                    .expect("No Image tag found")
                                    .attr("src")
                                    .expect("No src tag found");

                                // let einleitung_header = document.find(Class("einleitung-header")).next().unwrap().text();
                                let einleitung_text = document
                                    .find(Class("einleitung-text"))
                                    .next()
                                    .unwrap()
                                    .text();
                                let mut image_artikel_rechts =
                                    document.find(Class("image-artikel-rechts"));

                                let introduction_image = image_artikel_rechts
                                    .next()
                                    .expect("No Introduction found")
                                    .find(Name("img"))
                                    .next()
                                    .expect("No Image tag found")
                                    .attr("src")
                                    .expect("No src tag found");

                                introductionstate_mainimage.dispatch(
                                    IntroductionAction::UpdateMainImage {
                                        url: main_image_div.to_string(),
                                    },
                                );
                                introduction_state_intro_title.dispatch(
                                    IntroductionAction::UpdateTitle {
                                        text: einleitung_text.to_string(),
                                    },
                                );
                                introductionstate_intro_image.dispatch(
                                    IntroductionAction::UpdateIntroductionImage {
                                        url: introduction_image.to_string(),
                                    },
                                );

                                let mut all_texts = document.find(Class("text"));

                                let _ = all_texts.next().expect("First Chapter");

                                let all_manual_chapters = all_texts.take(1000);
                                let mut id: usize = 0;
                                for chapter in all_manual_chapters {
                                    section_state.dispatch(SectionAction::AddSection {
                                        window_size: window_size_state.clone(),
                                    });

                                    let search_rechts =
                                        chapter.find(Class("image-artikel-rechts")).next();
                                    let search_links =
                                        chapter.find(Class("image-artikel-links")).next();

                                    let chapter_title = chapter
                                        .find(Class("sonstige-header"))
                                        .next()
                                        .expect("Should have title")
                                        .text();

                                    section_title_state.dispatch(
                                        SectionAction::UpdateChapterTitle {
                                            id,
                                            text: chapter_title.to_string(),
                                        },
                                    );
                                    if Option::is_some(&search_rechts) {
                                        // image rechts -> text links -> kapitel 2 text
                                        let chapter_text = chapter
                                            .find(Class("kapitel-zwei-text"))
                                            .next()
                                            .expect("msg")
                                            .text();

                                        let chapter_image = chapter
                                            .find(Class("image-artikel-rechts"))
                                            .next()
                                            .expect("No Introduction found")
                                            .find(Name("img"))
                                            .next()
                                            .expect("No Image tag found")
                                            .attr("src")
                                            .expect("No src tag found");

                                        section_text_state.dispatch(SectionAction::UpdateText {
                                            id,
                                            text: chapter_text.to_string(),
                                        });
                                        section_image_state.dispatch(SectionAction::UpdateImage {
                                            id,
                                            text: chapter_image.to_string(),
                                        });
                                    } else if Option::is_some(&search_links) {
                                        let chapter_text = chapter
                                            .find(Class("kapitel-eins-text"))
                                            .next()
                                            .expect("msg")
                                            .text();

                                        let chapter_image = chapter
                                            .find(Class("image-artikel-links"))
                                            .next()
                                            .expect("No Introduction found")
                                            .find(Name("img"))
                                            .next()
                                            .expect("No Image tag found")
                                            .attr("src")
                                            .expect("No src tag found");

                                        section_text_state.dispatch(SectionAction::UpdateText {
                                            id,
                                            text: chapter_text.to_string(),
                                        });
                                        section_image_state.dispatch(SectionAction::UpdateImage {
                                            id,
                                            text: chapter_image.to_string(),
                                        });
                                    }
                                    let search_button = chapter.find(Class("link-button")).next();
                                    if Option::is_some(&search_button) {
                                        let button_url = search_button
                                            .expect("No Introduction found")
                                            .find(Name("link"))
                                            .next()
                                            .expect("No Image tag found")
                                            .attr("href")
                                            .expect("No src tag found");
                                        section_button_state.dispatch(
                                            SectionAction::UpdateLinkButton {
                                                id,
                                                text: button_url.to_string(),
                                            },
                                        );
                                    }
                                    id = id + 1;
                                }
                            }
                            Err(_err) => {
                                log!("Error reading file:");
                            }
                        }
                        change_szene.emit(&MainPageRoute::NewMenele);
                    });
                    reader_handle.set(Some(_reader));
                }
            }
        })
    };

    let on_button_click = Callback::from(move |_| {
        if let Some(input) = document().get_element_by_id("file-upload") {
            input.unchecked_into::<HtmlInputElement>().click();
        }
    });

    html! {
        <div>
            <input id="file-upload" type="file" accept=".html" style="display: none;" onchange={on_file_change} />
            <button onclick={on_button_click}>{"Html-Newsletter auswählen"}</button>
        </div>
    }
}
