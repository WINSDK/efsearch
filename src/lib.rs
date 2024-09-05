use std::cmp::Ordering;
use std::ops::Range;

#[inline]
fn sort_cmp(a: &str, b: &str) -> Ordering {
    for (x, y) in a.chars().zip(b.chars()) {
        if x < y {
            return Ordering::Less;
        } else if x > y {
            return Ordering::Greater;
        }
    }

    a.len().cmp(&b.len())
}

#[inline]
fn find_cmp(a: &str, b: &str) -> Ordering {
    for (x, y) in a.chars().zip(b.chars()) {
        if x < y {
            return Ordering::Less;
        } else if x > y {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

/// Datastructure for efficient string match searching.
#[derive(Default, Debug)]
pub struct PrefixMatch<Metadata> {
    items: Vec<(String, Metadata)>,
}

impl<Metadata> PrefixMatch<Metadata> {
    /// Insert an item, doesn't ensure the items are sorted.
    pub fn insert<S: Into<String>>(&mut self, s: S, meta: Metadata) {
        self.items.push((s.into(), meta));
    }

    /// Sorts elements to allow for searching.
    pub fn reorder(&mut self) {
        self.items.sort_unstable_by(|a, b| sort_cmp(&a.0, &b.0));
    }

    /// Find some given prefix and return a range back into the items.
    /// Must call [`PrefixMatch::reorder`] before calling this.
    pub fn find(&self, prefix: &str) -> Match {
        // This works as cmp() will return Ordering::Equal if the prefix matches.
        let Ok(mid) = self.items.binary_search_by(|item| find_cmp(&item.0, prefix)) else {
            return Match { range: 0..0 };
        };

        // Look left and try to find more matching prefixes
        let mut start = mid;
        while start > 0 && self.items[start - 1].0.starts_with(prefix) {
            start -= 1;
        }

        // Look right and try to find more matching prefixes
        let mut end = mid;
        while end + 1 < self.items.len() && self.items[end + 1].0.starts_with(prefix) {
            end += 1;
        }

        Match { range: start..end + 1 }
    }
}

/// Storage mechanism for [`PrefixMatch::find`].
pub struct Match {
    range: Range<usize>,
}

impl Match {
    /// Iterate through all items that match.
    pub fn iter<'s, Metadata>(
        &self,
        tree: &'s PrefixMatch<Metadata>,
    ) -> impl Iterator<Item = (&'s str, &'s Metadata)> {
        tree.items[self.range.clone()].iter().map(|item| (item.0.as_str(), &item.1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert() {
        let mut tree = PrefixMatch::default();
        tree.insert("file", 0);
        tree.insert("file_name", 1);
        tree.insert("file::name", 2);
        tree.insert("file::no", 3);
        tree.reorder();
        let expected = [
            ("file", 0),
            ("file::name", 2),
            ("file::no", 3),
            ("file_name", 1),
        ];
        assert_eq!(tree.items.len(), expected.len(), "Mismatched length");
        for (x, y) in tree.items.iter().zip(expected.iter()) {
            assert_eq!(x.0, y.0, "Mismatched strings");
            assert_eq!(x.1, y.1, "Mismatched metadata");
        }
    }

    #[test]
    fn find() {
        let mut tree = PrefixMatch::default();
        tree.insert("file", 0);
        tree.insert("file_name", 1);
        tree.insert("file::name", 2);
        tree.insert("file::no", 3);
        tree.reorder();
        let expected = [
            ("file::name", 2),
            ("file::no", 3),
        ];
        assert_eq!(tree.find("file::").range.len(), expected.len(), "Mismatched length");
        for (x, y) in tree.find("file::").iter(&tree).zip(expected.iter()) {
            assert_eq!(x.0, y.0, "Mismatched strings");
            assert_eq!(*x.1, y.1, "Mismatched metadata");
        }
    }
}
