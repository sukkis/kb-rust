use sqlx::PgPool;
use shuttle_runtime::CustomError;
use kbrust::{run, MyState}; // Import the run function and MyState struct from the kbrust crate

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let state = MyState { pool };

    let router = run(state);

    Ok(router.into())
}
