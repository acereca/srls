#[derive(Debug)]
pub enum CursorState {
    Comment,
    List,
    Token,
    Literal,
    Operator
}

pub struct Token {
    typ: CursorState,
    content: Vec<Token>,
}

pub struct Tokenizer {
    stateStack: Vec<CursorState>,
    tokenTree: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            stateStack: Vec::new(),
            tokenTree: Vec::new(),
        }
    }

    fn match_char(&mut self, c: char) {
        match self.stateStack.last() {
            None => match c {
                c if c.is_whitespace() => {}
                ';' => {
                    self.stateStack.push(CursorState::Comment);
                }
                '(' => {
                    self.stateStack.push(CursorState::List);
                }
                c if c.is_alphanumeric() => {}
                _ => {
                    println!("{}", c);
                    panic!("not a comment, list or symbol ");
                }
            },
            Some(CursorState::Comment) => match c {
                c if c.is_control() => {
                    self.stateStack.pop();
                }
                c if c.is_alphanumeric() => {}
                c if c.is_whitespace() => {}
                _ => {}
            },
            Some(CursorState::List) => match c {
                '(' => {
                    self.stateStack.push(CursorState::List);
                }
                ')' => {
                    self.stateStack.pop();
                }
                c if c.is_alphabetic() => {
                    self.stateStack.push(CursorState::Token);
                }
                c if c.is_numeric() => {
                    self.stateStack.push(CursorState::Literal);
                }
                '"' => {
                    self.stateStack.push(CursorState::Literal);
                }
                _ => {}
            },
            Some(CursorState::Token) => match c {
                c if !c.is_alphanumeric() => {
                    self.stateStack.pop();
                    self.match_char(c);
                }
                _ => {}
            },
            Some(CursorState::Literal) => match c {
                c if c.is_whitespace() => {
                    self.stateStack.pop();
                }
                '"' => {
                    self.stateStack.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn read_in(&mut self, content: String) {
        for c in content.chars() {
            self.match_char(c);
            println!("{} -> {}: {:?}", c, self.stateStack.len(), self.stateStack.last().clone());
        }
    }
}
