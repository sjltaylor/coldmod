use coldmod_msg::{
    proto::{SourceScan, Trace},
    web::{ElementKey, HeatMap, HeatMapDelta, HeatSource},
};
use prost::Message;
use redis::AsyncCommands;
use redis::{streams::StreamRangeReply, RedisError, Value};

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

    pub async fn initialize_heat_map(
        &mut self,
        source_scan: &SourceScan,
    ) -> Result<(), RedisError> {
        let bytes = source_scan.encode_to_vec();
        let mut q = redis::pipe();

        q.atomic();
        q.hset("source-scan", "raw", bytes).ignore();
        q.hset(
            "source-scan",
            "coldmod_root_marker_path",
            &source_scan.coldmod_root_marker_path,
        )
        .ignore();

        for source_element in source_scan.source_elements.iter() {
            q.hset("heat-map", source_element.key(), 0).ignore();
        }

        q.query_async(&mut self.heatmap_connection).await?;
        self._update_heatmap(false).await?;

        Ok(())
    }

    pub async fn update_heatmap(&mut self) -> Result<Option<HeatMapDelta>, RedisError> {
        self._update_heatmap(true).await
    }

    pub async fn reset(&mut self) -> Result<(), RedisError> {
        let mut q = redis::pipe();
        q.atomic();
        q.del("source-scan").ignore();
        q.del("heat-map").ignore();
        q.del("heat-map-status").ignore();
        q.del("tracing-stream").ignore();
        q.query_async(&mut self.connection).await?;

        tracing::info!("state reset");
        Ok(())
    }

    async fn _update_heatmap(
        &mut self,
        compute_delta: bool,
    ) -> Result<Option<HeatMapDelta>, RedisError> {
        let (heatmap_exists, last_update_id_from_tracing_stream, coldmod_root_marker_path): (
            bool,
            Option<String>,
            Option<String>,
        ) = redis::pipe()
            .exists("heat-map")
            .hget("heat-map-status", "last-update-id-from-tracing-stream")
            .hget("source-scan", "coldmod_root_marker_path")
            .query_async(&mut self.connection)
            .await?;

        if !heatmap_exists {
            tracing::info!("no heatmap exists");
            return Ok(None);
        }

        let mut tracing_stream_last_id = last_update_id_from_tracing_stream;

        let mut heat_map_deltas = HeatMapDelta::default();

        let coldmod_root_marker_path =
            coldmod_root_marker_path.expect("no coldmod_root_marker_path");
        let coldmod_root_marker_prefix = std::path::Path::new(&coldmod_root_marker_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();

        loop {
            let start_specifier = match &tracing_stream_last_id {
                Some(id) => format!("({}", id),
                None => "-".to_string(),
            };

            let traces: StreamRangeReply = self
                .heatmap_connection
                .xrange_count("tracing-stream", &start_specifier, "+", 65536)
                .await?;

            if traces.ids.is_empty() {
                break;
            }

            let mut q = redis::pipe();
            q.atomic();

            for id in traces.ids.iter() {
                tracing_stream_last_id = Some(id.id.clone());

                let trace = match id.map.get("trace") {
                    Some(Value::Data(raw)) => match Trace::decode(&raw[..]) {
                        Ok(trace) => trace,
                        Err(e) => {
                            tracing::error!("error decoding trace in {:?}: {:?}", id.id, e);
                            continue;
                        }
                    },
                    Some(_) => {
                        tracing::error!("trace in {:?} is the wrong type", id.id);
                        continue;
                    }
                    None => {
                        tracing::error!("no trace in {:?}", id.id);
                        continue;
                    }
                };

                let key = trace.key(coldmod_root_marker_prefix);

                if compute_delta {
                    match heat_map_deltas.deltas.get_mut(&key) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            heat_map_deltas.deltas.insert(key.clone(), 1);
                        }
                    }
                }

                q.hincr("heat-map", key, 1).ignore();
            }

            q.hset(
                "heat-map-status",
                "last-update-id-from-tracing-stream",
                &tracing_stream_last_id,
            )
            .ignore()
            .query_async(&mut self.heatmap_connection)
            .await?;
        }

        if compute_delta {
            Ok(Some(heat_map_deltas))
        } else {
            Ok(None)
        }
    }

    pub async fn get_heat_map(&mut self) -> Result<Option<HeatMap>, RedisError> {
        let source_scan = match self.get_source_scan().await? {
            Some(source_scan) => source_scan,
            None => return Ok(None),
        };

        let mut q = redis::pipe();

        for source_element in source_scan.source_elements.iter() {
            q.hget("heat-map", source_element.key());
        }

        let result: Vec<i64> = q.query_async(&mut self.heatmap_connection).await?;

        let heat_sources = source_scan
            .source_elements
            .into_iter()
            .zip(result)
            .map(|(source_element, trace_count)| HeatSource {
                source_element,
                trace_count,
            })
            .collect();

        Ok(Some(HeatMap {
            sources: heat_sources,
        }))
    }

    pub async fn get_source_scan(&mut self) -> Result<Option<SourceScan>, RedisError> {
        let raw: Vec<u8> = redis::cmd("HGET")
            .arg("source-scan")
            .arg("raw")
            .query_async(&mut self.connection)
            .await?;

        if raw.is_empty() {
            return Ok(None);
        }

        let scan = SourceScan::decode(&raw[..]).unwrap();

        return Ok(Some(scan));
    }

    pub async fn store_trace(&mut self, trace: Trace) -> Result<(), RedisError> {
        let coldmod_root_marker_path: String = self
            .trace_connection
            .hget("source-scan", "coldmod_root_marker_path")
            .await?;

        let coldmod_root_marker_prefix = std::path::Path::new(&coldmod_root_marker_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();

        let key = trace.key(coldmod_root_marker_prefix);

        let exists: bool = self.trace_connection.hexists("heat-map", &key).await?;

        if !exists {
            tracing::info!(
                "source scan did not include the traced function: {:?}",
                trace
            );
            return Ok(());
        }

        let bytes = trace.encode_to_vec();
        redis::cmd("XADD")
            .arg(&["tracing-stream", "*", "trace"])
            .arg(bytes)
            .query_async(&mut self.trace_connection)
            .await?;
        Ok(())
    }

    pub async fn trace_count(&mut self) -> Result<i64, RedisError> {
        let count: i64 = self.trace_connection.xlen("tracing-stream").await?;
        Ok(count)
    }

    pub async fn _raw_trace_data(&mut self) -> Result<Vec<Vec<u8>>, anyhow::Error> {
        let mut stream_range: StreamRangeReply =
            self.connection.xrange_all("tracing-stream").await?;
        let mut traces: Vec<Vec<u8>> = Vec::new();

        for id in stream_range.ids.iter_mut() {
            match id.map.remove("trace") {
                Some(Value::Data(raw)) => traces.push(raw),
                Some(_) => return Err(anyhow::anyhow!("trace in {:?} is the wrong type", id.id)),
                None => {
                    return Err(anyhow::anyhow!("no trace in {:?}", id.id));
                }
            }
        }

        Ok(traces)
    }
}
