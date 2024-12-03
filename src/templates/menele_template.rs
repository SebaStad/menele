use askama::Template;

#[derive(Template)]
#[template(path = "newsletter_template.html", escape = "none")]

pub struct MeneleTemplate<'a> {
    pub main_image: &'a str,
    pub einleitung_title: &'a str,
    pub einleitung_image: &'a str,
    pub sections: &'a str,
}

