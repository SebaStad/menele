use askama::Template;

#[derive(Template)]
#[template(path = "button.html", escape = "none")]

pub struct ButtonTemplate<'a> {
    pub url_href: &'a str,
}

