use crate::proto::{TraceSrc, TraceSrcs};
use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Msg {
    HeatMapAvailable(HeatMap),
    HeatMapChanged(HeatMapDelta),
    TracingStatsAvailable(TracingStats),
    SetFilterSet((TraceSrcs, String)),
    SetFilterSetInContext(TraceSrcs),
    SendYourFilterSet,
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Msg::HeatMapAvailable(_) => write!(f, "SourceDataAvailable"),
            Msg::TracingStatsAvailable(_) => write!(f, "TracingStatsAvailable"),
            Msg::HeatMapChanged(_) => write!(f, "HeatMapChanged"),
            Msg::SetFilterSet(_) => write!(f, "SetFilterSet"),
            Msg::SetFilterSetInContext(_) => write!(f, "SetFilterSetInContext"),
            Msg::SendYourFilterSet => write!(f, "SendYourFilterSet"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TracingStats {
    pub count: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatMap {
    pub srcs: Vec<HeatSrc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatSrc {
    pub trace_src: TraceSrc,
    pub trace_count: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatMapDelta {
    pub deltas: HashMap<String, i64>,
}
