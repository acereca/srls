use pest::error::Error;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "./skill.pest"]
pub struct SkillParser;

pub fn parse_skill(path: &str) -> Vec<Pair<Rule>> {
    vec![]
}
pub fn parse_global_symbols(token: Pair<Rule>) -> Result<&str, Error<Rule>> {
    Ok("")
}
