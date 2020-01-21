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
                tokens::WhiteSpaceMatcher::with_newlines(true),
                tokens::SimpleTokenMatcher::open_paren(),
                tokens::SimpleTokenMatcher::close_paren(),
                tokens::SimpleTokenMatcher::open_brace(),
                tokens::SimpleTokenMatcher::close_brace(),
                tokens::SimpleTokenMatcher::open_bracket(),
                tokens::SimpleTokenMatcher::close_bracket(),
                tokens::SimpleTokenMatcher::semicolon(),
                tokens::SimpleTokenMatcher::assignment(),
                tokens::SimpleTokenMatcher::comma(),
                tokens::OperatorTokenMatcher::new(Regex::new(r"^[\+\-\*/%]").unwrap(), false),
                tokens::FloatTokenMatcher::new(),
                tokens::IdentifierTokenMatcher::new(
                    Regex::new(r"^[[:alpha:]][[:alnum:]]*").unwrap(),
                    false,
                ),
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
