use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn configure_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "INFO".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() {
    configure_tracing();

    let dispatch = coldmod_d::dispatch::Dispatch::new().await;

    let grpc_dispatch = dispatch.clone();
    let web_dispatch = dispatch.clone();

    // let bx_dispatch = Box::pin(coldmod_d::dispatch::Dispatch::new().await);

    let web_worker = tokio::spawn(async move { coldmod_d::web::server(web_dispatch).await });
    let grpc_worker = tokio::spawn(async move { coldmod_d::grpc::server(&grpc_dispatch).await });
    // let dispatch_worker = tokio::spawn(async move { dispatch.start().await });

    match tokio::try_join!(web_worker, grpc_worker) {
        Ok(_) => println!("all workers exited"),
        Err(e) => println!("one or more workers exited with an error: {}", e),
    };
}
