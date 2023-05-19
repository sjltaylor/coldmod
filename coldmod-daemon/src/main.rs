#[tokio::main]
async fn main() {
    let (sender, receiver) = async_channel::bounded(65536);
    let triage = tokio::spawn(async { coldmod_daemon::triage::server(receiver).await });
    let tracing = tokio::spawn(async { coldmod_daemon::tracing::server(sender).await });

    match tokio::join!(triage, tracing) {
        (Ok(_), Ok(_)) => println!("all servers exited"),
        (Err(e), _) => println!("triage server exited with an error: {}", e),
        (_, Err(e)) => println!("tracing server exited with an error: {}", e),
    };
}
