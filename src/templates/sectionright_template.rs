use askama::Template;

#[derive(Template)]
#[template(path = "sectionright_template.html", escape = "none")]

pub struct SectionrightTemplate<'a> {
    pub headline: &'a str,
    pub text: &'a str,
    pub image: &'a str,
    pub url_button: &'a str,
}

