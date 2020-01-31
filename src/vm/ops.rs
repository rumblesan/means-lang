#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Pop,
    Add,
    Sub,
    Mult,
    Divide,
    Modulo,
    Negate,
    Constant(usize),
    Print,
    Local(usize),
}
