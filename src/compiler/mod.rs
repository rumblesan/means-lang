use std::fmt::{Debug, Display};

use super::ast;
use super::vm::code_block::CodeBlock;
use super::vm::ops::Op;
use super::vm::value;

#[derive(Debug)]
pub struct CompilerError {
    message: String,
}
impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

pub struct Compiler {
    locals: Vec<LocalVar>,
    scope_depth: i32,
    code: CodeBlock,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            locals: Vec::new(),
            scope_depth: 0,
            code: CodeBlock::create(),
        }
    }
    pub fn compile(&mut self, program: ast::Program) -> Result<CodeBlock, CompilerError> {
        self.compile_program(program)?;
        Ok(self.code.clone())
    }

    pub fn push_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn pop_scope(&mut self) -> Vec<Op> {
        self.scope_depth -= 1;
        let pop_count = self
            .locals
            .iter()
            .filter(|l| l.depth > self.scope_depth)
            .count();

        self.locals.truncate(self.locals.len() - pop_count);
        let ops: Vec<Op> = [Op::Pop].repeat(pop_count);
        ops
    }

    pub fn find_local(&mut self, name: String) -> Option<usize> {
        for (idx, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                return Some(idx);
            }
        }
        None
    }

    pub fn declare_local(&mut self, name: String) {
        self.locals.push(LocalVar::new(name, self.scope_depth))
    }

    pub fn compile_program(&mut self, program: ast::Program) -> Result<(), CompilerError> {
        for statement in program.statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    pub fn compile_statement(&mut self, statement: ast::Statement) -> Result<(), CompilerError> {
        match statement {
            ast::Statement::Assignment { id, expr } => {
                self.compile_expression(expr)?;
                self.declare_local(id);
                Ok(())
            }
        }
    }

    pub fn compile_expression(&mut self, expression: ast::Expression) -> Result<(), CompilerError> {
        match expression {
            ast::Expression::BinaryOp { op, left, right } => {
                self.compile_expression(*left)?;
                self.compile_expression(*right)?;
                let operator_op = self.binary_op_to_op(op);
                self.code.add_op(operator_op);
                Ok(())
            }
            ast::Expression::UnaryOp { op, expr } => {
                self.compile_expression(*expr)?;
                let operator_op = self.unary_op_to_op(op);
                self.code.add_op(operator_op);
                Ok(())
            }
            ast::Expression::EVal(val) => self.compile_value(val),
        }
    }

    pub fn binary_op_to_op(&mut self, op: ast::BinaryOperator) -> Op {
        match op {
            ast::BinaryOperator::Addition => Op::Add,
            ast::BinaryOperator::Subtraction => Op::Sub,
            ast::BinaryOperator::Multiplication => Op::Mult,
            ast::BinaryOperator::Division => Op::Divide,
            ast::BinaryOperator::Modulo => Op::Modulo,
        }
    }

    pub fn unary_op_to_op(&mut self, op: ast::UnaryOperator) -> Op {
        match op {
            ast::UnaryOperator::Negation => Op::Negate,
        }
    }

    pub fn compile_value(&mut self, value: ast::Value) -> Result<(), CompilerError> {
        match value {
            ast::Value::Float(v) => {
                let pos = self.code.add_constant(value::Value::Number(v));
                self.code.add_op(Op::Constant(pos));
                Ok(())
            }
            ast::Value::Integer(v) => {
                let pos = self.code.add_constant(value::Value::Number(v as f32));
                self.code.add_op(Op::Constant(pos));
                Ok(())
            }
            ast::Value::Variable(name) => match self.find_local(name) {
                None => Err(CompilerError {
                    message: String::from("Could not find variable"),
                }),
                Some(pos) => {
                    self.code.add_op(Op::Local(pos));
                    Ok(())
                }
            },
        }
    }
}

struct LocalVar {
    pub name: String,
    pub depth: i32,
}

impl LocalVar {
    pub fn new(name: String, depth: i32) -> Self {
        LocalVar { name, depth }
    }
}

#[cfg(test)]
mod tests;
