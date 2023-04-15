use log::info;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::fs;

use tower_lsp::lsp_types::{
    lsif::{EdgeDataMultiIn, Item, ItemKind},
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, Documentation, MarkupContent,
    MarkupKind,
};

#[derive(Parser)]
#[grammar = "skill.pest"]
pub struct SkillParser;

fn recurse_pairs<'a>(
    ps: Pair<Rule>,
    catalog: &'a mut Vec<CompletionItem>,
    last_comment: Option<String>,
) -> Option<String> {
    let mut comment = last_comment;
    match ps.as_rule() {
        Rule::skill => {
            comment = None;
            for p in ps.into_inner() {
                comment = recurse_pairs(p, catalog, comment);
            }
        }
        Rule::list => {
            comment = None;
            for p in ps.into_inner() {
                comment = recurse_pairs(p, catalog, comment);
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

pub fn parse_skill(path: &str) -> Vec<CompletionItem> {
    let content = fs::read_to_string(path).expect("could not read file");
    let parsed = SkillParser::parse(Rule::skill, &content);
    let mut ret = vec![];
    let mut last_comment: Option<String> = None;
    for pairs in parsed.into_iter() {
        for pair in pairs.into_iter() {
            last_comment = recurse_pairs(pair, &mut ret, last_comment)
        }
    }
    ret
}
pub fn parse_global_symbols(token: Pair<Rule>) -> Result<&str, Error<Rule>> {
    Ok("")
}
