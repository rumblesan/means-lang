use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Number(f32),
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
