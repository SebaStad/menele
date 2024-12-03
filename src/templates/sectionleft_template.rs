use askama::Template;

#[derive(Template)]
#[template(path = "sectionleft_template.html", escape = "none")]

pub struct SectionleftTemplate<'a> {
    pub headline: &'a str,
    pub text: &'a str,
    pub image: &'a str,
    pub url_button: &'a str,
}
