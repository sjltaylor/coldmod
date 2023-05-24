use crate::store;
use coldmod_msg::proto::{SourceScan, Trace};

#[derive(Clone)]
pub struct Dispatch {
    pub store: store::RedisStore,
    pub trace_ch: (async_channel::Sender<Trace>, async_channel::Receiver<Trace>),
}

#[tonic::async_trait]
impl crate::grpc::DispatchContext for Dispatch {
    async fn store_source_scan(&self, scan: SourceScan) -> Result<(), Box<dyn std::error::Error>> {
        self.store.store_source_scan(scan).await?;
        Ok(())
    }

    fn emit_trace(&self, trace: Trace) -> Result<(), Box<dyn std::error::Error>> {
        match self.trace_ch.0.try_send(trace) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl crate::web::DispatchContext for Dispatch {
    fn receiver(&self) -> async_channel::Receiver<Trace> {
        self.trace_ch.1.clone()
    }
}

impl crate::store::DispatchContext for Dispatch {
    fn get_redis_connection_info(&self) -> redis::ConnectionInfo {
        self.store.connection_info.clone()
    }

    fn trace_receiver(&self) -> async_channel::Receiver<Trace> {
        self.trace_ch.1.clone()
    }
}
