const _SZ: usize = 6;
const _ISZ32: i32 = _SZ as i32;

#[derive(Debug, Copy, Clone)]
pub struct FilterState {
    names: [&'static str; _SZ],
    range: (i32, i32),
    ascending: bool,
}

impl Default for FilterState {
    fn default() -> Self {
        let names = ["COLD", "P10", "P20", "P40", "P90", "HOT"];
        let range = (0, 0);
        let ascending = true;
        Self {
            names,
            range,
            ascending,
        }
    }
}

impl FilterState {
    pub fn keys(&self) -> Vec<&'static str> {
        return self.names.to_vec();
    }

    fn idx(&self, key: &str) -> i32 {
        self.names
            .iter()
            .position(|k| *k == key)
            .expect("key not valid") as i32
    }
    fn state_at(&self, idx: i32) -> bool {
        let (a, b) = self.range;
        idx >= a && idx < b
    }
    pub fn get(&self, key: &'static str) -> bool {
        self.state_at(self.idx(key))
    }
    pub fn is_ascending(&self) -> bool {
        self.ascending
    }
    pub fn selection(&self) -> (bool, f64, f64) {
        let (a, b) = self.range;

        if b == 0 {
            return (true, 0.0, 1.0);
        }

        let a_p_values_f = [0.0, 0.1, 0.2, 0.4, 0.9, 0.95];
        let b_p_values_f = [0.0, 0.1, 0.2, 0.4, 0.9, 1.0];

        if self.ascending {
            (a == 0, 0.0, b_p_values_f[(b - 1) as usize])
        } else {
            (a == 0, a_p_values_f[a as usize], 1.0)
        }
    }
    pub fn toggle(&mut self, key: &'static str) {
        let idx = self.idx(key);

        let (mut start, mut end) = self.range;
        let hot_idx = _ISZ32 - 1;

        if start == end && idx == hot_idx {
            self.ascending = false;
            end = _ISZ32;
        }

        if self.ascending {
            if idx == 0 && end > 1 {
                if start == 1 {
                    start = 0;
                } else {
                    start = 1;
                }
            } else if idx == 0 && end == 1 {
                end = 0;
                self.ascending = true;
            } else if idx == end - 1 {
                start = 0;
                end = 0;
                self.ascending = true;
            } else {
                end = idx + 1;
            }
        } else {
            if idx == 0 && start == 0 {
                start = 1;
            } else if idx == start || (idx == hot_idx && start == hot_idx) {
                start = 0;
                end = 0;
                self.ascending = true;
            } else {
                start = idx
            }
        }

        self.range = (start, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle() {
        let mut s = FilterState::default();
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        s.toggle("COLD");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), false);
    }

    #[test]
    fn test_increasing_cold_toggle() {
        let mut s = FilterState::default();
        s.toggle("P10");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P20");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P40");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P90");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), false);
        s.toggle("HOT");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
    }

    #[test]
    fn test_decreasing_cold_toggle() {
        let mut s = FilterState::default();
        s.toggle("P90");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P40");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P20");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P10");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("COLD");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P10");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
    }

    #[test]
    fn test_increasing_hot_toggle() {
        let mut s = FilterState::default();
        s.toggle("HOT");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P90");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P40");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P20");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P10");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("COLD");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
    }

    #[test]
    fn test_decreasing_hot_toggle() {
        let mut s = FilterState::default();
        s.toggle("HOT");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P40");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("HOT");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), true);
        s.toggle("HOT");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
    }

    #[test]
    fn test_clearing_a_hot_toggle() {
        let mut s = FilterState::default();
        s.toggle("HOT");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P40");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P40");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
    }

    #[test]
    fn test_clearing_a_cold_toggle() {
        let mut s = FilterState::default();
        s.toggle("P20");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P20");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
    }

    #[test]
    fn test_toggle_cold_when_cold() {
        let mut s = FilterState::default();
        s.toggle("P40");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("COLD");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("COLD");
        assert_eq!(s.is_ascending(), true);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
    }

    #[test]
    fn test_toggle_cold_when_hot() {
        let mut s = FilterState::default();
        s.toggle("HOT");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), true);
        s.toggle("COLD");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("COLD");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("COLD");
        assert_eq!(s.is_ascending(), false);
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
    }

    #[test]
    fn test_percentile_range() {
        let mut s = FilterState::default();
        assert_eq!(s.selection(), (true, 0.0, 1.0));

        s.toggle("COLD");
        assert_eq!(s.selection(), (true, 0.0, 0.0));

        s.toggle("COLD");
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.selection(), (true, 0.0, 1.0));

        s.toggle("P40");
        assert_eq!(s.selection(), (true, 0.0, 0.4));

        s.toggle("COLD");
        assert_eq!(s.selection(), (false, 0.0, 0.4));
        let mut s = FilterState::default();

        s.toggle("HOT");
        assert_eq!(s.selection(), (false, 0.95, 1.0));

        s.toggle("P90");
        assert_eq!(s.selection(), (false, 0.9, 1.0));

        s.toggle("P40");
        assert_eq!(s.selection(), (false, 0.4, 1.0));
    }

    #[test]
    fn test_percentile_range_full() {
        let mut s = FilterState::default();
        assert_eq!(s.selection(), (true, 0.0, 1.0));

        s.toggle("COLD");
        s.toggle("HOT");
        assert_eq!(s.selection(), (true, 0.0, 1.0));
    }

    #[test]
    fn test_percentile_range_cold_p90() {
        let mut s = FilterState::default();
        assert_eq!(s.selection(), (true, 0.0, 1.0));

        s.toggle("COLD");
        s.toggle("P90");
        assert_eq!(s.selection(), (true, 0.0, 0.90));
    }
}
