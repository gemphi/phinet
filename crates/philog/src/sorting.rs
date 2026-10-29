//! Sorting as a reasoning test (Ch 14.4 example).
//! A program generalizes to any list size; a neural net doesn't.
//! Phiano's phase model should sort by phase angle reliably.

use phiton::units::TWO_PI;
use phiecs::phasor::facet::Facet;

#[derive(Debug, Default)]
pub struct SortingTest;

impl SortingTest {
    /// Sorts words by their phase angle.
    pub fn by_phase(facet: &Facet, words: &[String]) -> Vec<String> {
        let mut indexed: Vec<(String, f64)> = words
            .iter()
            .filter_map(|w| facet.lexicon.get(w).map(|p| (w.clone(), p.phase)))
            .collect();
        indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        indexed.into_iter().map(|(w, _)| w).collect()
    }

    /// Tests if the model can sort arbitrary-length lists.
    /// Returns true if sorting produces a monotonic phase sequence.
    pub fn check(facet: &Facet) -> bool {
        let words: Vec<String> = facet.lexicon.keys().take(32).cloned().collect();
        if words.len() < 3 {
            return false;
        }

        let sorted = Self::by_phase(facet, &words);
        for i in 1..sorted.len() {
            let p1 = facet.lexicon.get(&sorted[i - 1]).map(|p| p.phase).unwrap_or(0.0);
            let p2 = facet.lexicon.get(&sorted[i]).map(|p| p.phase).unwrap_or(0.0);
            if p1 > p2 {
                return false;
            }
        }
        true
    }

    /// Sorts words by amplitude (familiarity).
    pub fn by_amplitude(facet: &Facet, words: &[String]) -> Vec<String> {
        let mut indexed: Vec<(String, f64)> = words
            .iter()
            .filter_map(|w| facet.lexicon.get(w).map(|p| (w.clone(), p.amplitude)))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        indexed.into_iter().map(|(w, _)| w).collect()
    }

    /// Sorts words by sector (coarse phase grouping).
    pub fn by_sector(facet: &Facet, words: &[String], n_sectors: u16) -> Vec<String> {
        let sector_width = TWO_PI / n_sectors as f64;
        let mut indexed: Vec<(String, u16)> = words
            .iter()
            .filter_map(|w| {
                facet.lexicon.get(w).map(|p| {
                    let sector = (p.phase / sector_width).floor() as u16;
                    (w.clone(), sector)
                })
            })
            .collect();
        indexed.sort_by_key(|a| a.1);
        indexed.into_iter().map(|(w, _)| w).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_phase() {
        let mut facet = Facet::new();
        facet.get_or_init("zebra");
        facet.get_or_init("apple");
        facet.get_or_init("mango");

        let words = vec!["zebra".to_string(), "apple".to_string(), "mango".to_string()];
        let sorted = SortingTest::by_phase(&facet, &words);
        assert_eq!(sorted.len(), 3);
    }
}
