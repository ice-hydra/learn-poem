mod errors;
mod handlers;

use poem::{get, listener::TcpListener, Route, Server};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", get(handlers::hello));
    let listener = TcpListener::bind("127.0.0.1:8080");
    Server::new(listener).await?.run(app).await
}
