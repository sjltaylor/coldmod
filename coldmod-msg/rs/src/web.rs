use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Msg {
    RequestSourceData,
    SourceDataAvailable(Option<crate::proto::SourceScan>),
    TraceReceived(crate::proto::Trace),
    SourceReceived(crate::proto::SourceScan),
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Msg::RequestSourceData => write!(f, "RequestSourceData"),
            Msg::SourceDataAvailable(_) => write!(f, "SourceDataAvailable"),
            Msg::TraceReceived(_) => write!(f, "TraceReceived"),
            Msg::SourceReceived(_) => write!(f, "SourceReceived"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Heatmap {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatmapElement {}
