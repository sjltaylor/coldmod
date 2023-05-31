use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn configure_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "coldmod_d=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() {
    configure_tracing();

    let mut dispatch = coldmod_d::dispatch::Dispatch::new().await;

    let grpc_dispatch = dispatch.clone();
    let web_dispatch = dispatch.clone();

    let web_worker = tokio::spawn(async move { coldmod_d::web::server(web_dispatch).await });
    let grpc_worker = tokio::spawn(async move { coldmod_d::grpc::server(grpc_dispatch).await });
    let dispatch_worker = tokio::spawn(async move { dispatch.start().await });

    match tokio::try_join!(web_worker, grpc_worker, dispatch_worker) {
        Ok(_) => println!("all servers exited"),
        Err(e) => println!("one or more servers exited with an error: {}", e),
    };
}
