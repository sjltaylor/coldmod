use crate::proto::{src_message::PossibleSrcMessage, HeatMap, ModCommand};
use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Msg {
    HeatMapAvailable(HeatMap),
    HeatMapChanged(HeatMapDelta),
    TracingStatsAvailable(TracingStats),
    ModCommandClientAvailable,
    ModCommandClientUnavailable,
    RouteModCommand(ModCommand),
    RouteModCommandTo((ModCommand, String)),
    SrcMessage(PossibleSrcMessage),
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Msg::HeatMapAvailable(_) => write!(f, "SourceDataAvailable"),
            Msg::TracingStatsAvailable(_) => write!(f, "TracingStatsAvailable"),
            Msg::HeatMapChanged(_) => write!(f, "HeatMapChanged"),
            Msg::ModCommandClientAvailable => write!(f, "ModCommandClientAvailable"),
            Msg::ModCommandClientUnavailable => write!(f, "ModCommandClientUnavailable"),
            Msg::RouteModCommand(_) => write!(f, "RouteModCommand"),
            Msg::RouteModCommandTo(_) => write!(f, "RouteModCommandTo"),
            Msg::SrcMessage(_) => write!(f, "SrcMessage"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TracingStats {
    pub count: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatMapDelta {
    pub deltas: HashMap<String, i64>,
}
