#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    let port = 8000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("Listening on: http://localhost:{port}");
    axum::serve(listener, stabil_api::router()).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    println!("Please use the server feature to start the server.");
}
