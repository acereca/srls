use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, DocumentSymbol, Documentation, Position, Range, SymbolKind,
};

#[derive(Debug)]
pub enum TokenScope<'a> {
    Global,
    Local(&'a Token<'a>),
}

impl<'a> TokenScope<'a> {
    fn value(&'a self) -> &str {
        match self {
            TokenScope::Local(_) => "local",
            TokenScope::Global => "global",
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Variable,
    Function,
    Struct,
}

impl TokenKind {
    fn to_completion_item_kind(&self) -> CompletionItemKind {
        match self {
            TokenKind::Variable => CompletionItemKind::VARIABLE,
            TokenKind::Function => CompletionItemKind::FUNCTION,
            TokenKind::Struct => CompletionItemKind::STRUCT,
        }
    }

    fn to_document_symbol_kind(&self) -> SymbolKind {
        match self {
            TokenKind::Variable => SymbolKind::VARIABLE,
            TokenKind::Function => SymbolKind::FUNCTION,
            TokenKind::Struct => SymbolKind::STRUCT,
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    /// kind of token
    kind: TokenKind,

    /// scope the token is declared for
    scope: TokenScope<'a>,

    /// name of the token
    name: String,

    documentation: Option<String>,

    /// if the token declares a scope this range will encompass it
    encloses: Option<Range>,

    /// place of declaration for the token (most likely the line)
    place: Range,
}

impl<'a> Token<'a> {
    pub fn in_scope(&self, at: Position) -> bool {
        match self.scope {
            TokenScope::Global => true,
            TokenScope::Local(scoping_token) => scoping_token.encloses.map_or(false, |scope| {
                (at.line > scope.start.line && at.line < scope.end.line)
                    || (at.line == scope.start.line && at.character > scope.start.character)
                    || (at.line == scope.end.line && at.character < scope.end.character)
            }),
        }
    }

    pub fn to_completion_item(&mut self, at: Option<Position>) -> Option<CompletionItem> {
        if at.map_or(true, |pos| self.in_scope(pos)) {
            Some(CompletionItem {
                label: self.name.to_owned(),
                kind: Some(self.kind.to_completion_item_kind()),
                detail: Some(self.scope.value().to_owned()),
                documentation: self
                    .documentation
                    .to_owned()
                    .map(|doc| Documentation::String(doc)),
                ..Default::default()
            })
        } else {
            None
        }
    }

    pub fn to_document_symbol(&mut self, at: Option<Position>) -> Option<DocumentSymbol> {
        if at.map_or(true, |pos| self.in_scope(pos)) {
            Some(DocumentSymbol {
                name: self.name.to_owned(),
                detail: Some(self.scope.value().to_owned()),
                kind: self.kind.to_document_symbol_kind(),
                range: self.encloses.unwrap_or(self.place),
                selection_range: self.place,
                children: Some(vec![]),
                tags: None,
                deprecated: None,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::{Position, Range};

    use crate::token::{Token, TokenKind, TokenScope};
    #[test]
    fn token_scope() {
        let tok = Token {
            kind: TokenKind::Function,
            scope: TokenScope::Global,
            name: "someVar".to_string(),
            documentation: None,
            encloses: Some(Range {
                start: Position {
                    line: 2,
                    character: 11,
                },
                end: Position {
                    line: 10,
                    character: 1,
                },
            }),
            place: Range {
                start: Position {
                    line: 2,
                    character: 0,
                },
                end: Position {
                    line: 2,
                    character: 10,
                },
            },
        };
        let tok2 = Token {
            kind: TokenKind::Variable,
            scope: TokenScope::Local(&tok),
            name: "someOtherVar".to_string(),
            documentation: Some("Some description".to_string()),
            encloses: None,
            place: Range {
                start: Position {
                    line: 3,
                    character: 0,
                },
                end: Position {
                    line: 3,
                    character: 10,
                },
            },
        };

        println!("{:?}", tok);
        assert!(tok.in_scope(Position {
            line: 11,
            character: 0
        }));
        println!("{:?}", tok2);
        assert!(!tok2.in_scope(Position {
            line: 11,
            character: 0
        }));
    }
}
