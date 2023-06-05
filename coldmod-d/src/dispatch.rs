use crate::store;
use coldmod_msg::web::{self, Msg};
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct Dispatch {
    store: store::RedisStore,
    broadcast: broadcast::Sender<Msg>,
}

impl Dispatch {
    pub async fn new() -> Self {
        let sender = broadcast::channel(65536).0;
        Self {
            store: store::RedisStore::new().await,
            broadcast: sender,
        }
    }

    pub async fn handle(&self, msg: Msg) -> Result<Vec<Msg>, anyhow::Error> {
        let mut store = self.store.clone();

        match msg {
            Msg::AppSocketConnected => {
                let count = store.trace_count().await?;
                let source_scan = store.get_source_scan().await?;
                return Ok(vec![
                    Msg::TracingStatsAvailable(web::TracingStats { count }),
                    Msg::SourceDataAvailable(source_scan),
                ]);
            }
            Msg::TraceReceived(trace) => {
                store.store_trace(trace).await?;
                self._broadcast_trace_count(store).await?;
            }
            Msg::SourceReceived(scan) => {
                store.store_source_scan(&scan).await?;
                self._broadcast(Msg::SourceDataAvailable(Some(scan)));
            }
            Msg::RequestSourceData => {
                let scan = store.get_source_scan().await?;
                return Ok(vec![Msg::SourceDataAvailable(scan)]);
            }
            _ => {}
        };

        Ok(vec![])
    }

    pub fn receiver(&self) -> broadcast::Receiver<Msg> {
        self.broadcast.subscribe()
    }

    async fn _broadcast_trace_count(
        &self,
        mut store: store::RedisStore,
    ) -> Result<(), anyhow::Error> {
        let count = store.trace_count().await?;
        self._broadcast(Msg::TracingStatsAvailable(web::TracingStats { count }));
        Ok(())
    }

    fn _broadcast(&self, msg: Msg) {
        match self.broadcast.send(msg) {
            Ok(_) => {
                tracing::trace!("message broadcast ok");
            }
            Err(e) => {
                tracing::error!("failed to broadcast message: {:?}", e);
            }
        }
    }
}
