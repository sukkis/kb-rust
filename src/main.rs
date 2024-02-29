use axum::{routing::get, Router};
use axum_test::TestServer;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn list_articles() -> &'static str {
    "Article list not yet implemented."

}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
	.route("/", get(hello_world))
        .route("/list", get(list_articles));

    Ok(router.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_list_articles_returns_something() {
	let app = Router::new().route("/list", get(list_articles));
	let server = TestServer::new(app).unwrap();
	let response = server.get("/list").await;
        let result = "Article list not yet implemented.";
        assert_eq!(response.text(), result);
    }
}
