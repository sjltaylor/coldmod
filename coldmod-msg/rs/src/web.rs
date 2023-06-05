use crate::proto::{SourceScan, Trace};
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Msg {
    AppSocketConnected,
    RequestSourceData,
    SourceDataAvailable(Option<SourceScan>),
    TraceReceived(Trace),
    SourceReceived(SourceScan),
    TracingStatsAvailable(TracingStats),
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Msg::AppSocketConnected => write!(f, "AppSocketConnected"),
            Msg::RequestSourceData => write!(f, "RequestSourceData"),
            Msg::SourceDataAvailable(_) => write!(f, "SourceDataAvailable"),
            Msg::TraceReceived(_) => write!(f, "TraceReceived"),
            Msg::SourceReceived(_) => write!(f, "SourceReceived"),
            Msg::TracingStatsAvailable(_) => write!(f, "TracingStatsAvailable"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TracingStats {
    pub count: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Heatmap {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatmapElement {}
