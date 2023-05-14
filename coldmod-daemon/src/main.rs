#[tokio::main]
async fn main() {
    let triage = tokio::spawn(async { coldmod_daemon::triage::server().await });
    let tracing = tokio::spawn(async { coldmod_daemon::tracing::server().await });

    match tokio::join!(triage, tracing) {
        (Ok(_), Ok(_)) => println!("all servers exited"),
        (Err(e), _) => println!("triage server exited with an error: {}", e),
        (_, Err(e)) => println!("tracing server exited with an error: {}", e),
    };
}
