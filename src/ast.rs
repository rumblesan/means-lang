
pub struct Program {
  pub statements: Vec<Statement>
}

pub enum Statement {
  SExpr(Expression),
}

pub enum Expression {
  BinaryOp(op: BinaryOperator, left: Value, right: Value),
  UnaryOp(op: UnaryOperator, value: Value),
  EVal(Value),
}

pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
}

pub enum UnaryOperator {
    Negation
}

pub enum Value {
  Number(f32),
}
