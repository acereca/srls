use dashmap::DashMap;
use log::info;
use tower_lsp::lsp_types::{CompletionItem, Diagnostic};

use crate::parse_skill;

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

#[derive(Debug)]
pub struct SymbolCache {
    pub symbols: DashMap<String, Vec<CompletionItem>>,
}

#[derive(Debug, Clone)]
struct FileNotInCache;

impl SymbolCache {
    pub fn new() -> SymbolCache {
        SymbolCache {
            symbols: DashMap::new(),
        }
    }

    pub fn update(&self, path: &str) -> Result<Vec<CompletionItem>, Diagnostic> {
        let parsed = parse_skill(path)?;
        info!("parsed: {:?}", parsed);
        let ret = self.symbols.insert(path.to_owned(), parsed);
        Ok(ret.unwrap_or(vec![]))
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
