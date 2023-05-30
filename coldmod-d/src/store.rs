use coldmod_msg::proto::{SourceScan, Trace};
use prost::Message;
use redis::RedisError;

#[derive(Clone)]
pub struct RedisStore {
    connection: redis::aio::MultiplexedConnection,
    trace_connection: redis::aio::MultiplexedConnection,
}

impl RedisStore {
    pub async fn new() -> RedisStore {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();

        RedisStore {
            connection: client
                .get_multiplexed_async_connection()
                .await
                .expect("store couldn't connect to redis"),
            trace_connection: client
                .get_multiplexed_async_connection()
                .await
                .expect("store couldn't connect to redis, for tracing connection"),
        }
    }

    pub async fn store_source_scan(&mut self, source_scan: SourceScan) -> Result<(), RedisError> {
        let bytes = source_scan.encode_to_vec();

        redis::cmd("HSET")
            .arg("source-scan")
            .arg("default")
            .arg(bytes)
            .query_async(&mut self.connection)
            .await
    }

    pub async fn store_trace(&mut self, trace: Trace) -> Result<(), RedisError> {
        let bytes = trace.encode_to_vec();
        redis::cmd("XADD")
            .arg(&["tracing-stream", "*", "trace"])
            .arg(bytes)
            .query_async(&mut self.trace_connection)
            .await
    }
}
