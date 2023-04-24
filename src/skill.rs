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

fn parse_scope(
    scoped_pair: Pair<Rule>,
    scope: Option<Range>,
) -> Vec<(Option<Range>, CompletionItem)> {
    let mut catalog: Vec<CompletionItem> = vec![];
    for inner in scoped_pair.into_inner() {
        info!("{:?}", inner);
        match inner.as_rule() {
            Rule::token => catalog.push(CompletionItem {
                label: inner.as_str().to_owned(),
                kind: Some(CompletionItemKind::VARIABLE),
                label_details: Some(CompletionItemLabelDetails {
                    description: None,
                    detail: Some("local".to_owned()),
                }),
                ..Default::default()
            }),
            Rule::list => catalog.push(CompletionItem {
                label: inner.into_inner().next().unwrap().as_str().to_owned(),
                kind: Some(CompletionItemKind::VARIABLE),
                label_details: Some(CompletionItemLabelDetails {
                    description: None,
                    detail: Some("local".to_owned()),
                }),
                ..Default::default()
            }),
            _ => {
                warn!(
                    "illegal expr in scoped variable definition {:?}",
                    inner.as_span().start_pos()
                )
            }
        }
    }
    catalog
        .into_iter()
        .map(|completion| (scope, completion))
        .collect()
}

fn recurse_pairs<'a>(
    ps: Pair<Rule>,
    catalog: &'a mut Vec<(Option<Range>, CompletionItem)>,
    last_comment: Option<String>,
    mut scoped: Option<Range>,
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
            let (pair_start_line, pair_start_col) = ps.as_span().start_pos().line_col();
            let (pair_end_line, pair_end_col) = ps.as_span().end_pos().line_col();
            for (ix, p) in ps.into_inner().enumerate() {
                if ix == 0 && p.as_str() == "let" {
                    scoped = Some(Range::new(
                        Position::new(pair_start_line as u32 - 1, pair_start_col as u32 - 1),
                        Position::new(pair_end_line as u32 - 1, pair_end_col as u32 - 1),
                    ));
                } else {
                    match scoped {
                        Some(_) => {
                            for scoped_completion in parse_scope(p, scoped) {
                                catalog.push(scoped_completion)
                            }
                            scoped = None;
                        }
                        None => {
                            comment = recurse_pairs(p, catalog, comment, scoped);
                        }
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
            catalog.push((
                None,
                CompletionItem {
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
                },
            ));
            comment = None;
        }
        _ => {
            comment = None;
        }
    }
    comment
}

pub fn parse_skill(path: &str) -> Result<Vec<(Option<Range>, CompletionItem)>, Diagnostic> {
    match fs::read_to_string(path) {
        Ok(content) => match SkillParser::parse(Rule::skill, &content) {
            Ok(parsed) => {
                let mut ret: Vec<(Option<Range>, CompletionItem)> = vec![];
                let mut last_comment: Option<String> = None;
                for pair in parsed.into_iter() {
                    last_comment = recurse_pairs(pair, &mut ret, last_comment, None)
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
        Err(err) => Ok(vec![]),
    }
}
pub fn parse_global_symbols(token: Pair<Rule>) -> Result<&str, Error<Rule>> {
    Ok("")
}
