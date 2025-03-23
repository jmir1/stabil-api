#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    let port = 8000;
    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await {
        Ok(listener) => listener,
        Err(_) => {
            println!("Failed to bind to port {port}");
            return;
        }
    };
    println!("Listening on: http://localhost:{port}");
    axum::serve(listener, stabil_api::router())
        .await
        .unwrap_or_else(|e| println!("Failed to start server: {e}"));
}

#[cfg(not(feature = "server"))]
fn main() {
    println!("Please use the server feature to start the server.");
}
