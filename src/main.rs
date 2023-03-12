use std::{env, fs};
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "skill.pest"]
pub struct SkillParser;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let data = fs::read_to_string(args[1].as_mut_str()).expect("could not read from file");


    let parse = SkillParser::parse(Rule::skill, data.as_str()).expect("ha").next().unwrap();
    for inner in parse.into_inner() {
        println!("{:?}: {:?}", inner.as_rule(), inner);
    }
}
