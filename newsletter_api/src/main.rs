use newsletter_api::generate_routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = generate_routes();
    Ok(app.into())
}
