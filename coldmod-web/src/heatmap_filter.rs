use crate::filter_state::FilterState;
use coldmod_msg::web::HeatMap;
use coldmod_msg::web::HeatSource;
/*
    takes a heat map and groups it into buckets according to filter keys
    provides an iterator which respects a filter state
*/

struct HeatmapFilter {
    filter_state: FilterState,
    heatmap: HeatMap,
}

impl HeatmapFilter {
    fn sources(&self) -> Vec<HeatSource> {
        if self.filter_state.is_ascending() {
            return self.heatmap.sources.clone();
        }
        self.heatmap.sources.clone().into_iter().rev().collect()
    }
    fn set_state(&mut self, filter_state: FilterState) {
        self.filter_state = filter_state;
    }
}

#[cfg(test)]
mod tests {

    use coldmod_msg::proto::SourceElement;

    use super::*;

    fn heatmap_sample_1() -> HeatMap {
        let mut sources = Vec::<HeatSource>::new();
        let source_element = SourceElement { elem: None };

        for i in 0..=10 {
            sources.push(HeatSource {
                source_element: source_element.clone(),
                trace_count: i,
            });
        }

        HeatMap { sources }
    }

    #[test]
    fn test_coldest_first() {
        let heatmap = heatmap_sample_1();
        let mut filter_state = FilterState::default();
        let mut filter = HeatmapFilter {
            heatmap,
            filter_state,
        };

        let sources = filter
            .sources()
            .iter()
            .map(|s| s.trace_count)
            .collect::<Vec<i64>>();

        assert_eq!(sources, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        filter_state.toggle("HOT");
        filter_state.toggle("COLD");

        filter.set_state(filter_state);

        let sources = filter
            .sources()
            .iter()
            .map(|s| s.trace_count)
            .collect::<Vec<i64>>();

        let mut expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        expected.reverse();

        assert_eq!(sources, expected);

        // implement this and the rework the API w/ the UI

        // test the iteration
    }

    /*
        all elements with the same trace_count should be in the same PXX block (greedy, lowest first)
        cold elements are excluded from the binning distribution

        test sort order both ways
        test different filter states
            * all off
            * all on except cold
            * all on
            * only cold
            * cold -> P10
            * cold -> P40
            * P10-hot
            * P40-hot
            * P90-hot
        test trace counts changing value accross bucket boundaries
            * e.g something that was in P10 changes to become in P20
            * something moves from cold to P10
            * something moves from P90 to HOT
            * something moves from P40 to P90 when the filter state is hot


    */
}
