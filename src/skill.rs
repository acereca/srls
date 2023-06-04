use log::info;
use pest::error::{InputLocation, LineColLocation};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use regex::Regex;
use std::borrow::Cow;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, DiagnosticTag, Position, Range};

use crate::token::{Token, TokenKind, TokenScope, TokenUse};

#[derive(Parser)]
#[grammar = "skill.pest"]
pub struct SkillParser;

// fn parse_skill_pairs(pairs: &Pairs<Rule>) -> Vec<Result<Token, Diagnostic>> {
//     let mut prev_docstring: Option<Pair<Rule>> = None;
//     let mut collection: Vec<Result<Token, Diagnostic>> = vec![];
//
//     for pair in pairs {
//         match pair.as_rule() {
//             Rule::COMMENT => {
//                 prev_docstring = Some(pair);
//             }
//             Rule::token => {
//                 let (start_line, start_col) = pair.as_span().start_pos().line_col();
//                 let (end_line, end_col) = pair.as_span().end_pos().line_col();
//                 let docstring_start = Regex::new(r";;;\s*").unwrap();
//                 let applicable_docstring = match prev_docstring.clone() {
//                     Some(pair) => {
//                         let prev_docstring_line = pair.as_span().end_pos().line_col().0;
//                         if prev_docstring_line == start_line {
//                             Some(docstring_start.replace_all(pair.as_str(), "").to_string())
//                         } else {
//                             None
//                         }
//                     }
//                     None => None,
//                 };
//                 let name = pair.as_str().to_string();
//                 if !vec!["t", "nil"].contains(&name.as_str()) {
//                     let last_declaration = collection
//                         .iter()
//                         .filter(|tok| tok.is_ok())
//                         .filter(|tok| {
//                             tok.as_ref().unwrap().name == name
//                                 && matches!(tok.as_ref().unwrap().usage, TokenUse::Declaration)
//                         })
//                         .rev()
//                         .next()
//                         .unwrap();
//
//                     // let last_instantiation = collection
//                     //     .iter_mut()
//                     //     .filter(|tok| tok.is_ok())
//                     //     .filter(|tok| {
//                     //         tok.as_ref().unwrap().name == name
//                     //             && matches!(
//                     //                 tok.as_ref().unwrap().usage,
//                     //                 TokenUse::Instantiation { decl: _ }
//                     //             )
//                     //     })
//                     //     .rev()
//                     //     .next()
//                     //     .unwrap();
//
//                     collection.push(Ok(Token {
//                         name,
//                         usage: TokenUse::Use {
//                             decl: &last_declaration.unwrap().clone(),
//                             inst: &last_declaration.unwrap().clone(),
//                         },
//                         kind: TokenKind::Variable,
//                         scope: TokenScope::Global,
//                         documentation: applicable_docstring,
//                         encloses: None,
//                         place: Range {
//                             start: Position {
//                                 line: start_line as u32,
//                                 character: start_col as u32,
//                             },
//                             end: Position {
//                                 line: end_line as u32,
//                                 character: end_col as u32,
//                             },
//                         },
//                     }));
//                 } else {
//                 }
//             }
//             Rule::skill => {
//                 collection.append(parse_skill_pairs(pair.into_inner()));
//             }
//             Rule::list => {
//                 collection.append(parse_skill_pairs(pair.into_inner()).as_mut());
//             }
//             Rule::cstyle_list => {
//                 collection.append(parse_skill_pairs(pair.into_inner()).as_mut());
//             }
//             Rule::assign => {
//                 let (start_line, start_col) = pair.as_span().start_pos().line_col();
//                 let (end_line, end_col) = pair.as_span().end_pos().line_col();
//                 let name_rule = pair.into_inner().next().unwrap();
//                 let name = name_rule.as_str().to_string();
//                 let docstring_start = Regex::new(r";;;\s*").unwrap();
//                 let applicable_docstring = match prev_docstring.clone() {
//                     Some(pair) => {
//                         let prev_docstring_line = pair.as_span().end_pos().line_col().0;
//                         if prev_docstring_line == start_line {
//                             Some(docstring_start.replace_all(pair.as_str(), "").to_string())
//                         } else {
//                             None
//                         }
//                     }
//                     None => None,
//                 };
//                 let last_declaration = collection
//                     .iter()
//                     .filter(|tok| {
//                         tok.is_ok()
//                             && (tok.unwrap().name == name)
//                             && matches!(tok.unwrap().usage, TokenUse::Declaration)
//                     })
//                     .rev()
//                     .next()
//                     .unwrap()
//                     .unwrap();
//                 collection.push(Ok(Token {
//                     name: name,
//                     usage: TokenUse::Instantiation {
//                         decl: &last_declaration,
//                     },
//                     kind: TokenKind::Variable,
//                     scope: TokenScope::Global,
//                     documentation: applicable_docstring,
//                     encloses: None,
//                     place: Range {
//                         start: Position {
//                             line: start_line as u32,
//                             character: start_col as u32,
//                         },
//                         end: Position {
//                             line: end_line as u32,
//                             character: end_col as u32,
//                         },
//                     },
//                 }))
//             }
//             _ => {
//                 println!(
//                     "unhandled {:?}: ({},{}) -> ({},{})",
//                     pair.as_rule(),
//                     pair.as_span().start_pos().line_col().0,
//                     pair.as_span().start_pos().line_col().1,
//                     pair.as_span().end_pos().line_col().0,
//                     pair.as_span().end_pos().line_col().1
//                 );
//             }
//         };
//     }
//
//     collection
// }
//

