use std::fmt::Display;

use dass::error::ParserError;
use dass::tokens::TokenData;

use super::parser::lexer::TokenTag;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn pprint(&self) {
        println!("Program:");
        for s in &self.statements {
            println!("{}\n", s)
        }
    }
}

pub enum Statement {
    Assignment { id: String, expr: Expression },
}
impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Assignment { id, expr } => write!(f, "assign {} with {}\n", id, expr),
        }
    }
}

pub enum Expression {
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    EVal(Value),
}
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::BinaryOp { op, left, right } => write!(f, "{} {} {}", left, op, right),
            Expression::UnaryOp { op, expr } => write!(f, "{} {}", op, expr),
            Expression::EVal(v) => write!(f, "{}", v),
        }
    }
}

pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
}
impl BinaryOperator {
    pub fn from_str(op_token: &TokenData<TokenTag>) -> Result<Self, ParserError> {
        let op: &str = &op_token.contents;
        match op {
            "+" => Ok(BinaryOperator::Addition),
            "-" => Ok(BinaryOperator::Subtraction),
            "*" => Ok(BinaryOperator::Multiplication),
            "/" => Ok(BinaryOperator::Division),
            "%" => Ok(BinaryOperator::Modulo),
            _ => Err(ParserError::new(
                String::from("+, -, *, / or %"),
                op.to_owned(),
                Some(op_token.position()),
            )),
        }
    }
    pub fn get_precedence(&self) -> u8 {
        match self {
            BinaryOperator::Addition => 13,
            BinaryOperator::Subtraction => 13,
            BinaryOperator::Multiplication => 14,
            BinaryOperator::Division => 14,
            BinaryOperator::Modulo => 14,
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinaryOperator::Addition => write!(f, "+"),
            BinaryOperator::Subtraction => write!(f, "-"),
            BinaryOperator::Multiplication => write!(f, "*"),
            BinaryOperator::Division => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
        }
    }
}

pub enum UnaryOperator {
    Negation,
}
impl UnaryOperator {
    pub fn from_str(op_token: &TokenData<TokenTag>) -> Result<Self, ParserError> {
        let op: &str = &op_token.contents;
        match op {
            "-" => Ok(UnaryOperator::Negation),
            _ => Err(ParserError::new(
                String::from("-"),
                op.to_owned(),
                Some(op_token.position()),
            )),
        }
    }
}
impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnaryOperator::Negation => write!(f, "-"),
        }
    }
}

pub enum Value {
    Float(f32),
    Integer(i32),
    Variable(String),
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Float(v) => write!(f, "Float {}", v),
            Value::Integer(v) => write!(f, "Integer {}", v),
            Value::Variable(v) => write!(f, "Var {}", v),
        }
    }
}
