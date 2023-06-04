use std::fs::read_to_string;

use dashmap::DashMap;
use log::info;
use tower_lsp::lsp_types::Diagnostic;

use crate::{skill::parse_skill_content, token::Token};

#[derive(Debug)]
pub struct TokenCache {
    pub symbols: DashMap<String, Vec<Token>>,
}

#[derive(Debug, Clone)]
struct FileNotInCache;

impl TokenCache {
    pub fn new() -> TokenCache {
        TokenCache {
            symbols: DashMap::new(),
        }
    }

    pub fn update(&self, path: &str) -> (Vec<Token>, Vec<Diagnostic>) {
        let content = read_to_string(path);

        match content {
            Ok(skill_code) => {
                let (parsed_tokens, parsed_errors) = parse_skill_content(&skill_code);
                info!("parsed: {:?}", parsed_tokens.clone());
                info!("parsed_errs: {:?}", parsed_errors.clone());
                self.symbols.insert(path.to_owned(), parsed_tokens.clone());
                (parsed_tokens, parsed_errors)
            }
            Err(_) => (vec![], vec![]), // FIXME: missing err diag
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn insert() {}
}
