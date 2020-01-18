#[derive(Debug)]
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
}
