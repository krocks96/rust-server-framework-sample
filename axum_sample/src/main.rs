#[tokio::main]
async fn main() {
    let app = axum_sample::apis::router();

    let listener = axum_sample::apis::listener("0.0.0.0", 8081);
    axum::serve(listener.await, app).await.unwrap();
}