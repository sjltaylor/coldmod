use crate::store;
use coldmod_msg::web::Event;
use tokio::sync::broadcast;

pub struct Dispatch {
    pub store: store::RedisStore,
    pub web_ch: (broadcast::Sender<Event>, broadcast::Receiver<Event>),
}

impl Clone for Dispatch {
    fn clone(&self) -> Self {
        let tx = self.web_ch.0.clone();
        let rx = self.web_ch.0.subscribe();

        Self {
            store: self.store.clone(),
            web_ch: (tx, rx),
        }
    }
}

#[tonic::async_trait]
pub trait WebDispatch: Clone + Send + Sync + 'static {
    async fn emit(&self, event: Event) -> Result<(), anyhow::Error>;
    async fn receive(&mut self) -> Result<Event, anyhow::Error>;
}

#[tonic::async_trait]
impl WebDispatch for Dispatch {
    // using "emit" here - we might need a "send" in the future where we need a response payload
    async fn emit(&self, event: Event) -> Result<(), anyhow::Error> {
        match event {
            _ => {
                if let Err(e) = self.web_ch.0.send(event) {
                    tracing::error!("failed to dispatch send event: {:?}", e);
                    return Err(e.into());
                }
                Ok(())
            }
        }
    }

    async fn receive(&mut self) -> Result<Event, anyhow::Error> {
        let event = self.web_ch.1.recv().await?;
        Ok(event)
    }
}

impl Dispatch {
    pub async fn new() -> Self {
        Self {
            store: store::RedisStore::new().await,
            web_ch: broadcast::channel(65536),
        }
    }

    pub async fn start(&mut self) {
        loop {
            let event = self.receive().await;
            if let Err(e) = event {
                tracing::error!("dispatch: failed to receive event: {:?}", e);
                return;
            }
            if let Err(e) = self.handle_event(event.unwrap()).await {
                tracing::error!("dispatch: handling failed: {:?}", e);
            }
        }
    }

    async fn handle_event(&mut self, event: Event) -> Result<(), anyhow::Error> {
        match event {
            Event::TraceReceived(trace) => {
                self.store.store_trace(trace).await?;
            }
            Event::SourceReceived(scan) => {
                self.store.store_source_scan(&scan).await?;
                self.emit(Event::SourceDataAvailable(scan)).await?;
            }
            Event::RequestSourceData => {
                let scan = self.store.get_source_scan().await?;
                self.emit(Event::SourceDataAvailable(scan)).await?;
            }
            _ => {}
        };
        Ok(())
    }
}
