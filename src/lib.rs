use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
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
pub struct MyState {
    pub pool: PgPool,
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
    match sqlx::query_as::<_, Article>(
        "INSERT INTO articles (title, notebook, tags, content) VALUES ($1, $2, $3, $4) RETURNING id, title, notebook, tags, content"
    )
    .bind(&data.title)
    .bind(&data.notebook)
    .bind(&data.tags) // Assuming tags are stored as a single string, adjust accordingly
    .bind(&data.content)
    .fetch_one(&state.pool)
    .await
    {
        Ok(article) => Ok((StatusCode::CREATED, Json(article))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub fn run(state: MyState) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/list", get(list_articles))
        .route("/add", post(add_article))
        .route("/articles/:id", get(get_article))
        .with_state(state)
}
