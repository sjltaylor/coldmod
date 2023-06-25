use coldmod_msg::proto::{
    source_daemon_client::SourceDaemonClient, tracing_daemon_client::TracingDaemonClient,
    SourceScan, Trace,
};
use futures_util::stream;
use prost::Message;

pub async fn send_source_scan() {
    let mut client = SourceDaemonClient::connect("http://127.0.0.1:7777")
        .await
        .expect("failed to connect to source daemon");

    let raw = std::fs::read("samples/source-scan.flexbuffers").expect("failed to read source scan");

    let source_scan: SourceScan =
        flexbuffers::from_slice(&raw).expect("failed to parse source scan");

    match client.submit(source_scan).await {
        Ok(_) => {
            println!("done.");
        }
        Err(e) => {
            eprintln!("failed to send source scan: {:?}", e);
        }
    }
}

pub async fn send_tracing_stream() {
    let mut client = TracingDaemonClient::connect("http://127.0.0.1:7777")
        .await
        .expect("failed to connect to source daemon");

    let raw =
        std::fs::read("samples/tracing-stream.flexbuffers").expect("failed to read tracing stream");

    let raw_traces: Vec<Vec<u8>> =
        flexbuffers::from_slice(&raw).expect("failed to decode raw traces");

    let traces: Vec<Trace> = raw_traces
        .into_iter()
        .map(|raw| Trace::decode(&raw[..]).expect("failed to decode trace"))
        .collect();

    match client.collect(stream::iter(traces)).await {
        Ok(_) => {
            println!("done.");
        }
        Err(e) => {
            eprintln!("failure sending traces: {:?}", e);
        }
    }
}
