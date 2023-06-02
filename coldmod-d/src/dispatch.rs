use crate::store;
use coldmod_msg::web::Msg;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct Dispatch {
    store: store::RedisStore,
    broadcast: broadcast::Sender<Msg>,
}

#[tonic::async_trait]
pub trait WebDispatch: Clone + Send + Sync + 'static {
    async fn emit(&self, event: Msg) -> Result<(), anyhow::Error>;
    async fn receive(&mut self) -> Result<Msg, anyhow::Error>;
}

#[tonic::async_trait]
impl WebDispatch for Dispatch {
    // using "emit" here - we might need a "send" in the future where we need a response payload
    async fn emit(&self, _event: Msg) -> Result<(), anyhow::Error> {
        // match event {
        //     _ => {
        //         if let Err(e) = self.web_ch.0.send(event) {
        //             tracing::error!("failed to dispatch send event: {:?}", e);
        //             return Err(e.into());
        //         }
        //         Ok(())
        //     }
        // }
        todo!("implement")
    }

    async fn receive(&mut self) -> Result<Msg, anyhow::Error> {
        // let event = self.web_ch.1.recv().await?;
        // Ok(event)
        //retrun an error
        todo!("implement")
    }
}

impl Dispatch {
    pub async fn new() -> Self {
        let sender = broadcast::channel(65536).0;
        Self {
            store: store::RedisStore::new().await,
            broadcast: sender,
        }
    }

    pub async fn handle(&self, msg: Msg) -> Result<Option<Msg>, anyhow::Error> {
        let mut store = self.store.clone();

        match msg {
            Msg::TraceReceived(trace) => {
                store.store_trace(trace).await?;
            }
            Msg::SourceReceived(scan) => {
                store.store_source_scan(&scan).await?;
                self._broadcast(Msg::SourceDataAvailable(Some(scan)));
            }
            Msg::RequestSourceData => {
                let scan = store.get_source_scan().await?;
                return Ok(Msg::SourceDataAvailable(scan).into());
            }
            _ => {}
        };

        Ok(None)
    }

    pub fn receiver(&self) -> broadcast::Receiver<Msg> {
        self.broadcast.subscribe()
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
