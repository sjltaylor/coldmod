use std::collections::BTreeMap;

use crate::filter_state::FilterState;
use coldmod_msg::web::HeatMap;
use coldmod_msg::web::HeatSource;
/*
    takes a heat map and groups it into buckets according to filter keys
    provides an iterator which respects a filter state
*/

#[derive(Clone)]
pub struct HeatmapFilter {
    pub filter_state: FilterState,
    pub heatmap: HeatMap,
}

impl HeatmapFilter {
    pub fn sources(&self) -> Vec<HeatSource> {
        let mut not_cold: BTreeMap<i64, Vec<&HeatSource>> = BTreeMap::new();
        let mut not_cold_count = 0;

        let mut cold = Vec::<&HeatSource>::new();

        for source in self.heatmap.sources.iter() {
            let trace_count = source.trace_count;
            if trace_count == 0 {
                cold.push(source);
                continue;
            }
            let bucket = not_cold
                .entry(trace_count)
                .or_insert(Vec::<&HeatSource>::new());
            bucket.push(source);
            not_cold_count += 1;
        }

        let (include_zero, p_lower, p_upper) = self.filter_state.selection();
        let (i_lower, i_upper) = (
            (not_cold_count as f64 * p_lower).floor() as usize,
            (not_cold_count as f64 * p_upper).ceil() as usize,
        );

        let mut buckets: Vec<&Vec<&HeatSource>> = Vec::new();

        let _all = not_cold
            .clone()
            .values()
            .into_iter()
            .flatten()
            .map(|s| s.trace_count)
            .collect::<Vec<_>>();

        let mut i = 0;

        for bucket in not_cold.values() {
            let j = i + bucket.len();
            if j >= i_lower && i < i_upper {
                buckets.push(bucket);
            }

            i = j;
        }

        let base: Box<dyn Iterator<Item = &Vec<&HeatSource>>> = if include_zero {
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
}

#[cfg(test)]
mod tests {

    use coldmod_msg::proto::SourceElement;

    use super::*;

    fn collect_trace_counts(filter: &HeatmapFilter) -> Vec<i64> {
        filter
            .sources()
            .iter()
            .map(|s| s.trace_count)
            .collect::<Vec<i64>>()
    }

    fn heatmap_sample_1() -> HeatMap {
        let mut sources = Vec::<HeatSource>::new();
        let source_element = SourceElement { elem: None };

        let vs: Vec<i64> = vec![1, 6, 7, 3, 4, 0, 2, 8, 9, 10, 5];

        for i in vs.iter() {
            sources.push(HeatSource {
                source_element: source_element.clone(),
                trace_count: *i,
            });
        }

        HeatMap { sources }
    }

    fn heatmap_sample_2() -> HeatMap {
        let mut sources = Vec::<HeatSource>::new();
        let source_element = SourceElement { elem: None };

        let vs: Vec<i64> = vec![1, 5, 2, 0, 6, 324, 0, 4, 23, 166, 0];

        for i in vs.iter() {
            sources.push(HeatSource {
                source_element: source_element.clone(),
                trace_count: *i,
            });
        }

        HeatMap { sources }
    }

    fn heatmap_sample_3() -> HeatMap {
        let mut sources = Vec::<HeatSource>::new();
        let source_element = SourceElement { elem: None };
        for _ in 0..10 {
            sources.push(HeatSource {
                source_element: source_element.clone(),
                trace_count: 5,
            });
        }

        HeatMap { sources }
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
            .sources()
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

    /*
        all elements with the same trace_count should be in the same PXX block (greedy, lowest first)
        cold elements are excluded from the binning distribution

        test trace counts changing value accross bucket boundaries
            * e.g something that was in P10 changes to become in P20
            * something moves from cold to P10
            * something moves from P90 to HOT
            * something moves from P40 to P90 when the filter state is hot


    */
}
