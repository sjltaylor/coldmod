use crate::store;
use async_trait::async_trait;
use coldmod_msg::proto::{
    src_message::Message, ModCommand, SrcMessage, Trace, TraceSrcs,
};
use coldmod_msg::web::{self, Msg};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Dispatch {
    grpc_host: SocketAddr,
    web_host: SocketAddr,
    api_key: Option<String>,
    tls: Option<(String, String)>,
    // TODO: why is this pub crate?
    pub(crate) store: store::RedisStore,
    internal: broadcast::Sender<Msg>,
    rate_limiter: mpsc::Sender<()>,
    command_listeners: Arc<RwLock<HashMap<String, mpsc::Sender<ModCommand>>>>,
    websocket_listeners: Arc<RwLock<HashMap<String, mpsc::Sender<Msg>>>>,
    trace_sink: mpsc::Sender<Trace>,
}

impl Dispatch {
    pub fn web_host(&self) -> SocketAddr {
        self.web_host.clone()
    }

    pub fn grpc_host(&self) -> SocketAddr {
        self.grpc_host.clone()
    }

    pub fn api_key(&self) -> Option<String> {
        self.api_key.clone()
    }

    pub fn tls(&self) -> Option<(String, String)> {
        self.tls.clone()
    }

    pub async fn new(
        grpc_host: SocketAddr,
        web_host: SocketAddr,
        redis_host: String,
        api_key: Option<String>,
        tls: Option<(String, String)>,
        rate_limiter: mpsc::Sender<()>,
        trace_sink: mpsc::Sender<Trace>,
    ) -> Self {
        let internal = broadcast::channel(6553).0;

        Self {
            grpc_host,
            web_host,
            api_key,
            tls,
            store: store::RedisStore::new(redis_host).await,
            internal,
            rate_limiter,
            command_listeners: Arc::new(RwLock::new(HashMap::new())),
            websocket_listeners: Arc::new(RwLock::new(HashMap::new())),
            trace_sink,
        }
    }

    async fn _handle_websocket_msg(&self, msg: Msg) -> Result<Vec<Msg>, anyhow::Error> {
        match msg {
            Msg::RouteModCommandTo((command, receiver_key)) => {
                self.route_mod_command(command, receiver_key).await?;
            }
            _ => {
                tracing::warn!("unexpected websocket message: {}", msg);
            }
        };

        Ok(vec![])
    }

    pub async fn route_mod_command(
        &self,
        mod_command: ModCommand,
        key: String,
    ) -> Result<(), anyhow::Error> {
        tracing::info!("route mod command: {:?}", key);
        self._send_command_to_listener(&key, mod_command).await;
        Ok(())
    }

    pub async fn reset_all(&self) -> Result<(), anyhow::Error> {
        let mut store = self.store.clone();
        store.reset_all().await?;
        Ok(())
    }

    pub async fn reset_heatmap(&self) -> Result<(), anyhow::Error> {
        let mut store = self.store.clone();
        store.reset_heatmap().await?;
        Ok(())
    }

    pub async fn set_trace_srcs(&self, trace_srcs: TraceSrcs) -> Result<(), anyhow::Error> {
        let mut store = self.store.clone();

        store.set_trace_srcs(&trace_srcs).await?;
        let heat_map = store.get_heat_map().await?.unwrap();
        self._broadcast(Msg::HeatMapAvailable(heat_map));

        Ok(())
    }

    pub async fn trace_received(&self, trace: Trace) -> Result<(), anyhow::Error> {
        self.trace_sink.send(trace).await?;
        Ok(())
    }

    async fn store_traces(&self, traces: &Vec<Trace>) -> Result<(), anyhow::Error> {
        let mut store = self.store.clone();
        store.store_traces(traces).await?;
        self._pulse_rate_limiter();
        Ok(())
    }

