use coldmod_msg::proto::tracing_daemon_client::TracingDaemonClient;
use coldmod_msg::proto::Trace;
use futures_util::stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TracingDaemonClient::connect("http://127.0.0.1:7777").await?;

    let trace = Trace {
        path: "/a/path/to/a/file".into(),
        line: 7263,
        process_id: 1231231,
        thread_id: 1230920,
    };

    let traces = vec![trace];
    let response = client.collect(stream::iter(traces)).await;

    println!("response: {:?}", response);

    Ok(())
}
