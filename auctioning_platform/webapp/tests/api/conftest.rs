use tokio::net::TcpListener;

pub struct Context {
    pub address: String,
}

pub async fn spawn_app() -> Context {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener,webapp::startup::create_app())
            .await
            .unwrap();
    });
    Context {
        address: format!("http://localhost:{}", addr.port()),
    }
}
