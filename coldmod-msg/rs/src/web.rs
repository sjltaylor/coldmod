use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {
    RequestSourceData,
    SourceDataAvailable,
    #[cfg(feature = "proto")]
    TraceReceived(crate::proto::Trace),
    #[cfg(feature = "proto")]
    SourceReceived(crate::proto::SourceScan),
}
