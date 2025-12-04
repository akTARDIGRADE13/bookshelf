use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // ルーター定義
    let app = Router::new().route("/", get(root));

    // アドレスを決める
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // TCPリスナーをバインド
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    println!("Listening on http://{}", addr);

    // サーバー起動
    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn root() -> &'static str {
    "Hello, Bookshelf!"
}

