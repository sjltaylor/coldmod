use crate::store;
use coldmod_msg::web::Event;

#[derive(Clone)]
pub struct Dispatch {
    pub store: store::RedisStore,
    pub web_ch: (async_channel::Sender<Event>, async_channel::Receiver<Event>),
}

#[tonic::async_trait]
pub trait WebDispatch: Clone + Send + Sync + 'static {
    async fn emit(&self, event: Event) -> Result<(), anyhow::Error>;
    async fn receive(&self) -> Result<Event, anyhow::Error>;
}

#[tonic::async_trait]
impl WebDispatch for Dispatch {
    // using "emit" here - we might need a "send" in the future where we need a response payload
    async fn emit(&self, event: Event) -> Result<(), anyhow::Error> {
        match event {
            _ => {
                if let Err(e) = self.web_ch.0.send(event).await {
                    tracing::error!("failed to dispatch send event: {:?}", e);
                    return Err(e.into());
                }
                Ok(())
            }
        }
    }

    async fn receive(&self) -> Result<Event, anyhow::Error> {
        let event = self.web_ch.1.recv().await?;
        Ok(event)
    }
}

impl Dispatch {
    pub async fn new() -> Self {
        Self {
            store: store::RedisStore::new().await,
            web_ch: async_channel::bounded(65536),
        }
    }

    pub async fn start(&mut self) {
        loop {
            let event = self.receive().await;
            if let Err(e) = event {
                tracing::error!("dispatch: failed to receive event: {:?}", e);
                return;
            }
            let result = match event.unwrap() {
                Event::TraceReceived(trace) => {
                    println!("dispatch: received trace: {:?}", trace);
                    self.store.store_trace(trace).await
                }
                Event::SourceReceived(scan) => {
                    println!("dispatch: received scan: {:?}", scan);
                    self.store.store_source_scan(scan).await
                }
                Event::RequestSourceData => {
                    println!("REQUEST SOURCE DATA");
                    Ok(())
                }
                _ => Ok(()),
            };
            if let Err(e) = result {
                tracing::error!("dispatch: handling failed: {:?}", e);
            }
        }
    }
}
