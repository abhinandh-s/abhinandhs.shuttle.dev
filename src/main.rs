use abhinandh_s::ArticleTemplate;
use askama::Template;
use axum::extract::Path;
use axum::response::Html;
use axum::{routing::get, Router};
use pulldown_cmark::{html::push_html, Parser};
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(index_handler))
        .route("/articles/{name}", get(article_handler))
        .nest_service("/static", ServeDir::new("static"));

    Ok(router.into())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    articles: Vec<String>,
}

async fn index_handler() -> Html<String> {
    let paths = std::fs::read_dir("articles").unwrap();
    let mut articles = Vec::new();

    for entry in paths.flatten() {
        if let Some(name) = entry.path().file_stem() {
            articles.push(name.to_string_lossy().to_string());
        }
    }

    let template = IndexTemplate { articles };
    Html(template.render().unwrap())
}

async fn article_handler(Path(name): Path<String>) -> impl axum::response::IntoResponse {
    let file_path = format!("articles/{}.md", name);
    let md = std::fs::read_to_string(&file_path).unwrap_or_else(|_| "# Not Found".to_owned());
    let parser = Parser::new(&md);
    let mut content = String::new();
    push_html(&mut content, parser);
    let template = ArticleTemplate::new(name, content);
    Html(template.render().unwrap())
}
