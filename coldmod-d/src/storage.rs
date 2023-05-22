use async_channel::Receiver;
use coldmod_msg::proto::{SourceScan, Trace};
use prost::Message;
use redis::RedisError;

pub async fn server(receiver: Receiver<Trace>) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client
        .get_async_connection()
        .await
        .expect("couldn't connect to redis");

    while let Ok(trace) = receiver.recv().await {
        let bytes = trace.encode_to_vec();
        let result: Result<String, RedisError> = redis::cmd("XADD")
            .arg(&["tracing-stream", "*", "trace"])
            .arg(bytes)
            .query_async(&mut conn)
            .await;
        match result {
            Ok(_) => (), // this is an id of the newly stored entry
            Err(e) => {
                eprintln!("failed to store trace: {:?}", e);
            }
        }
    }
}

pub async fn store_source_scan(source_scan: SourceScan) -> Result<(), RedisError> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client
        .get_async_connection()
        .await
        .expect("couldn't connect to redis");

    let bytes = source_scan.encode_to_vec();

    redis::cmd("HSET")
        .arg("source-scan")
        .arg("default")
        .arg(bytes)
        .query_async(&mut conn)
        .await
}
