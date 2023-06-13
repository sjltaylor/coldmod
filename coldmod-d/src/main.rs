use coldmod_msg::web::Msg;
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
    let cmd_reset = std::env::args().any(|arg| arg == "--reset");
    let cmd_start = std::env::args().any(|arg| arg == "--start") || std::env::args().count() == 1;

    configure_tracing();

    let (rate_limiter, rate_limited) = tokio::sync::mpsc::channel::<()>(1);

    let dispatch = coldmod_d::dispatch::Dispatch::new(rate_limiter).await;

    if cmd_reset {
        dispatch.handle(Msg::Reset).await.unwrap();
    }

    if !cmd_start {
        return;
    }

    let grpc_dispatch = dispatch.clone();
    let web_dispatch = dispatch.clone();

    let web_worker = tokio::spawn(async move { coldmod_d::web::server(web_dispatch).await });
    let grpc_worker = tokio::spawn(async move { coldmod_d::grpc::server(&grpc_dispatch).await });
    let event_spool_worker =
        tokio::spawn(async move { dispatch.start_rate_limited_event_spool(rate_limited).await });

    match tokio::try_join!(web_worker, grpc_worker, event_spool_worker) {
        Ok(_) => println!("all workers exited"),
        Err(e) => println!("one or more workers exited with an error: {}", e),
    };
}
