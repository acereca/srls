use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, DocumentSymbol, Documentation, Position, Range, SymbolKind,
};

#[derive(Debug, Clone)]
pub enum TokenScope {
    Global(Position),
    Local(Range),
}

impl TokenScope {
    pub fn value(&self) -> &str {
        match self {
            TokenScope::Local(_) => "local",
            TokenScope::Global(_) => "global",
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenUse<'a> {
    Declaration,
    Instantiation { decl: &'a Token },
    Use { decl: &'a Token, inst: &'a Token },
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    VariableAssignment,
    VariableUse,
    Function,
    Struct,
}

impl TokenKind {
    fn to_completion_item_kind(&self) -> CompletionItemKind {
        match self {
            TokenKind::VariableAssignment => CompletionItemKind::VARIABLE,
            TokenKind::Function => CompletionItemKind::FUNCTION,
            TokenKind::Struct => CompletionItemKind::STRUCT,
            TokenKind::VariableUse => CompletionItemKind::VARIABLE,
        }
    }

    fn to_document_symbol_kind(&self) -> SymbolKind {
        match self {
            TokenKind::VariableAssignment => SymbolKind::VARIABLE,
            TokenKind::Function => SymbolKind::FUNCTION,
            TokenKind::Struct => SymbolKind::STRUCT,
            TokenKind::VariableUse => SymbolKind::VARIABLE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    /// kind of token
    pub kind: TokenKind,

    /// scope the token is declared for
    pub scope: TokenScope,

    /// name of the token
    pub name: String,

    pub info: Option<String>,

    pub documentation: Option<String>,

    /// if the token declares a scope this range will encompass it
    pub encloses: Option<Range>,

    /// place of declaration for the token (most likely the line)
    pub place: Range,
}

impl Token {
    pub fn in_scope(&self, at: Position) -> bool {
        match self.scope {
            TokenScope::Global(from) => {
                from.line < at.line || (from.line == at.line && from.character < at.character)
            }
            TokenScope::Local(from_to) => {
                (at.line > from_to.start.line && at.line < from_to.end.line)
                    || (at.line == from_to.start.line && at.character > from_to.start.character)
                    || (at.line == from_to.end.line && at.character < from_to.end.character)
            }
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

    use crate::token::{Token, TokenKind, TokenScope, TokenUse};
    #[test]
    fn token_scope() {
        let tok = Token {
            kind: TokenKind::Function,
            scope: TokenScope::Global(Position {
                line: 2,
                character: 0,
            }),
            name: "someVar".to_string(),
            info: None,
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
            kind: TokenKind::VariableAssignment,
            scope: TokenScope::Local(tok.encloses.unwrap()),
            name: "someOtherVar".to_string(),
            info: None,
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
