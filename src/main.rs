#![allow(unused)]

use std::fmt::Display;

use askama::Template;
use axum::extract::Path;
use axum::response::Html;
use axum::Error;
use axum::{routing::get, Router};
use pulldown_cmark::{html::push_html, Parser};
use serde::{self, Deserialize};
use tracing::info;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(index))
        .route("/blog/{slug}", get(blog));

    Ok(router.into())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<String>, // slugs or titles
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    title: String,
    content: String,
}

async fn index() -> impl axum::response::IntoResponse {
    use std::env;

    tracing::info!("printing CWD..." );
    let cwd = env::current_dir().unwrap();
    tracing::info!("CWD: {}", cwd.display());

    let posts = std::fs::read_dir("posts")
        .unwrap()
        .filter_map(|e| {
            let path = e.ok()?.path();
            path.file_stem().map(|s| s.to_string_lossy().to_string())
        })
        .collect::<Vec<_>>();

    let rendered = IndexTemplate { posts }.render().unwrap();
    Html(rendered) // ✅ This tells Axum to send HTML content
}

async fn blog(Path(slug): Path<String>) -> impl axum::response::IntoResponse {
    let path = format!("posts/{}.md", slug);
    let markdown = std::fs::read_to_string(&path).unwrap_or("Post not found".into());

    let parser = pulldown_cmark::Parser::new(&markdown);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let post = PostTemplate {
        title: slug,
        content: html_output,
    };

    Html(post.render().unwrap())
}
