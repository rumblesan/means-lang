use regex::Regex;

use super::tokens;
use super::tokens::{Token, TokenParser};

pub struct MeansLexer {
    matchers: Vec<Box<dyn TokenParser>>,
}

impl MeansLexer {
    pub fn create() -> MeansLexer {
        let lexer = MeansLexer {
            matchers: vec![
                tokens::WhiteSpaceMatcher::new(Regex::new(r"^\s").unwrap(), true),
                tokens::SimpleTokenMatcher::new(
                    Regex::new(r"^\(").unwrap(),
                    Token::OpenParen,
                    false,
                ),
                tokens::SimpleTokenMatcher::new(
                    Regex::new(r"^\)").unwrap(),
                    Token::CloseParen,
                    false,
                ),
                tokens::OperatorTokenMatcher::new(Regex::new(r"^[\+\-\*/%]").unwrap(), false),
                tokens::FloatTokenMatcher::new(Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap(), false),
            ],
        };
        lexer
    }
    pub fn tokenise(&self, input: &str) -> Vec<Token> {
        let mut source = input;
        let mut tokens = Vec::new();
        let mut matched = false;
        loop {
            if source.len() < 1 {
                break;
            }

            for matcher in &self.matchers {
                if matcher.regex().is_match(&source) {
                    matched = true;
                    let m = matcher.regex().find(&source).unwrap();
                    let s = m.as_str();
                    if !matcher.skip() {
                        let t = matcher.parse(m.as_str());
                        println!("{:?}", t);
                        tokens.push(t);
                    }
                    source = &source[s.len()..];
                }
            }
            if !matched {
                println!("Matching Error!");
            }
            matched = false;
        }
        return tokens;
    }
}
