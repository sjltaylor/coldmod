use std::{collections::HashMap, fmt::Display, ops::Range};

static FILTER_OPTIONS: [&str; 6] = ["COLD", "P10", "P20", "P40", "P90", "HOT"];
static FILTER_GROUPS: [(usize, usize); 2] = [(0, 1), (1, 6)];

const _SZ: usize = 6;
const _iSZ: i32 = _SZ as i32;

#[derive(Debug, Copy, Clone)]
pub struct FilterState {
    names: [&'static str; _SZ],
    range: (i32, i32),
    cold: bool,
}

impl Default for FilterState {
    fn default() -> Self {
        let names = ["COLD", "P10", "P20", "P40", "P90", "HOT"];
        let range = (0, 0);
        let r = 0..0;
        let cold = true;
        Self { names, range, cold }
    }
}

impl FilterState {
    pub fn keys(&self) -> Vec<&'static str> {
        return FILTER_OPTIONS.to_vec();
    }
    pub fn groups(&self) -> [(usize, usize); 2] {
        return FILTER_GROUPS;
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
    pub fn toggle(&mut self, key: &'static str) {
        let idx = self.idx(key);

        let (mut start, mut end) = self.range;
        let hot_idx = _iSZ - 1;

        if start == end && idx == hot_idx {
            self.cold = false;
            end = _iSZ;
        }

        if self.cold {
            if idx == 0 && end == 1 {
                end = 0;
                self.cold = true;
            } else if idx == end - 1 {
                start = 0;
                end = 0;
                self.cold = true;
            } else {
                end = idx + 1;
            }
        } else {
            if idx == hot_idx && start == hot_idx {
                start = 0;
                end = 0;
                self.cold = true;
            } else if idx == start {
                start = 0;
                end = 0;
                self.cold = true;
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
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        s.toggle("COLD");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), false);
    }

    #[test]
    fn test_increasing_cold_toggle() {
        let mut s = FilterState::default();
        s.toggle("P10");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P20");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P40");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P90");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), false);
        s.toggle("HOT");
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
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P40");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P20");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P10");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("COLD");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("COLD");
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
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P90");
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P40");
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P20");
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P10");
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("COLD");
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
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P40");
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("HOT");
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), true);
        s.toggle("HOT");
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
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P40");
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
        s.toggle("P40");
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
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
        s.toggle("P20");
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
        assert_eq!(s.get("P40"), false);
        assert_eq!(s.get("P90"), false);
        assert_eq!(s.get("HOT"), false);
    }
}
