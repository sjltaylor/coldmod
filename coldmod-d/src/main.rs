use argh::FromArgs;
use coldmod_msg::proto::Trace;
use std::net::{SocketAddr, ToSocketAddrs};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn configure_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "INFO".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn read_env_vars() -> (
    SocketAddr,
    SocketAddr,
    String,
    Option<(String, String)>,
    Option<String>,
    Option<String>,
) {
    let web_host = std::env::var("COLDMOD_WEB_HOST")
        .expect("COLDMOD_WEB_HOST not set")
        .to_socket_addrs()
        .expect("COLDMOD_WEB_HOST is not a valid socket address")
        .filter(|i| i.is_ipv4())
        .next()
        .expect("COLDMOD_WEB_HOST is not an IPv4 address");
    let grpc_host = std::env::var("COLDMOD_GRPC_HOST")
        .expect("COLDMOD_GRPC_HOST not set")
        .to_socket_addrs()
        .expect("COLDMOD_GRPC_HOST is not a valid socket address")
        .filter(|i| i.is_ipv4())
        .next()
        .expect("COLDMOD_GRPC_HOST is not an IPv4 address");
    let redis_host = std::env::var("COLDMOD_REDIS_HOST").expect("COLDMOD_REDIS_HOST not set");

    let insecure = std::env::var("COLDMOD_INSECURE").map_or_else(|_| false, |v| v == "on");

    let (api_key, tls, origin) = if insecure {
        (None, None, None)
    } else {
        (
            Some(std::env::var("COLDMOD_API_KEY").expect("COLDMOD_API_KEY not set")),
            Some((
                std::env::var("COLDMOD_TLS_CERT").expect("COLDMOD_TLS_CERT not set"),
                std::env::var("COLDMOD_TLS_KEY").expect("COLDMOD_TLS_KEY not set"),
            )),
            Some(std::env::var("COLDMOD_APP_ORIGIN").expect("COLDMOD_APP_ORIGIN not set")),
        )
    };

    return (web_host, grpc_host, redis_host, tls, api_key, origin);
}

#[derive(FromArgs)]
/// This is the Coldmod server CLI
struct Main {
    #[argh(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Cmd {
    Reset(ResetCmd),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Reset data (only heatmap by default)
#[argh(subcommand, name = "reset")]
struct ResetCmd {
    #[argh(switch)]
    /// confirmation that data should be reset
    confirm: bool,

    #[argh(switch)]
    /// clear all data
    all: bool,
}

#[tokio::main]
async fn main() {
    let main: Main = argh::from_env();

    if let Some(cmd) = main.cmd {
        match cmd {
            Cmd::Reset(reset_cmd) => {
                if reset_cmd.confirm {
                    reset(reset_cmd.all).await;
                } else {
                    println!("--confirm that you want to reset the data");
                }
            }
        };
    } else {
        start().await
    }
}

async fn reset(all: bool) {
    let redis_host = std::env::var("COLDMOD_REDIS_HOST").expect("COLDMOD_REDIS_HOST not set");
    let mut store = coldmod_d::store::RedisStore::new(redis_host).await;
    if all {
        store.reset_all().await.unwrap();
    } else {
        store.reset_heatmap().await.unwrap();
    }
    println!("done.");
}

async fn start() {
    let (web_host, grpc_host, redis_host, tls, api_key, origin) = read_env_vars();

    configure_tracing();

    let (rate_limiter, rate_limited) = tokio::sync::mpsc::channel::<()>(1);
    let (trace_sink, trace_source) = tokio::sync::mpsc::channel::<Trace>(65536);

    let dispatch = coldmod_d::dispatch::Dispatch::new(
        grpc_host,
        web_host,
        redis_host,
        api_key,
        tls,
        origin,
        rate_limiter,
        trace_sink,
    )
    .await;

    let grpc_dispatch = dispatch.clone();
    let web_dispatch = dispatch.clone();
    let event_spool_dispatch = dispatch.clone();

    // TODO: why sometimes &dispatch and sometimes dispatch?
    let web_worker = tokio::spawn(async move { coldmod_d::web::server(web_dispatch).await });
    let grpc_worker = tokio::spawn(async move { coldmod_d::grpc::server(&grpc_dispatch).await });
    let event_spool_worker = tokio::spawn(async move {
        event_spool_dispatch
            .start_rate_limited_event_spool(rate_limited)
            .await
    });
    let trace_sink_worker =
        tokio::spawn(async move { dispatch.start_trace_sink(trace_source).await });

    match tokio::try_join!(
        web_worker,
        grpc_worker,
        event_spool_worker,
        trace_sink_worker
    ) {
        Ok(_) => println!("all workers exited"),
        Err(e) => println!("one or more workers exited with an error: {}", e),
    };
}
