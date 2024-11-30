use crate::templates::sectionleft_template::SectionleftTemplate;
use crate::templates::sectionright_template::SectionrightTemplate;
use askama::Template;

pub enum SectionTemplate<'a> {
    Left(SectionleftTemplate<'a>),
    Right(SectionrightTemplate<'a>),
}

impl<'a> SectionTemplate<'a> {
    pub fn render(&self) -> Result<String, askama::Error> {
        match self {
            SectionTemplate::Left(template) => template.render(),
            SectionTemplate::Right(template) => template.render(),
        }
    }
}
