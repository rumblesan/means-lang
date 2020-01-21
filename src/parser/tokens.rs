use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Token {
    End,
    WhiteSpace,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    SemiColon,
    Comma,
    Assignment,
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
    pub fn simple(skip: bool) -> Box<WhiteSpaceMatcher> {
        let regex = Regex::new(r"^\s").unwrap();
        Box::new(WhiteSpaceMatcher { regex, skip })
    }
    pub fn with_newlines(skip: bool) -> Box<WhiteSpaceMatcher> {
        let regex = Regex::new(r"^[[:space:]]+").unwrap();
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

fn gen_regex(symbol: &str) -> Regex {
    let pattern = format!("^{}", regex::escape(symbol));
    Regex::new(&pattern).unwrap()
}

fn constant(symbol: &str, output: Token, skip: bool) -> Box<SimpleTokenMatcher> {
    Box::new(SimpleTokenMatcher {
        regex: gen_regex(symbol),
        output,
        skip,
    })
}

impl SimpleTokenMatcher {
    pub fn new(regex: Regex, output: Token, skip: bool) -> Box<SimpleTokenMatcher> {
        Box::new(SimpleTokenMatcher {
            regex,
            output,
            skip,
        })
    }
    pub fn open_paren() -> Box<SimpleTokenMatcher> {
        constant("(", Token::OpenParen, false)
    }
    pub fn close_paren() -> Box<SimpleTokenMatcher> {
        constant(")", Token::CloseParen, false)
    }
    pub fn open_brace() -> Box<SimpleTokenMatcher> {
        constant("{", Token::OpenBrace, false)
    }
    pub fn close_brace() -> Box<SimpleTokenMatcher> {
        constant("}", Token::CloseBrace, false)
    }
    pub fn open_bracket() -> Box<SimpleTokenMatcher> {
        constant("[", Token::OpenBracket, false)
    }
    pub fn close_bracket() -> Box<SimpleTokenMatcher> {
        constant("]", Token::CloseBracket, false)
    }
    pub fn semicolon() -> Box<SimpleTokenMatcher> {
        constant(";", Token::SemiColon, false)
    }
    pub fn comma() -> Box<SimpleTokenMatcher> {
        constant(",", Token::Comma, false)
    }
    pub fn assignment() -> Box<SimpleTokenMatcher> {
        constant("=", Token::Assignment, false)
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
    pub fn new() -> Box<FloatTokenMatcher> {
        let regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        let skip = false;
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
