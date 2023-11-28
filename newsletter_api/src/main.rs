use newsletter_api::generate_routes;
use shuttle_runtime::CustomError;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres()] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;
    let router = generate_routes(pool);
    Ok(router.into())
}
