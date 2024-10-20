use std::net::SocketAddr;

// use hyper::client::conn;
use newsletter_api::{configuration::get_configuration, startup::generate_routes};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .map_err(CustomError::new)?;
    let router = generate_routes(connection_pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    Ok(())
}
