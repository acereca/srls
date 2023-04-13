use pest::error::Error;
use pest::iterators::Pair;

use pest::Parser;

use log::info;

#[derive(Parser)]
#[grammar = "skill.pest"]
pub struct SkillParser;

pub fn parse_skill(path: &str) -> Vec<Pair<Rule>> {
    let parsed = SkillParser::parse(Rule::skill, path);
    info!("{:?}", parsed);
    vec![]
}
pub fn parse_global_symbols(token: Pair<Rule>) -> Result<&str, Error<Rule>> {
    Ok("")
}
