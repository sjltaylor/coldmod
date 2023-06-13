use crate::proto::{SourceElement, SourceScan, Trace};
use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Msg {
    Reset,
    TraceReceived(Trace),
    SourceReceived(SourceScan),
    HeatMapAvailable(HeatMap),
    HeatMapChanged(HeatMapDelta),
    TracingStatsAvailable(TracingStats),
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Msg::HeatMapAvailable(_) => write!(f, "SourceDataAvailable"),
            Msg::TraceReceived(_) => write!(f, "TraceReceived"),
            Msg::SourceReceived(_) => write!(f, "SourceReceived"),
            Msg::TracingStatsAvailable(_) => write!(f, "TracingStatsAvailable"),
            Msg::HeatMapChanged(_) => write!(f, "HeatMapChanged"),
            Msg::Reset => write!(f, "Reset"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TracingStats {
    pub count: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatMap {
    pub sources: Vec<HeatSource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatMapDelta {
    pub deltas: HashMap<String, i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatSource {
    pub source_element: SourceElement,
    pub trace_count: i64,
}

pub trait ElementKey {
    fn key(&self) -> String;
}

impl ElementKey for SourceElement {
    fn key(&self) -> String {
        match self.elem.as_ref().expect("source element to be present") {
            crate::proto::source_element::Elem::Fn(f) => format!("{}:{}", f.path, f.line),
        }
    }
}

impl Trace {
    pub fn key(&self, coldmod_root_marker_prefix: impl Into<String>) -> String {
        format!(
            "{}/{}:{}",
            coldmod_root_marker_prefix.into(),
            self.path,
            self.line
        )
    }
}
