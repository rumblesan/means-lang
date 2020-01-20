use regex::Regex;

#[derive(Debug, Clone)]
pub enum TokenTag {
    End,
    OpenParen,
    CloseParen,
    Operator,
    Number,
}

struct Matcher {
    regex: Regex,
    tag: TokenTag,
    parse: bool,
}

pub struct MeansLexer {
    matchers: Vec<Matcher>,
}

#[derive(Debug)]
pub struct Token {
    pub tag: TokenTag,
    pub contents: String,
    pub line: u64,
    pub character: u64,
}

impl MeansLexer {
    pub fn create() -> MeansLexer {
        let lexer = MeansLexer {
            matchers: vec![
                Matcher {
                    regex: Regex::new(r"^\s").unwrap(),
                    tag: TokenTag::OpenParen,
                    parse: false,
                },
                Matcher {
                    regex: Regex::new(r"^\(").unwrap(),
                    tag: TokenTag::OpenParen,
                    parse: true,
                },
                Matcher {
                    regex: Regex::new(r"^\)").unwrap(),
                    tag: TokenTag::CloseParen,
                    parse: true,
                },
                Matcher {
                    regex: Regex::new(r"^[\+\-\*/%]").unwrap(),
                    tag: TokenTag::Operator,
                    parse: true,
                },
                Matcher {
                    regex: Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap(),
                    tag: TokenTag::Operator,
                    parse: true,
                },
            ],
        };
        lexer
    }
    pub fn tokenise(&self, input: &str) -> Vec<Token> {
        let mut source = input;
        let mut tokens = Vec::new();
        loop {
            if source.len() < 1 {
                break;
            }

            for matcher in &self.matchers {
                if matcher.regex.is_match(&source) {
                    let m = matcher.regex.find(&source).unwrap();
                    let s = m.as_str();
                    if matcher.parse {
                        tokens.push(Token {
                            tag: matcher.tag.clone(),
                            contents: s.to_owned(),
                            character: 0,
                            line: 0,
                        });
                    }
                    source = &source[s.len()..];
                }
            }
        }
        return tokens;
    }
}