    pub async fn start_trace_sink(&self, mut trace_source: mpsc::Receiver<Trace>) {
        let timer_duration = tokio::time::Duration::from_millis(10);
        let mut interval = tokio::time::interval(timer_duration);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        let buffer_len = 65536;
        let mut buffer = Vec::<Trace>::with_capacity(buffer_len);

        loop {
            let mut timer_flush = false;

            tokio::select! {
                trace = trace_source.recv() => {
                    match trace {
                        Some(trace) => {
                            buffer.push(trace);
                            interval.reset();
                        }
                        None => {
                            return;
                        }
                    }
                }
                _ = interval.tick() => {
                    timer_flush = buffer.len() > 0;
                }
            }

            if timer_flush || buffer.len() >= buffer_len {
                self.store_traces(&buffer).await.unwrap();
                buffer.clear();
            }
        }
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
                    tracing::debug!("heatmap changed");
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

    pub async fn serve_socket<WS: WebSocket + Clone + 'static>(&self, mut ws: WS) {
        let mut store = self.store.clone();

        let (send_to_key, mut receive_for_key) = mpsc::channel::<Msg>(65536);

        if self
            .command_listeners
            .read()
            .await
            .contains_key(ws.key().as_str())
        {
            if let Err(e) = send_to_key.send(Msg::ModCommandClientAvailable).await {
                tracing::error!("failed to send ModCommandClientAvailable: {:?}", e);
            }
        }

        self.websocket_listeners
            .write()
            .await
            .insert(ws.key(), send_to_key);

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
                            match self._handle_websocket_msg(msg).await {
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
                            break;
                        }
                    }
                },
                key_msg = receive_for_key.recv() => {
                    match key_msg {
                        Some(msg) => vec!(msg),
                        None => {
                            tracing::info!("key channel closed");
                            break;
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
                            break;
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

        // TODO: implement Drop to handle this cleanup
        self.websocket_listeners.write().await.remove(&ws.key());
    }

    pub async fn handle_messages_until_closed(
        &self,
        mod_command_tx: tokio::sync::mpsc::Sender<ModCommand>,
        mut src_message_rx: tokio::sync::mpsc::Receiver<SrcMessage>,
    ) {
        let mut key = Option::<String>::None;

        loop {
            tokio::select! {
                maybe_src_message = src_message_rx.recv() => {
                    if maybe_src_message.is_none() {
                        tracing::info!("src message channel closed");
                        break;
                    }
                    let message = maybe_src_message.unwrap().message;
                    match message {
                        Some(Message::ConnectKey(connect_key)) => {
                            key = Some(connect_key.key.clone());

                            self.command_listeners
                                .write()
                                .await
                                .insert(connect_key.key.clone(), mod_command_tx.clone());

                            // if there is a app already, let it know a cli is available
                            if let Some(ws) = self.websocket_listeners.write().await.get_mut(&connect_key.key) {
                                if let Err(e) = ws.send(Msg::ModCommandClientAvailable).await {
                                    tracing::error!("failed to send ModCommandClientAvailable: {:?}", e);
                                }
                            }
                        }
                        _ => {
                            tracing::warn!("unexpected src message: {:?}", message);
                        }
                    }
                }
                _ = mod_command_tx.closed() => {
                    tracing::info!("tx closed - removing listener");
                    if let Some(key) = key {
                        self.command_listeners.write().await.remove(&key);
                        // if there is an app, let it know a cli is unavailable
                        if let Some(ws) = self.websocket_listeners.write().await.get_mut(&key) {
                            let msg = Msg::ModCommandClientUnavailable;
                            if let Err(e) = ws.send(msg).await {
                                tracing::error!("failed to send ModCommandClientUnavailable: {:?}", e);
                            }
                        }
                    }

                    break;
                }
            }
        }
    }

    async fn _send_command_to_listener(&self, key: &String, cmd: ModCommand) {
        match self.command_listeners.read().await.get(key) {
            Some(tx) => {
                if let Err(e) = tx.send(cmd).await {
                    tracing::error!("failed to send ModCommand to listener: {:?}", e);
                }
            }
            None => {}
        }
    }
}

#[async_trait]
pub trait WebSocket: Send + Sync {
    async fn send(&mut self, msg: &Msg) -> Result<(), anyhow::Error>;
    async fn receive(&mut self) -> Option<Result<Msg, anyhow::Error>>;
    fn key(&self) -> String;
}
