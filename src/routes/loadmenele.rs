use std::fmt::Debug;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::testmod::MainPageRoute;
use crate::styling::centered_container::CenteredContainer;

use gloo_file::callbacks::FileReader;
use gloo_file::File;
use gloo::utils::format::JsValueSerdeExt;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};


#[function_component(LoadMenele)]
pub fn loadmenele() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&MainPageRoute::Home));

    // let onclick_read_newspaper: Callback<MouseEvent> = Callback::from(
    //     move |_| navigator.push(&MainPageRoute::NewMenele)
    // );
    html! {
        <div>
            <CenteredContainer>
                <h1>{ "Load old Newsletter" }</h1>
                <button {onclick}>{ "Go Home" }</button>
                <button> { "Read Newspaper"} </button>
            </CenteredContainer>
        </div>
    }
}


// #[function_component(FileUpload)]
// fn file_upload() -> Html {
//     let file_content = use_state(|| None::<String>);

//     let on_file_change = {
//         let file_content = file_content.clone();
//         Callback::from(move |event: Event| {
//             let input: HtmlInputElement = event.target().unwrap().unchecked_into();
//             if let Some(file) = input.files().and_then(|files| files.get(0)) {
//                 let file = File::from(file);
//                 let file_content = file_content.clone();

//                 println!({"{:?}", *file_content.unwrap()});
//                 let reader: FileReader;
//                 let task = reader.from(&file, move |result| {
//                     if let Ok(content) = result {
//                         file_content.set(Some(content));
//                     }
//                 });

//                 // Drop task immediately if you don't need to keep track of it
//                 drop(task);
//             }
//         })
//     };

//     html! {
//         <div>
//             <input type="file" accept=".html" onchange={on_file_change} />
//             {
//                 if let Some(content) = (*file_content).clone() {
//                     html! { <pre>{ content }</pre> }
//                 } else {
//                     html! { <p>{ "No file selected." }</p> }
//                 }
//             }
//         </div>
//     }
// }