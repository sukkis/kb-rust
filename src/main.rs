use axum::{routing::get, routing::post, Router};
use serde::{Deserialize, Serialize};

/// Represents an article in knowledge base.
#[derive(Deserialize, Serialize)]
struct Article {
    title: String,
    notebook: String, // Collection of articles.
    tags: Vec<String>,
    content: String,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

/// List titles of all articles in knowledge base.
async fn list_articles() -> &'static str {
    "Article list not yet implemented."
}

/// Add article to knowledge base.
async fn create_article() -> &'static str {
    "Article created."
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/list", get(list_articles))
        .route("/add", post(create_article));

    Ok(router.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_list_articles_returns_something() {
        let app = Router::new().route("/list", get(list_articles));
        let server = TestServer::new(app).unwrap();
        let response = server.get("/list").await;
        let result = "Article list not yet implemented.";
        assert_eq!(response.text(), result);
    }

    #[tokio::test]
    async fn test_create_article() {
        let app = Router::new().route("/articles", post(create_article));
        let server = TestServer::new(app).unwrap();

        let article = json!({
            "title": "Test Article",
        "notebook": "Test notebook",
            "tags": ["test", "article"],
            "content": "This is a test article."
        })
        .to_string();

        let response = server
            .post("/articles")
            .json(&article)
            .content_type("application/json")
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
