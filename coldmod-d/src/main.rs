use coldmod_d::store;
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

    let dispatch = coldmod_d::dispatch::Dispatch {
        store: store::RedisStore::new().await,
        trace_ch: async_channel::bounded(65536),
    };
    let grpc_dispatch = dispatch.clone();
    let web_dispatch = dispatch.clone();
    let storage_dispatch = dispatch.clone();

    let web = tokio::spawn(async move { coldmod_d::web::server(web_dispatch).await });
    let grpc = tokio::spawn(async move { coldmod_d::grpc::server(grpc_dispatch).await });
    let storage = tokio::spawn(async move { store::tracing_sink(storage_dispatch).await });

    match tokio::try_join!(web, grpc, storage) {
        Ok(_) => println!("all servers exited"),
        Err(e) => println!("one or more servers exited with an error: {}", e),
    };
}
