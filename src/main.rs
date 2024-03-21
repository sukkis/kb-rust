use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use shuttle_runtime::CustomError;
use sqlx::{FromRow, PgPool};

/// Represents a new article, not yet persisted.
#[derive(Deserialize)]
struct ArticleNew {
    title: String,
    notebook: String, // Collection of articles.
    tags: Vec<String>,
    content: String,
}

/// Represents an article in database.
#[derive(Serialize, FromRow)]
struct Article {
    id: i32,
    title: String,
    notebook: String, // Collection of articles.
    tags: Vec<String>,
    content: String,
}

/// Persistence with Postgres.
#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

/// List titles of all articles in knowledge base.
async fn list_articles() -> &'static str {
    "Article list not yet implemented."
}

/// Handler to retrieve an article by ID
async fn get_article(
    Path(id): Path<i32>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Article>(
        "SELECT id, title, notebook, tags, content FROM articles WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(article) => Ok((StatusCode::OK, Json(article))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

/// Handler to add a new article.
async fn add_article(
    State(state): State<MyState>,
    Json(data): Json<ArticleNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Convert the tags Vec<String> into a single string representation
    // This step needs to be adjusted if you use a different method to store tags in the database
    //   let tags_joined = data.tags.join(","); // Simple join, consider JSON or array if supported

    match sqlx::query_as::<_, Article>("INSERT INTO articles (title, notebook, tags, content) VALUES ($1, $2, $3, $4) RETURNING id, title, notebook, tags, content")
        .bind(&data.title)
        .bind(&data.notebook)
        .bind(&data.tags) // Assuming tags are stored as a single string, adjust accordingly
        .bind(&data.content)
        .fetch_one(&state.pool)
        .await
    {
        Ok(article) => Ok((StatusCode::CREATED, Json(article))), // Ensure the variable name matches here
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let state = MyState { pool };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/list", get(list_articles))
        .route("/add", post(add_article))
        .route("/articles/:id", get(get_article))
        .with_state(state);

    Ok(router.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;
    use serde_json::json;

    #[tokio::test]
    async fn test_add_article() {
        let client = reqwest::Client::new();
        let article = json!({
            "title": "Test Article",
            "notebook": "Test Notebook",
            "tags": ["test", "rust"],
            "content": "This is a test article."
        });

        let resp = client
            .post("http://127.0.0.1:8000/add")
            .json(&article)
            .send()
            .await
            .expect("Failed to send request");

        assert!(
            resp.status().is_success(),
            "Expected successful response, got {:?}",
            resp.status()
        );
    }
}
