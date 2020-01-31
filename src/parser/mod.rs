pub mod lexer;

use super::ast::*;
use dass::error::ParserError;
use dass::lexer::DassLexer;
use dass::parser::{DassParser, DassTokenParser};
use lexer::TokenTag;

pub struct MeansParser<'a> {
    p: DassTokenParser<'a, TokenTag>,
}

impl<'a> MeansParser<'a> {
    pub fn new(lexer: DassLexer<'a, TokenTag>) -> Self {
        MeansParser {
            p: DassTokenParser::new(lexer),
        }
    }
    pub fn parse(&mut self) -> Result<Program, Vec<ParserError>> {
        let mut statements = vec![];
        let mut errors = vec![];
        while !self.p.eof() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    errors.push(e);
                    match self.p.pop_until(&TokenTag::SemiColon) {
                        Err(e) => errors.push(e),
                        Ok(_) => (),
                    }
                }
            }
        }
        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(Program { statements })
    }

    pub fn statement(&mut self) -> Result<Statement, ParserError> {
        let id = self.identifier()?;
        self.p.match_token(&TokenTag::Assignment)?;
        let expr = self.expression()?;
        self.p.match_token(&TokenTag::SemiColon)?;
        Ok(Statement::Assignment { id, expr })
    }

    pub fn expression(&mut self) -> Result<Expression, ParserError> {
        if self.p.la1(&TokenTag::OpenParen) {
            self.p.match_token(&TokenTag::OpenParen)?;
            let expr = self.expression()?;
            self.p.match_token(&TokenTag::CloseParen)?;
            return Ok(expr);
        }
        if self.p.la1(&TokenTag::Operator) {
            let op = self.unary_operator()?;
            return Ok(op);
        }
        let value = self.value()?;
        if self.p.la1(&TokenTag::Operator) {
            let op = self.binary_operator(Expression::EVal(value))?;
            return Ok(op);
        }
        Ok(Expression::EVal(value))
    }

    pub fn value(&mut self) -> Result<Value, ParserError> {
        if self.p.la1(&TokenTag::Float) {
            let f = self.p.match_token(&TokenTag::Float)?;
            let v = f.contents.parse::<f32>().unwrap();
            return Ok(Value::Float(v));
        }
        if self.p.la1(&TokenTag::Integer) {
            let f = self.p.match_token(&TokenTag::Integer)?;
            let i = f.contents.parse::<i32>().unwrap();
            return Ok(Value::Integer(i));
        }
        if self.p.la1(&TokenTag::Identifier) {
            let id = self.identifier()?;
            return Ok(Value::Variable(id));
        }
        let t = self.p.pop_token()?;
        Err(ParserError::unexpected_token(&TokenTag::Number, t))
    }

    pub fn identifier(&mut self) -> Result<String, ParserError> {
        let id = self.p.match_token(&TokenTag::Identifier)?;
        Ok(id.contents.to_owned())
    }

    pub fn unary_operator(&mut self) -> Result<Expression, ParserError> {
        let op_token = self.p.match_token(&TokenTag::Operator)?;
        let op = UnaryOperator::from_str(&op_token)?;
        let expr = self.expression()?;
        return Ok(Expression::UnaryOp {
            op: op,
            expr: Box::new(expr),
        });
    }

    pub fn binary_operator(&mut self, left: Expression) -> Result<Expression, ParserError> {
        let op_token = self.p.match_token(&TokenTag::Operator)?;
        let op = BinaryOperator::from_str(&op_token)?;
        let right = self.expression()?;
        Ok(Expression::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}
