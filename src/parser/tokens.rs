use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Token {
    End,
    WhiteSpace,
    OpenParen,
    CloseParen,
    Operator(String),
    Number(f64),
    Identifier(String),
}

pub trait TokenParser {
    fn parse(&self, input: &str) -> Token;
    fn regex(&self) -> &Regex;
    fn skip(&self) -> bool;
}

pub struct WhiteSpaceMatcher {
    regex: Regex,
    skip: bool,
}

impl WhiteSpaceMatcher {
    pub fn new(regex: Regex, skip: bool) -> Box<WhiteSpaceMatcher> {
        Box::new(WhiteSpaceMatcher { regex, skip })
    }
}

impl TokenParser for WhiteSpaceMatcher {
    fn parse(&self, _input: &str) -> Token {
        Token::WhiteSpace
    }
    fn regex(&self) -> &Regex {
        &self.regex
    }
    fn skip(&self) -> bool {
        self.skip
    }
}

pub struct SimpleTokenMatcher {
    regex: Regex,
    output: Token,
    skip: bool,
}

impl SimpleTokenMatcher {
    pub fn new(regex: Regex, output: Token, skip: bool) -> Box<SimpleTokenMatcher> {
        Box::new(SimpleTokenMatcher {
            regex,
            output,
            skip,
        })
    }
}

impl TokenParser for SimpleTokenMatcher {
    fn parse(&self, _input: &str) -> Token {
        self.output.clone()
    }
    fn regex(&self) -> &Regex {
        &self.regex
    }
    fn skip(&self) -> bool {
        self.skip
    }
}

pub struct OperatorTokenMatcher {
    regex: Regex,
    skip: bool,
}

impl OperatorTokenMatcher {
    pub fn new(regex: Regex, skip: bool) -> Box<OperatorTokenMatcher> {
        Box::new(OperatorTokenMatcher { regex, skip })
    }
}

impl TokenParser for OperatorTokenMatcher {
    fn parse(&self, input: &str) -> Token {
        Token::Operator(input.to_owned())
    }
    fn regex(&self) -> &Regex {
        &self.regex
    }
    fn skip(&self) -> bool {
        self.skip
    }
}

pub struct IdentifierTokenMatcher {
    regex: Regex,
    skip: bool,
}

impl IdentifierTokenMatcher {
    pub fn new(regex: Regex, skip: bool) -> Box<IdentifierTokenMatcher> {
        Box::new(IdentifierTokenMatcher { regex, skip })
    }
}

impl TokenParser for IdentifierTokenMatcher {
    fn parse(&self, input: &str) -> Token {
        Token::Identifier(input.to_owned())
    }
    fn regex(&self) -> &Regex {
        &self.regex
    }
    fn skip(&self) -> bool {
        self.skip
    }
}

pub struct FloatTokenMatcher {
    regex: Regex,
    skip: bool,
}

impl FloatTokenMatcher {
    pub fn new(regex: Regex, skip: bool) -> Box<FloatTokenMatcher> {
        Box::new(FloatTokenMatcher { regex, skip })
    }
}

impl TokenParser for FloatTokenMatcher {
    fn parse(&self, input: &str) -> Token {
        // TODO make this all safe?
        let n = f64::from_str(input).unwrap();
        Token::Number(n)
    }
    fn regex(&self) -> &Regex {
        &self.regex
    }
    fn skip(&self) -> bool {
        self.skip
    }
}
