use crate::store;
use async_trait::async_trait;
use coldmod_msg::proto::{FilterSet, FilterSetQuery};
use coldmod_msg::web::{self, Msg};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Dispatch {
    pub(crate) store: store::RedisStore,
    internal: broadcast::Sender<Msg>,
    rate_limiter: mpsc::Sender<()>,
    tracesrcs_listeners: Arc<RwLock<HashMap<String, mpsc::Sender<FilterSet>>>>,
}

impl Dispatch {
    pub async fn new(rate_limiter: mpsc::Sender<()>) -> Self {
        let internal = broadcast::channel(6553).0;

        Self {
            store: store::RedisStore::new().await,
            internal,
            rate_limiter,
            tracesrcs_listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn handle(&self, msg: Msg) -> Result<Vec<Msg>, anyhow::Error> {
        let mut store = self.store.clone();

        match msg {
            Msg::Reset => {
                store.reset().await?;
            }
            Msg::TraceReceived(trace) => {
                store.store_trace(trace).await?;
                self._pulse_rate_limiter();
            }
            Msg::SetTraceSrcs(scan) => {
                store.set_trace_srcs(&scan).await?;
                let heat_map = store.get_heat_map().await?.unwrap();
                self._broadcast(Msg::HeatMapAvailable(heat_map));
            }
            Msg::SetFilterSet((filterset, key)) => {
                tracing::info!("set filterset:{:?}", key);
                self._send_filterset_to_listener(&key, filterset).await;
            }
            _ => {}
        };

        Ok(vec![])
    }

    pub fn receiver(&self) -> broadcast::Receiver<Msg> {
        self.internal.subscribe()
    }

    fn _broadcast(&self, msg: Msg) {
        if self.internal.receiver_count() == 0 {
            return;
        }

        match self.internal.send(msg) {
            Ok(_) => {
                tracing::trace!("message broadcast ok");
            }
            Err(e) => {
                tracing::error!("failed to broadcast message: {:?}", e);
            }
        }
    }

    fn _pulse_rate_limiter(&self) {
        match self.rate_limiter.try_send(()) {
            Ok(_) | Err(mpsc::error::TrySendError::Full(_)) => return,
            Err(e) => {
                tracing::error!("rate limited channel error");
                panic!("{}", e);
            }
        }
    }

    pub async fn start_rate_limited_event_spool(&self, mut rate_limited: mpsc::Receiver<()>) {
        let mut store = self.store.clone();

        loop {
            match rate_limited.recv().await {
                Some(_) => {}
                None => {
                    tracing::error!("rate limited event spooler channel closed");
                    return;
                }
            }

            match store.update_heatmap().await {
                Ok(Some(heatmap_delta)) => {
                    self._broadcast(Msg::HeatMapChanged(heatmap_delta));
                }
                Ok(None) => {}
                Err(e) => {
                    tracing::error!("failed to update heat map: {:?}", e);
                }
            }

            match store.trace_count().await {
                Ok(count) => {
                    self._broadcast(Msg::TracingStatsAvailable(web::TracingStats { count }));
                }
                Err(e) => {
                    tracing::error!("failed to get trace count: {:?}", e);
                }
            }
        }
    }

    pub async fn serve_socket<WS: WebSocket>(&self, mut ws: WS) {
        let mut store = self.store.clone();

        // before any awaiting to make sure internal messages are buffered
        let mut broadcast = self.internal.subscribe();

        match store.trace_count().await {
            Ok(count) => {
                let tracing_states_available =
                    Msg::TracingStatsAvailable(web::TracingStats { count });
                if let Err(e) = ws.send(&tracing_states_available).await {
                    tracing::error!("failed to send trace count: {:?}", e);
                }
            }
            Err(e) => {
                tracing::error!("failed to get trace count: {:?}", e);
                return;
            }
        };

        match store.get_heat_map().await {
            Ok(Some(heat_map)) => {
                let heat_map_available = Msg::HeatMapAvailable(heat_map);
                if let Err(e) = ws.send(&heat_map_available).await {
                    tracing::error!("failed to send heat map: {:?}", e);
                }
            }
            Ok(None) => {}
            Err(e) => {
                tracing::error!("failed to get heat map: {:?}", e);
                return;
            }
        };

        loop {
            let ws_send = tokio::select! {
                ws_msg = ws.receive() => {
                    match ws_msg {
                        Some(Ok(msg)) => {
                            match self.handle(msg).await {
                                Ok(replies) => replies,
                                Err(e) => {
                                    tracing::error!("error handling message: {}", e);
                                    continue;
                                }
                            }
                        },
                        Some(Err(e)) => {
                            tracing::error!("error receiving message: {}", e);
                            continue;
                        }
                        None => {
                            tracing::info!("socket closed");
                            return;
                        }
                    }
                },
                b_msg = broadcast.recv() => {
                    match b_msg {
                        Ok(msg) => match msg {
                            Msg::HeatMapAvailable(_) |  Msg::TracingStatsAvailable(_) | Msg::HeatMapChanged(_) => vec!(msg),
                            _ => vec!()
                        }
                        Err(e) => {
                            tracing::error!("error receiving broadcast: {}", e);
                            continue;
                        }
                    }
                }
            };

            for msg in ws_send {
                if let Err(e) = ws.send(&msg).await {
                    tracing::error!("failed to send reply: {:?}", e);
                    return;
                }
            }
        }
    }

    pub async fn send_filtersets_until_closed(
        &self,
        q: FilterSetQuery,
        tx: tokio::sync::mpsc::Sender<FilterSet>,
    ) {
        self.tracesrcs_listeners
            .write()
            .await
            .insert(q.key.clone(), tx.clone());

        tx.closed().await;
        tracing::info!("tx closed - removing listener");
        self.tracesrcs_listeners.write().await.remove(&q.key);
    }

    async fn _send_filterset_to_listener(&self, key: &String, filterset: FilterSet) {
        tracing::info!("listeners: {:?}", self.tracesrcs_listeners.read().await);

        match self.tracesrcs_listeners.read().await.get(key) {
            Some(tx) => {
                if let Err(e) = tx.send(filterset).await {
                    tracing::error!("failed to send filterset to listener: {:?}", e);
                }
            }
            None => {}
        }
    }
}

#[async_trait]
pub trait WebSocket {
    async fn send(&mut self, msg: &Msg) -> Result<(), anyhow::Error>;
    async fn receive(&mut self) -> Option<Result<Msg, anyhow::Error>>;
}
