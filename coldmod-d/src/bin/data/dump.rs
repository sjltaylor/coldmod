use coldmod_d::store::RedisStore;

pub async fn dump_source_scan() {
    // let mut store = RedisStore::new().await;

    // let source_scan = store
    //     .get_source_scan()
    //     .await
    //     .expect("failed to get source scan");
    // let source_scan = source_scan.expect("no source scan");
    // let payload = flexbuffers::to_vec(&source_scan).expect("serialization failed");
    // match std::fs::write("samples/source-scan.flexbuffers", payload) {
    //     Ok(_) => {
    //         println!("done.");
    //     }
    //     Err(e) => {
    //         eprintln!("failed to write source scan: {:?}", e);
    //     }
    // }
}

pub async fn dump_tracing_stream() {
    // let mut store = RedisStore::new().await;
    // let traces = store
    //     ._raw_trace_data()
    //     .await
    //     .expect("failed to get raw trace data");

    // if traces.is_empty() {
    //     eprintln!("no traces to dump");
    //     return;
    // }

    // let payload = flexbuffers::to_vec(&traces).expect("serialization failed");

    // match std::fs::write("samples/tracing-stream.flexbuffers", payload) {
    //     Ok(_) => {
    //         println!("done.");
    //     }
    //     Err(e) => {
    //         eprintln!("failed to write tracing stream: {:?}", e);
    //     }
    // }
}
