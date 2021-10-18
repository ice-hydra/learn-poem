use poem::{get, listener::TcpListener, web::Path, Route, Server};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/hello/:name", get(hello));
    let addr = TcpListener::bind("127.0.0.1:8080");
    Server::new(addr).await?.run(app).await
}
#[poem::handler]
fn hello(Path(name): Path<String>) -> String {
    format!("你好：{}", name)
}
