use coldmod_msg::proto::{tracing_daemon_client::TracingDaemonClient, Trace};
use futures_util::stream;

pub(crate) async fn simulate_tracing(key: String, incr: Option<usize>) {
    let mut client = TracingDaemonClient::connect("http://127.0.0.1:7777")
        .await
        .expect("failed to connect to source daemon");

    let parts = key.split(":").collect::<Vec<&str>>();
    let path = parts[0];
    let line = parts[1].parse::<u32>().expect("failed to parse line");

    let c = incr.unwrap_or(1);

    let mut traces: Vec<Trace> = Vec::new();

    for _ in 0..c {
        traces.push(Trace {
            path: path.to_string(),
            line,
            process_id: 0,
            thread_id: 0,
        });
    }

    match client.collect(stream::iter(traces)).await {
        Ok(_) => {
            println!("done.");
        }
        Err(e) => {
            eprintln!("failure sending traces: {:?}", e);
        }
    }
}