fn start_position_of_pair(pair: &Pair<Rule>) -> Position {
    let (line, character) = pair.as_span().start_pos().line_col();

    Position {
        line: line as u32 - 1,
        character: character as u32 - 1,
    }
}
fn end_position_of_pair(pair: &Pair<Rule>) -> Position {
    let (line, character) = pair.as_span().end_pos().line_col();

    Position {
        line: line as u32 - 1,
        character: character as u32 - 1,
    }
}

fn range_of_pair(pair: &Pair<Rule>) -> Range {
    Range {
        start: start_position_of_pair(pair),
        end: end_position_of_pair(pair),
    }
}

fn parse_skill_pairs(pairs: Pairs<Rule>) -> Vec<Pair<Rule>> {
    let mut collection: Vec<Pair<Rule>> = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::COMMENT | Rule::token => collection.push(pair),
            Rule::skill | Rule::assign => {
                collection.push(pair.clone());
                collection.append(parse_skill_pairs(pair.into_inner()).as_mut())
            }
            Rule::list | Rule::cstyle_list => {
                collection.push(pair.clone());
                collection.append(parse_skill_pairs(pair.into_inner()).as_mut())
                // let inners = pair.into_inner();
                // match inners.peek() {
                //     Some(first_inner_pair) => match first_inner_pair.as_rule() {
                //         Rule::keywords => {
                //             let parser: fn(Pairs<Rule>) -> Vec<Pair<Rule>> =
                //                 inners.peek().map_or(parse_skill_pairs, |keyword| {
                //                     match keyword.as_str() {
                //                         "let" => parse_let,
                //                         _ => parse_skill_pairs,
                //                     }
                //                 });
                //             collection.append(parser(inners).as_mut());
                //         }
                //         _ => {
                //             collection.append(parse_skill_pairs(inners).as_mut());
                //         }
                //     },
                //     None => {
                //         collection.append(parse_skill_pairs(inners).as_mut());
                //     }
                // }
            }
            _ => {
                println!(
                    "unhandled {:?}: ({},{}) -> ({},{})",
                    pair.as_rule(),
                    pair.as_span().start_pos().line_col().0,
                    pair.as_span().start_pos().line_col().1,
                    pair.as_span().end_pos().line_col().0,
                    pair.as_span().end_pos().line_col().1
                );
            }
        };
    }

    collection
}

fn variable_declaration(name: &str, scope: Range, info: &str, place: Range) -> Token {
    Token {
        kind: TokenKind::VariableAssignment,
        scope: TokenScope::Local(scope),
        documentation: None,
        name: name.to_string(),
        info: Some(info.to_string()),
        encloses: None,
        place,
    }
}

fn parse_scoped_vars(pairs: Pairs<Rule>, scope: &Range) -> Vec<Token> {
    let mut passed_assigns = vec![];

    for p in pairs {
        match p.as_rule() {
            Rule::list => {
                let info = p.as_str();
                let f = p.clone().into_inner().peek().unwrap();
                passed_assigns.push(variable_declaration(
                    f.as_str(),
                    scope.to_owned(),
                    info,
                    range_of_pair(&p),
                ))
            }
            Rule::token => {
                passed_assigns.push(variable_declaration(
                    p.as_str(),
                    scope.to_owned(),
                    format!("({} nil)", p.as_str()).as_str(),
                    range_of_pair(&p),
                ));
            }
            _ => {}
        }
    }

    passed_assigns
}

