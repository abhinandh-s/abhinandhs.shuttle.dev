use askama::Template;

#[derive(Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate {
    title: String,
    content: String,
}

impl ArticleTemplate {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
