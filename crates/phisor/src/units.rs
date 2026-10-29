//! `units` — what phisor hands the one routine (docs/phisor/units.md):
//! names interned as unit ids, words for the code and templates for the
//! kinds. The routine itself, halving a code as far as it pays, is phical's
//! (`phical::classes::halve`); phisor turns its text into ids and back.

use std::collections::HashMap;

/// Names interned as unit ids, in the order first seen.
#[derive(Clone, Debug, Default)]
pub struct Units {
    ids: HashMap<String, usize>,
    names: Vec<String>,
}

impl Units {
    pub fn new() -> Self {
        Self::default()
    }

    /// The id of a name, coined when new.
    pub fn id(&mut self, name: &str) -> usize {
        match self.ids.get(name) {
            Some(&i) => i,
            None => {
                self.ids.insert(name.to_string(), self.names.len());
                self.names.push(name.to_string());
                self.names.len() - 1
            }
        }
    }

    /// The id of a name already coined.
    pub fn get(&self, name: &str) -> Option<usize> {
        self.ids.get(name).copied()
    }

    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_are_interned_in_the_order_first_seen() {
        let mut u = Units::new();
        assert_eq!((u.id("the"), u.id("cat"), u.id("the")), (0, 1, 0));
        assert_eq!(u.get("cat"), Some(1));
        assert_eq!(u.get("dog"), None);
        assert_eq!(u.names(), ["the", "cat"]);
    }
}
