#[derive(Debug)]
pub enum VMError {
    NoError,
    NoConstant,
    StackOver,
    StackUnder,
}
