use std::collections::HashMap;

use dashmap::DashMap;
use log::info;
use tower_lsp::lsp_types::{CompletionItem, Diagnostic, Position, Range};

use crate::parse_skill;

#[derive(Debug)]
pub struct SymbolCache {
    pub symbols: DashMap<String, HashMap<Range, Vec<CompletionItem>>>,
}

#[derive(Debug, Clone)]
struct FileNotInCache;

impl SymbolCache {
    pub fn new() -> SymbolCache {
        SymbolCache {
            symbols: DashMap::new(),
        }
    }

    pub fn update(&self, path: &str) -> Result<HashMap<Range, Vec<CompletionItem>>, Diagnostic> {
        let parsed = parse_skill(path)?;
        info!("parsed: {:?}", parsed);
        let ret = self.symbols.insert(path.to_owned(), parsed);
        Ok(ret.unwrap_or(HashMap::new()))
    }
}

#[cfg(test)]
mod tests {
    // use crate::cache::SymbolCache;
    // use std::collections::HashMap;
    // use std::path::Path;

    #[test]
    fn insert() {
        // let mut d = SymbolCache::new();
        // d.update();
        // let mut comp = HashMap::new();
        // comp.insert();
        // assert_eq!(d.documents, comp)
    }
}
