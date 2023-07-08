use std::{collections::HashMap, fmt::Display};

static FILTER_OPTIONS: [&str; 6] = ["COLD", "P10", "P20", "P40", "P90", "HOT"];
static FILTER_GROUPS: [(usize, usize); 2] = [(0, 1), (1, 6)];

const _SZ: usize = 6;

#[derive(Debug, Copy, Clone)]
pub struct FilterState {
    names: [&'static str; _SZ],
    range: (usize, usize),
}

impl Default for FilterState {
    fn default() -> Self {
        let names = ["COLD", "P10", "P20", "P40", "P90", "HOT"];
        let range = (0, 0);
        Self { names, range }
    }
}

impl FilterState {
    pub fn keys(&self) -> Vec<&'static str> {
        return FILTER_OPTIONS.to_vec();
    }
    pub fn groups(&self) -> [(usize, usize); 2] {
        return FILTER_GROUPS;
    }

    fn idx(&self, key: &str) -> usize {
        self.names
            .iter()
            .position(|k| *k == key)
            .expect("key not valid")
    }
    fn state_at(&self, idx: usize) -> bool {
        let (a, b) = self.range;
        idx >= a && idx < b
    }
    pub fn get(&self, key: &'static str) -> bool {
        self.state_at(self.idx(key))
    }
    pub fn toggle(&mut self, key: &'static str) -> bool {
        let idx = self.idx(key);
        let value = self.state_at(idx);
        let new_value = !value;

        let (a, mut b) = self.range;

        if idx == 0 && b == 1 {
            b = 0
        } else {
            b = idx + 1
        }

        self.range = (a, b);

        new_value
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
    fn test_sliding_toggle_from_default() {
        let mut s = FilterState::default();
        assert_eq!(s.get("COLD"), false);
        assert_eq!(s.get("P10"), false);
        assert_eq!(s.get("P20"), false);
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
        s.toggle("HOT");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
    }

    #[test]
    fn test_increasing_toggle() {
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
    fn test_decreasing_toggle() {
        let mut s = FilterState::default();
        s.toggle("HOT");
        assert_eq!(s.get("COLD"), true);
        assert_eq!(s.get("P10"), true);
        assert_eq!(s.get("P20"), true);
        assert_eq!(s.get("P40"), true);
        assert_eq!(s.get("P90"), true);
        assert_eq!(s.get("HOT"), true);
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
}
