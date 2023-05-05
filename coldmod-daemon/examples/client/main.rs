use coldmod_daemon::proto::tracing_collector_client::TracingCollectorClient;
use coldmod_daemon::proto::Trace;
use futures_util::stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TracingCollectorClient::connect("http://127.0.0.1:7777").await?;

    let trace = Trace {
        path: "/a/path/to/a/file".into(),
        line: 7263,
    };

    let traces = vec![trace];
    let response = client.trace(stream::iter(traces)).await;

    println!("response: {:?}", response);

    Ok(())
}
