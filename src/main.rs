use axum::{response::Html, routing::post, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
            .route("/", get(handler))
            .route("/clicked", post(handlerpost));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
    // time. This method is relative to current `main.rs` file.
    Html(include_str!("../index.html"))
}

async fn handlerpost() -> Html<String> {
    Html(format!("
    <div>
        Hello, World I am the swap!
        <img src='https://fronty.com/static/uploads/code_tester.png'>
    </div>"))
}
