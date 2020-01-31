use super::ops::Op;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct CodeBlock {
    pub code: Vec<Op>,
    pub constants: Vec<Value>,
}

impl CodeBlock {
    pub fn create() -> CodeBlock {
        CodeBlock {
            code: vec![],
            constants: vec![],
        }
    }

    pub fn add_op(&mut self, op: Op) {
        self.code.push(op);
    }
    pub fn add_constant(&mut self, val: Value) -> usize {
        self.constants.push(val);
        self.constants.len() - 1
    }
}
