use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {
    RequestSourceData,
    SourceDataAvailable(Option<crate::proto::SourceScan>),
    TraceReceived(crate::proto::Trace),
    SourceReceived(crate::proto::SourceScan),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Heatmap {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatmapElement {}