fn parse_flat_pairs(pairs: Vec<Pair<Rule>>) -> (Vec<Token>, Vec<Diagnostic>) {
    let mut last_comment: (Position, Cow<str>) = (
        Position {
            line: 100000000,
            character: 0,
        },
        Cow::Owned("".to_string()),
    );
    let mut active_scope: Option<Range> = None;
    let mut parsed_tokens = vec![];
    let mut parsed_declarations = vec![];
    let mut parsed_errors = vec![];
    let docstring_start = Regex::new(r";;;\s*").unwrap();

    for pair in pairs {
        match pair.as_rule() {
            Rule::COMMENT => {
                if pair.as_str().starts_with(";;;") {
                    let corrected_docstring = docstring_start.replace_all(pair.as_str(), "");
                    last_comment = (end_position_of_pair(&pair), corrected_docstring);
                    println!("{:?}", last_comment.clone());
                }
            }
            Rule::token => {
                let name = pair.as_str().to_string();
                parsed_tokens.push(Token {
                    kind: TokenKind::VariableUse,
                    scope: TokenScope::Global(end_position_of_pair(&pair)),
                    name: name.clone(),
                    info: None,
                    documentation: None,
                    encloses: None,
                    place: range_of_pair(&pair),
                });

                if !parsed_declarations.contains(&name) {
                    parsed_errors.push(Diagnostic {
                        range: range_of_pair(&pair),
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("srls".to_string()),
                        message: "variable used before declaration".to_string(),
                        related_information: None,
                        tags: None,
                        data: None,
                    })
                }
            }
            Rule::assign => {
                let range = range_of_pair(&pair);
                let info = Some(pair.as_str().to_string());
                let assigned_to = pair.into_inner().next().unwrap();
                let name = assigned_to.as_str().to_string();
                parsed_declarations.push(name.clone());
                parsed_tokens.push(Token {
                    kind: TokenKind::VariableAssignment,
                    scope: TokenScope::Global(range.end),
                    info,
                    name,
                    documentation: if last_comment.0.line == range.start.line {
                        Some(last_comment.1.to_string())
                    } else {
                        None
                    },
                    encloses: None,
                    place: range,
                });
            }
            Rule::list => {
                let range = range_of_pair(&pair);
                let info = pair
                    .as_str()
                    .split("\n")
                    .next()
                    .map(|info| info.to_string());

                match pair.clone().into_inner().peek().map_or(None, |first| {
                    let kw = first.as_str();
                    match kw {
                        "let" => Some(kw),
                        _ => None,
                    }
                }) {
                    Some(_) => {
                        parsed_tokens.push(Token {
                            kind: TokenKind::LetBlock,
                            scope: TokenScope::Local(range),
                            info,
                            name: format!("let:{line}", line = range.start.line),
                            documentation: if last_comment.0.line == range.start.line {
                                Some(last_comment.1.to_string())
                            } else {
                                None
                            },
                            encloses: Some(range),
                            place: range,
                        });

                        active_scope = Some(range);
                    }
                    None => match active_scope {
                        Some(scope) => {
                            let mut variables = parse_scoped_vars(pair.into_inner(), &scope);

                            parsed_declarations.append(
                                variables
                                    .clone()
                                    .iter()
                                    .map(|tok| tok.name.clone())
                                    .collect::<Vec<_>>()
                                    .as_mut(),
                            );
                            parsed_tokens.append(variables.as_mut());

                            active_scope = None;
                        }
                        None => {
                            // parsed_tokens.push(Token {
                            //     kind: TokenKind::List,
                            //     scope: TokenScope::Local(range),
                            //     info,
                            //     name: format!("list:{line}", line = range.start.line),
                            //     documentation: if last_comment.0.line == range.start.line {
                            //         Some(last_comment.1.to_string())
                            //     } else {
                            //         None
                            //     },
                            //     encloses: Some(range),
                            //     place: range,
                            // });
                        }
                    },
                };

                // println!("");
                // println!(
                //     "let: {}, {:?}",
                //     first_pair(&pair.clone().into_inner()),
                //     pair.clone()
                // );
                // println!("");
            }
            _ => {
                println!(
                    "unhandled {:?} ({:?})",
                    pair.as_rule(),
                    range_of_pair(&pair)
                );
            }
        }
    }

    (parsed_tokens, parsed_errors)
}

pub fn parse_skill_content(content: &str) -> (Vec<Token>, Vec<Diagnostic>) {
    match SkillParser::parse(Rule::skill, &content) {
        Ok(file) => {
            let flat_tokens = parse_skill_pairs(file);
            parse_flat_pairs(flat_tokens)
        }
        Err(e) => (
            vec![],
            vec![Diagnostic {
                range: match e.line_col {
                    LineColLocation::Pos((line, col)) => Range {
                        start: Position {
                            line: line as u32 - 1,
                            character: col as u32 - 1,
                        },
                        end: Position {
                            line: line as u32 - 1,
                            character: col as u32 - 1,
                        },
                    },
                    LineColLocation::Span((from_line, from_col), (to_line, to_col)) => Range {
                        start: Position {
                            line: from_line as u32 - 1,
                            character: from_col as u32 - 1,
                        },
                        end: Position {
                            line: to_line as u32 - 1,
                            character: to_col as u32 - 1,
                        },
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: "invalid syntax".to_string(),
                ..Default::default()
            }],
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::parse_skill_content;

    #[test]
    fn parse_testing_file() {
        let content = fs::read_to_string("test/data/test.il").unwrap();
        let (parsed_tokens, parsed_errors) = parse_skill_content(&content);

        for token in parsed_tokens {
            println!("{:?}", token);
        }
        for error in parsed_errors {
            println!("{:?}", error);
        }
    }
}
