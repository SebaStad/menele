use askama::Template;

#[derive(Template)]
#[template(path = "newsletter_template.html", escape = "none")]

pub struct MeneleTemplate<'a> {
    pub introduction_image: &'a str,
    pub sections: &'a str,
}

