use log::{info, warn};
use pest::error::{Error, LineColLocation};
use pest::iterators::Pair;
use pest::Parser;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, Diagnostic, DiagnosticSeverity,
    Position, Range,
};

#[derive(Parser)]
#[grammar = "skill.pest"]
pub struct SkillParser;

fn parse_scope(scoped_pair: Pair<Rule>) -> (Range, Vec<CompletionItem>) {
    let mut catalog: Vec<CompletionItem> = vec![];
    let (start, end) = scoped_pair.as_span().split();
    for inner in scoped_pair.into_inner() {
        info!("{:?}", inner);
        match inner.as_rule() {
            Rule::token => catalog.push(CompletionItem::new_simple(
                inner.as_str().to_owned(),
                "local".to_owned(),
            )),
            Rule::list => catalog.push(CompletionItem::new_simple(
                inner.into_inner().next().unwrap().as_str().to_owned(),
                "local".to_owned(),
            )),
            _ => {
                warn!(
                    "illegal expr in scoped variable definition {:?}",
                    inner.as_span().start_pos()
                )
            }
        }
    }
    let (start_line, start_col) = start.line_col();
    let (end_line, end_col) = end.line_col();
    (
        Range::new(
            Position::new(start_line as u32 - 1, start_col as u32),
            Position::new(end_line as u32 - 1, end_col as u32),
        ),
        catalog,
    )
}

fn recurse_pairs<'a>(
    ps: Pair<Rule>,
    catalog: &'a mut HashMap<Range, Vec<CompletionItem>>,
    last_comment: Option<String>,
    mut scoped: bool,
) -> Option<String> {
    let mut comment = last_comment;
    match ps.as_rule() {
        Rule::skill => {
            comment = None;
            for p in ps.into_inner() {
                comment = recurse_pairs(p, catalog, comment, scoped);
            }
        }
        Rule::list => {
            comment = None;
            for (ix, p) in ps.into_inner().enumerate() {
                if ix == 0 && p.as_str() == "let" {
                    scoped = true;
                } else {
                    if scoped && ix == 1 {
                        let scope_catalog = parse_scope(p);
                        scoped = false;
                    } else {
                        comment = recurse_pairs(p, catalog, comment, scoped);
                    }
                }
            }
        }
        Rule::COMMENT => {
            info!("encountered comment: {:?}", ps.as_str());
            if ps.as_str().starts_with(";;;") {
                comment = Some(ps.as_str().strip_prefix(";;;").unwrap().trim().to_owned());
                info!("encountered docstring: {:?}", comment);
            }
        }
        Rule::assign => {
            let k = ps.into_inner().next().unwrap();
            let local_catalog = catalog.get(Range::new(Position::new(0, 0), Position::new(0, 0)));
            catalog.push(CompletionItem {
                label: k.as_str().to_owned(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: match &comment {
                    Some(s) => Some(s.to_owned()),
                    None => None,
                },
                label_details: Some(CompletionItemLabelDetails {
                    description: None,
                    detail: Some("global".to_owned()),
                }),
                ..Default::default()
            });
            comment = None;
        }
        _ => {
            comment = None;
        }
    }
    comment
}

pub fn parse_skill(path: &str) -> Result<HashMap<Range, Vec<CompletionItem>>, Diagnostic> {
    match fs::read_to_string(path) {
        Ok(content) => match SkillParser::parse(Rule::skill, &content) {
            Ok(parsed) => {
                let mut ret: HashMap<Range, Vec<CompletionItem>> = HashMap::new();
                let mut last_comment: Option<String> = None;
                for pair in parsed.into_iter() {
                    last_comment = recurse_pairs(pair, &mut ret, last_comment, false)
                }
                Ok(ret)
            }
            Err(err) => {
                let pos: (usize, usize);
                match err.line_col {
                    LineColLocation::Pos(line_col) => pos = line_col,
                    LineColLocation::Span(line_col, _) => pos = line_col,
                }

                Err(Diagnostic::new(
                    Range::new(
                        Position::new(pos.0 as u32 - 1, pos.1 as u32),
                        Position::new(pos.0 as u32 - 1, pos.1 as u32),
                    ),
                    Some(DiagnosticSeverity::ERROR),
                    None,
                    Some(path.to_owned()),
                    match err.variant.message() {
                        Cow::Borrowed(msg) => msg.to_owned(),
                        Cow::Owned(msg) => msg,
                    },
                    None,
                    None,
                ))
            }
        },
        Err(err) => Ok(HashMap::new()),
    }
}
pub fn parse_global_symbols(token: Pair<Rule>) -> Result<&str, Error<Rule>> {
    Ok("")
}
