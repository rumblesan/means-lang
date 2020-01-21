use regex::Regex;
use std::fmt::{Debug, Display};

use dass::lexer::DassLexer;
use dass::tokens::{TokenData, TokenMatcher};

pub type Token = TokenData<TokenTag>;

#[derive(Debug, Clone)]
pub enum TokenTag {
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
    Operator,
    Float,
    Integer,
    Number,
    Identifier,
}

impl Display for TokenTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

pub fn new_lexer() -> DassLexer<TokenTag> {
    DassLexer::create(vec![
        TokenMatcher::whitespace_with_newlines(TokenTag::WhiteSpace),
        TokenMatcher::constant("(", TokenTag::OpenParen),
        TokenMatcher::constant(")", TokenTag::CloseParen),
        TokenMatcher::constant("{", TokenTag::OpenBrace),
        TokenMatcher::constant("}", TokenTag::CloseBrace),
        TokenMatcher::constant("[", TokenTag::OpenBracket),
        TokenMatcher::constant("]", TokenTag::CloseBracket),
        TokenMatcher::constant(";", TokenTag::SemiColon),
        TokenMatcher::constant("=", TokenTag::Assignment),
        TokenMatcher::constant(",", TokenTag::Comma),
        TokenMatcher::operators("+-*/%", TokenTag::Operator),
        TokenMatcher::float(TokenTag::Float),
        TokenMatcher::integer(TokenTag::Integer),
        TokenMatcher::regex(
            Regex::new(r"^[[:alpha:]][[:alnum:]]*").unwrap(),
            TokenTag::Identifier,
        ),
    ])
}
