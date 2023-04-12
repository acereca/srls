use dashmap::DashMap;
use std::path::{Path, PathBuf};

#[derive(PartialEq, Debug)]
struct Position {
    line: u32,
    character: u16,
}

#[derive(PartialEq, Debug)]
struct Range {
    from: Position,
    to: Position,
}

#[derive(PartialEq, Debug)]
struct CachedScope {
    range: Range,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CachedItems {
    global_tokens: Vec<String>,
    scopes: Vec<CachedScope>,
}

#[derive(Debug)]
pub struct DocumentCache {
    pub documents: DashMap<PathBuf, CachedItems>,
}

#[derive(Debug, Clone)]
struct FileNotInCache;

impl DocumentCache {
    pub fn new() -> DocumentCache {
        DocumentCache {
            documents: DashMap::new(),
        }
    }

    pub fn update_document(&self, path: PathBuf, items: CachedItems) {
        self.documents.insert(path, items);
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::{CachedItems, DocumentCache};
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn insert() {
        let mut d = DocumentCache::new(Path::new("/example/workdir").to_path_buf());
        d.update_document(
            Path::new("example_file.ext").to_path_buf(),
            CachedItems {
                global_tokens: vec![],
                scopes: vec![],
            },
        );
        let mut comp = HashMap::new();
        comp.insert(
            Path::new("example_file.ext").to_path_buf(),
            CachedItems {
                global_tokens: vec![],
                scopes: vec![],
            },
        );
        assert_eq!(d.documents, comp)
    }
}
