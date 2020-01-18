use crate::code_block::CodeBlock;
use crate::ops::Op;
use crate::value::Value;
use crate::vm_error::VMError;

const VM_STACK_SIZE: usize = 100;

pub struct MeansVM {
    pc: u32,
    exit_code: u32,
    error: VMError,
    sc: usize,
    stack: [Value; VM_STACK_SIZE],
}

macro_rules! incr_stack {
    ($vm_var:ident) => {
        if $vm_var.sc == VM_STACK_SIZE {
            $vm_var.set_error(VMError::StackOver);
            break;
        }
        $vm_var.sc += 1;
    };
}

macro_rules! decr_stack {
    ($vm_var:ident) => {
        if $vm_var.sc == 0 {
            $vm_var.set_error(VMError::StackUnder);
            break;
        }
        $vm_var.sc -= 1;
    };
}

macro_rules! vm_push {
    ($vm_var:ident, $val:expr) => {
        $vm_var.stack[$vm_var.sc] = $val;
        incr_stack!($vm_var);
    };
}

macro_rules! vm_pop {
    ($vm_var:ident, $var_name:ident) => {
        decr_stack!($vm_var);
        let $var_name = $vm_var.stack[$vm_var.sc];
    };
}

macro_rules! binary_op {
    ($vm_var:ident, $op:tt) => {{
        vm_pop!($vm_var, v1);
        vm_pop!($vm_var, v2);
        let newv = match (v1, v2) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 $op n2)
        };
        vm_push!($vm_var, newv);
    }}
}

macro_rules! unary_op {
    ($vm_var:ident, $op:tt) => {{
        vm_pop!($vm_var, v);
        let newv = match (v) {
            Value::Number(n) => Value::Number($op n)
        };
        vm_push!($vm_var, newv);
    }}
}

impl MeansVM {
    pub fn create() -> MeansVM {
        MeansVM {
            pc: 0,
            exit_code: 0,
            error: VMError::NoError,
            sc: 0,
            stack: [Value::Number(0.0); VM_STACK_SIZE],
        }
    }
    pub fn run(&mut self, blk: &mut CodeBlock) -> u32 {
        for op in &blk.code {
            println!("op -> {:?}", op);
            match op {
                Op::Pop => {
                    decr_stack!(self);
                }
                Op::Add => binary_op!(self, +),
                Op::Sub => binary_op!(self, -),
                Op::Mult => binary_op!(self, *),
                Op::Divide => binary_op!(self, /),
                Op::Modulo => binary_op!(self, %),
                Op::Negate => unary_op!(self, -),
                Op::Constant(p) => match blk.constants.get(*p) {
                    None => {
                        vm_error!(self, VMError::NoConstant);
                    }
                    Some(v) => {
                        println!("position: {:?}, value: {:?}", p, v);
                        self.stack[self.sc] = *v;
                        incr_stack!(self);
                    }
                },
                Op::Print => {
                    decr_stack!(self);
                    let v = self.stack[self.sc];
                    println!("Printing {:?}", v);
                }
            }
        }
        self.exit_code
    }
    pub fn set_error(&mut self, error: VMError) {
        self.exit_code = 1;
        self.error = error;
    }

    pub fn print_error(&self) {
        println!("ERROR: {:?} at {}", self.error, self.pc);
    }
}
