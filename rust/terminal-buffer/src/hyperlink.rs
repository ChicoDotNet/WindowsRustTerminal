use std::collections::HashMap;

/// Safe, platform-neutral ownership of the hyperlink map semantics used by TextBuffer.
///
/// Hyperlinks without a custom id receive a fresh numeric id every time. Hyperlinks with
/// a custom id are stable for the same `(custom_id, uri)` pair, while the same custom id
/// may legitimately identify different URIs and therefore receives a distinct numeric id.
#[derive(Clone, Debug, Default)]
pub struct HyperlinkStore {
    next_id: u16,
    uri_by_id: HashMap<u16, String>,
    custom_pair_to_id: HashMap<(String, String), u16>,
}

impl HyperlinkStore {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            uri_by_id: HashMap::new(),
            custom_pair_to_id: HashMap::new(),
        }
    }

    pub fn add(&mut self, uri: impl Into<String>, custom_id: Option<&str>) -> u16 {
        let uri = uri.into();

        if let Some(custom_id) = custom_id {
            let key = (custom_id.to_owned(), uri.clone());
            if let Some(existing) = self.custom_pair_to_id.get(&key) {
                return *existing;
            }

            let id = self.allocate_id();
            self.uri_by_id.insert(id, uri);
            self.custom_pair_to_id.insert(key, id);
            return id;
        }

        let id = self.allocate_id();
        self.uri_by_id.insert(id, uri);
        id
    }

    pub fn uri(&self, id: u16) -> Option<&str> {
        self.uri_by_id.get(&id).map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.uri_by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.uri_by_id.is_empty()
    }

    fn allocate_id(&mut self) -> u16 {
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).expect("hyperlink id space exhausted");
        id
    }
}

#[cfg(test)]
mod tests {
    use super::HyperlinkStore;

    #[test]
    fn anonymous_hyperlinks_are_independent() {
        let mut store = HyperlinkStore::new();
        let first = store.add("https://example.test/a", None);
        let second = store.add("https://example.test/a", None);
        assert_ne!(first, second);
        assert_eq!(store.uri(first), Some("https://example.test/a"));
        assert_eq!(store.uri(second), Some("https://example.test/a"));
    }

    #[test]
    fn custom_pair_is_stable() {
        let mut store = HyperlinkStore::new();
        let first = store.add("https://example.test/a", Some("same"));
        let second = store.add("https://example.test/a", Some("same"));
        assert_eq!(first, second);
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn same_custom_id_different_uri_is_not_aliased() {
        let mut store = HyperlinkStore::new();
        let first = store.add("https://example.test/a", Some("same"));
        let second = store.add("https://example.test/b", Some("same"));
        assert_ne!(first, second);
        assert_eq!(store.uri(first), Some("https://example.test/a"));
        assert_eq!(store.uri(second), Some("https://example.test/b"));
    }
}
