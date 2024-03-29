use std::collections::BTreeMap;

use crate::filter_state::FilterState;
use coldmod_msg::proto::{HeatMap, HeatSrc};
use coldmod_msg::web::HeatMapDelta;

#[derive(Clone)]
pub struct HeatmapFilter {
    pub filter_state: FilterState,
    pub heatmap: HeatMap,
}

impl HeatmapFilter {
    pub fn heat_srcs(&self) -> Vec<HeatSrc> {
        let mut not_cold: BTreeMap<i64, Vec<&HeatSrc>> = BTreeMap::new();
        let mut not_cold_count = 0;

        let mut cold = Vec::<&HeatSrc>::new();

        for source in self.heatmap.srcs.iter() {
            let trace_count = source.trace_count;
            if trace_count == 0 {
                cold.push(source);
                continue;
            }
            let bucket = not_cold
                .entry(trace_count)
                .or_insert(Vec::<&HeatSrc>::new());
            bucket.push(source);
            not_cold_count += 1;
        }

        let (include_zero, p_lower, p_upper) = self.filter_state.selection();
        let (i_lower, i_upper) = (
            (not_cold_count as f64 * p_lower).floor() as usize,
            (not_cold_count as f64 * p_upper).ceil() as usize,
        );

        let mut buckets: Vec<&Vec<&HeatSrc>> = Vec::new();

        let mut i = 0;

        for bucket in not_cold.values() {
            let j = i + bucket.len();
            if j >= i_lower && i < i_upper {
                buckets.push(bucket);
            }

            i = j;
        }

        let base: Box<dyn Iterator<Item = &Vec<&HeatSrc>>> = if include_zero {
            Box::new(vec![&cold].into_iter().chain(buckets.into_iter()))
        } else {
            Box::new(buckets.into_iter())
        };

        let mut selection = base
            .into_iter()
            .flatten()
            .map(|s| (*s).clone())
            .collect::<Vec<_>>();

        if !self.filter_state.is_ascending() {
            selection.reverse();
        }

        selection
    }
    pub fn update(&mut self, heatmap_delta: &HeatMapDelta) {
        // this doesn't need to be a loop, but we haven't created the data structure to look up the heat source
        // so for now there's a simple optimization - loop from hottest first because that's most likely
        for source in self.heatmap.srcs.iter_mut().rev() {
            let k = source.key.clone();

            if let Some(delta) = heatmap_delta.deltas.get(&k) {
                source.trace_count += *delta;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_trace_counts(filter: &HeatmapFilter) -> Vec<i64> {
        filter
            .heat_srcs()
            .iter()
            .map(|s| s.trace_count)
            .collect::<Vec<i64>>()
    }

    fn heatmap_sample_1() -> HeatMap {
        let mut sources = Vec::<HeatSrc>::new();
        let vs: Vec<i64> = vec![1, 6, 7, 3, 4, 0, 2, 8, 9, 10, 5];

        for i in vs.iter() {
            sources.push(HeatSrc {
                key: "STUB_KEY".to_string(),
                trace_count: *i,
            });
        }

        HeatMap { srcs: sources }
    }

    fn heatmap_sample_2() -> HeatMap {
        let mut sources = Vec::<HeatSrc>::new();

        let vs: Vec<i64> = vec![1, 5, 2, 0, 6, 324, 0, 4, 23, 166, 0];

        for i in vs.iter() {
            sources.push(HeatSrc {
                key: "STUB_KEY".to_string(),
                trace_count: *i,
            });
        }

        HeatMap { srcs: sources }
    }

    fn heatmap_sample_3() -> HeatMap {
        let mut sources = Vec::<HeatSrc>::new();
        for _ in 0..10 {
            sources.push(HeatSrc {
                key: "STUB_KEY".to_string(),
                trace_count: 5,
            });
        }

        HeatMap { srcs: sources }
    }

    #[test]
    fn test_sort_order() {
        let heatmap = heatmap_sample_1();
        let mut filter = HeatmapFilter {
            heatmap,
            filter_state: FilterState::default(),
        };

        let trace_counts = collect_trace_counts(&filter);

        assert_eq!(trace_counts, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        filter.filter_state.toggle("HOT");
        filter.filter_state.toggle("COLD");

        let sources = filter
            .heat_srcs()
            .iter()
            .map(|s| s.trace_count)
            .collect::<Vec<i64>>();

        let mut expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        expected.reverse();

        assert_eq!(sources, expected);
    }

    #[test]
    fn test_cold_only() {
        let heatmap = heatmap_sample_2();
        let mut filter = HeatmapFilter {
            heatmap,
            filter_state: FilterState::default(),
        };

        filter.filter_state.toggle("COLD");

        let trace_counts = collect_trace_counts(&filter);

        assert_eq!(trace_counts, vec![0, 0, 0]);
    }

    #[test]
    fn test_cold_p10() {
        let heatmap = heatmap_sample_2();
        let mut filter = HeatmapFilter {
            heatmap,
            filter_state: FilterState::default(),
        };

        filter.filter_state.toggle("COLD");
        filter.filter_state.toggle("P10");

        let trace_counts = collect_trace_counts(&filter);

        assert_eq!(trace_counts, vec![0, 0, 0, 1]);
    }

    #[test]
    fn test_cold_p40() {
        let heatmap = heatmap_sample_2();
        let mut filter = HeatmapFilter {
            heatmap,
            filter_state: FilterState::default(),
        };

        filter.filter_state.toggle("COLD");
        filter.filter_state.toggle("P40");

        let trace_counts = collect_trace_counts(&filter);

        assert_eq!(trace_counts, vec![0, 0, 0, 1, 2, 4, 5]);
    }

    #[test]
    fn test_p10_hot() {
        let heatmap = heatmap_sample_2();
        let mut filter = HeatmapFilter {
            heatmap,
            filter_state: FilterState::default(),
        };

        filter.filter_state.toggle("HOT");
        filter.filter_state.toggle("P10");

        let trace_counts = collect_trace_counts(&filter);

        assert_eq!(
            trace_counts,
            vec![1, 2, 4, 5, 6, 23, 166, 324]
                .into_iter()
                .rev()
                .collect::<Vec<i64>>()
        );
    }

    #[test]
    fn test_p40_hot() {
        let heatmap = heatmap_sample_2();
        let mut filter = HeatmapFilter {
            heatmap,
            filter_state: FilterState::default(),
        };

        filter.filter_state.toggle("HOT");
        filter.filter_state.toggle("P40");

        let trace_counts = collect_trace_counts(&filter);

        assert_eq!(
            trace_counts,
            vec![4, 5, 6, 23, 166, 324]
                .into_iter()
                .rev()
                .collect::<Vec<i64>>()
        );
    }

    #[test]
    fn test_p10_p40_same_value() {
        let heatmap = heatmap_sample_3();
        let mut filter = HeatmapFilter {
            heatmap,
            filter_state: FilterState::default(),
        };

        filter.filter_state.toggle("P40");
        filter.filter_state.toggle("COLD");

        let trace_counts = collect_trace_counts(&filter);

        assert_eq!(trace_counts, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_cold_hot() {
        let mut sources = Vec::<HeatSrc>::new();

        for i in 0..21 {
            sources.push(HeatSrc {
                key: "STUB_KEY".to_string(),
                trace_count: i,
            });
        }

        let heatmap = HeatMap { srcs: sources };

        let mut filter = HeatmapFilter {
            heatmap,
            filter_state: FilterState::default(),
        };

        filter.filter_state.toggle("COLD");
        filter.filter_state.toggle("HOT");

        assert_eq!(21, filter.heat_srcs().len());
    }

    #[test]
    fn test_p10_p40() {
        let mut sources = Vec::<HeatSrc>::new();

        for i in 0..21 {
            sources.push(HeatSrc {
                key: "STUB_KEY".to_string(),
                trace_count: i,
            });
        }

        let heatmap = HeatMap { srcs: sources };

        let mut filter = HeatmapFilter {
            heatmap,
            filter_state: FilterState::default(),
        };

        filter.filter_state.toggle("P40");
        filter.filter_state.toggle("COLD");

        assert_eq!(8, filter.heat_srcs().len());
    }
}
