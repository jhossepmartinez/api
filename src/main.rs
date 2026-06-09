use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    const PORT: u16 = 4000;
    let app = Router::new().route("/", get(|| async { "Hello, World" }));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", PORT))
        .await
        .unwrap();

    println!("Listening at port: {}!", PORT);
    axum::serve(listener, app).await.unwrap();
}
