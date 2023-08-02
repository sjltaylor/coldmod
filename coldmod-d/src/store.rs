use std::collections::HashSet;

use coldmod_msg::{
    proto::{Trace, TraceSrc, TraceSrcs},
    web::{HeatMap, HeatMapDelta, HeatSrc},
};
use prost::Message;
use redis::AsyncCommands;
use redis::{streams::StreamRangeReply, RedisError};

#[derive(Clone)]
pub struct RedisStore {
    connection: redis::aio::MultiplexedConnection,
    trace_connection: redis::aio::MultiplexedConnection,
    heatmap_connection: redis::aio::MultiplexedConnection,
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

            heatmap_connection: client
                .get_multiplexed_async_connection()
                .await
                .expect("store couldn't connect to redis, for stream connection"),
        }
    }

    pub async fn set_trace_srcs(&mut self, trace_srcs: &TraceSrcs) -> Result<(), RedisError> {
        let mut q = redis::pipe();

        q.keys("trace_src:*").hkeys("heat_map");
        let (trace_src_keys, heatmap_keys): (Vec<String>, Vec<String>) =
            q.query_async(&mut self.heatmap_connection).await?;

        let mut trace_src_keys: HashSet<_> = trace_src_keys.into_iter().collect();
        let mut heatmap_keys: HashSet<_> = heatmap_keys.into_iter().collect();

        let mut q = redis::pipe();

        q.atomic();

        q.hset("trace_srcs", "root_path", &trace_srcs.root_path)
            .ignore();

        for trace_src in trace_srcs.trace_srcs.iter() {
            let digest = &trace_src.digest;

            q.hset_nx("heat_map", digest, 0).ignore();
            heatmap_keys.remove(digest);

            let key = format!("trace_src:{digest}");
            let bytes = trace_src.encode_to_vec();
            q.hset(&key, "raw", bytes).ignore();
            trace_src_keys.remove(&key);
        }

        for key in trace_src_keys {
            q.del(key).ignore();
        }

        for key in heatmap_keys {
            q.hdel("heat_map", key).ignore();
        }

        q.query_async(&mut self.heatmap_connection).await?;

        Ok(())
    }

    pub async fn reset(&mut self) -> Result<(), RedisError> {
        let keys: Vec<String> = self.connection.keys("*").await?;

        let mut q = redis::pipe();

        for key in keys {
            q.del(key).ignore();
        }

        q.query_async(&mut self.connection).await?;

        tracing::info!("state reset");
        Ok(())
    }

    pub async fn update_heatmap(&mut self) -> Result<Option<HeatMapDelta>, RedisError> {
        let (heatmap_exists, last_update_id_from_trace_stream): (bool, Option<String>) =
            redis::pipe()
                .exists("heat_map")
                .hget("heat_map_status", "last_update_id_from_trace_stream")
                .query_async(&mut self.connection)
                .await?;

        if !heatmap_exists {
            tracing::info!("no heatmap exists");
            return Ok(None);
        }

        let mut trace_stream_last_id = last_update_id_from_trace_stream;

        let mut heat_map_deltas = HeatMapDelta::default();

        loop {
            let start_specifier = match &trace_stream_last_id {
                Some(id) => format!("({}", id),
                None => "-".to_string(),
            };

            let traces: StreamRangeReply = self
                .heatmap_connection
                .xrange_count("trace_stream", &start_specifier, "+", 65536)
                .await?;

            if traces.ids.is_empty() {
                break;
            }

            let mut q = redis::pipe();
            q.atomic();

            for id in traces.ids.iter() {
                trace_stream_last_id = Some(id.id.clone());

                let digest: String = id.get("digest").unwrap();

                match heat_map_deltas.deltas.get_mut(&digest) {
                    Some(count) => {
                        *count += 1;
                    }
                    None => {
                        heat_map_deltas.deltas.insert(digest.clone(), 1);
                    }
                }

                q.hincr("heat_map", &digest, 1).ignore();
            }

            q.hset(
                "heat_map_status",
                "last_update_id_from_trace_stream",
                &trace_stream_last_id,
            )
            .ignore()
            .query_async(&mut self.heatmap_connection)
            .await?;
        }

        Ok(Some(heat_map_deltas))
    }

    pub async fn get_heat_map(&mut self) -> Result<Option<HeatMap>, RedisError> {
        let root_path: Option<String> = self
            .heatmap_connection
            .hget("trace_srcs", "root_path")
            .await?;

        if root_path.is_none() {
            return Ok(None);
        }

        let trace_src_keys: Vec<String> = self.heatmap_connection.get("trace_src:*").await?;

        let mut q = redis::pipe();
        for key in trace_src_keys.iter() {
            q.hget(key, "raw");
        }
        let trace_srcs_raw: Vec<Vec<u8>> = q.query_async(&mut self.heatmap_connection).await?;

        let trace_srcs: Vec<TraceSrc> = trace_srcs_raw
            .iter()
            .map(|raw| TraceSrc::decode(&raw[..]).unwrap())
            .collect();

        q = redis::pipe();
        for trace_src in trace_srcs.iter() {
            q.hget("heat_map", &trace_src.digest);
        }
        let counts: Vec<i64> = q.query_async(&mut self.heatmap_connection).await?;

        let heat_sources = trace_srcs
            .into_iter()
            .zip(counts)
            .map(|(trace_src, trace_count)| HeatSrc {
                trace_src,
                trace_count,
            })
            .collect();

        Ok(Some(HeatMap { srcs: heat_sources }))
    }

    pub async fn store_trace(&mut self, trace: Trace) -> Result<(), RedisError> {
        let bytes = trace.encode_to_vec();
        redis::cmd("XADD")
            .arg(&["trace_stream", "*", "digest", &trace.digest])
            .arg("raw")
            .arg(bytes)
            .query_async(&mut self.trace_connection)
            .await?;
        Ok(())
    }

    pub async fn trace_count(&mut self) -> Result<i64, RedisError> {
        let count: i64 = self.trace_connection.xlen("trace_stream").await?;
        Ok(count)
    }
}
