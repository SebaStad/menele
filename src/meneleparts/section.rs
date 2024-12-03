// use crate::meneleparts::leftmenele::MeneleSectionLeft;
// use crate::meneleparts::rightmenele::MeneleSectionRight;
// use yew::prelude::*;


#[derive(PartialEq)]
pub struct ImageUrl {
    pub content: String
}


#[derive(PartialEq)]
pub struct SectionContent {
    pub text: String,
    pub image: ImageUrl
}


// #[derive(PartialEq)]
// pub enum MeneleSection {
//     Left(SectionContent),
//     Right(SectionContent)
// }

// impl MeneleSection {
//     pub fn provide_html(self) -> Html{
//         html!{
//             <div>
//             </div>
//         }
        
//     }
// }