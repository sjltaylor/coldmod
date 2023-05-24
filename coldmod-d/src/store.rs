use coldmod_msg::proto::{SourceScan, Trace};
use prost::Message;
use redis::RedisError;

#[derive(Clone)]
pub struct RedisStore {
    pub connection_info: redis::ConnectionInfo,
    connection: redis::aio::MultiplexedConnection,
}

impl RedisStore {
    pub async fn new() -> RedisStore {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();

        RedisStore {
            connection_info: client.get_connection_info().clone(),
            connection: client
                .get_multiplexed_async_connection()
                .await
                .expect("store couldn't connect to redis"),
        }
    }

    pub async fn store_source_scan(&self, source_scan: SourceScan) -> Result<(), RedisError> {
        let bytes = source_scan.encode_to_vec();

        redis::cmd("HSET")
            .arg("source-scan")
            .arg("default")
            .arg(bytes)
            .query_async(&mut self.connection.clone())
            .await
    }
}

pub trait DispatchContext: Send + Sync + Clone + 'static {
    fn get_redis_connection_info(&self) -> redis::ConnectionInfo;
    fn trace_receiver(&self) -> async_channel::Receiver<Trace>;
}

pub async fn tracing_sink<Dispatch: DispatchContext>(dispatch: Dispatch) {
    let client = redis::Client::open(dispatch.get_redis_connection_info()).unwrap();
    let mut conn = client
        .get_async_connection()
        .await
        .expect("couldn't connect to redis");

    while let Ok(trace) = dispatch.trace_receiver().recv().await {
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
