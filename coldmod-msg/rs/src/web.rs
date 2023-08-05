use crate::proto::{FilterSet, FilterSetQuery, Trace, TraceSrc, TraceSrcs};
use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Msg {
    Reset,
    TraceReceived(Trace),
    SetTraceSrcs(TraceSrcs),
    HeatMapAvailable(HeatMap),
    HeatMapChanged(HeatMapDelta),
    TracingStatsAvailable(TracingStats),
    SetFilterSet((FilterSet, String)),
    SetFilterSetInContext(FilterSet),
    GetFilterSet(FilterSetQuery),
    FilterSetAvailable(FilterSet),
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Msg::HeatMapAvailable(_) => write!(f, "SourceDataAvailable"),
            Msg::TraceReceived(_) => write!(f, "TraceReceived"),
            Msg::SetTraceSrcs(_) => write!(f, "SourceReceived"),
            Msg::TracingStatsAvailable(_) => write!(f, "TracingStatsAvailable"),
            Msg::HeatMapChanged(_) => write!(f, "HeatMapChanged"),
            Msg::Reset => write!(f, "Reset"),
            Msg::SetFilterSet(_) => write!(f, "SetFilterSet"),
            Msg::GetFilterSet(_) => write!(f, "GetFilterSet"),
            Msg::FilterSetAvailable(_) => write!(f, "FilterSetAvailable"),
            Msg::SetFilterSetInContext(_) => write!(f, "SetFilterSetInContext"),
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
