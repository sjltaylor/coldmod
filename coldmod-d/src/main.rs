use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn configure_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "coldmod_d=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() {
    configure_tracing();

    let (sender, receiver) = async_channel::bounded(65536);
    let web_receiver = receiver.clone();
    let storage_receiver = receiver.clone();

    let web = tokio::spawn(async move { coldmod_d::web::server(web_receiver).await });
    let grpc = tokio::spawn(async { coldmod_d::grpc::server(sender).await });
    let storage = tokio::spawn(async move { coldmod_d::storage::server(storage_receiver).await });

    match tokio::try_join!(web, grpc, storage) {
        Ok(_) => println!("all servers exited"),
        Err(e) => println!("one or more servers exited with an error: {}", e),
    };
}
