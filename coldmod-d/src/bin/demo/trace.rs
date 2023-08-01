use coldmod_msg::proto::{traces_client::TracesClient, Trace};
use futures_util::stream;

pub(crate) async fn trace(digest: String, incr: Option<usize>) {
    let mut client = TracesClient::connect("http://127.0.0.1:7777")
        .await
        .expect("failed to connect to source daemon");

    let c = incr.unwrap_or(1);

    let mut traces: Vec<Trace> = Vec::new();

    for _ in 0..c {
        traces.push(Trace {
            digest: digest.clone(),
            process_id: "0".into(),
            thread_id: "0".into(),
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
