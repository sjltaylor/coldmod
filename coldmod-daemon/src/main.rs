#[tokio::main]
async fn main() {
    let (sender, receiver) = async_channel::bounded(65536);
    let web = tokio::spawn(async { coldmod_daemon::web::server(receiver).await });
    let tracing = tokio::spawn(async { coldmod_daemon::tracing::server(sender).await });

    match tokio::join!(web, tracing) {
        (Ok(_), Ok(_)) => println!("all servers exited"),
        (Err(e), _) => println!("web server exited with an error: {}", e),
        (_, Err(e)) => println!("tracing server exited with an error: {}", e),
    };
}
