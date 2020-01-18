mod code_block;
mod ops;
mod value;
mod vm;

use code_block::CodeBlock;
use ops::Op;
use value::Value;
use vm::MeansVM;

fn main() {
    let mut blk = CodeBlock::create();
    let n1 = blk.add_constant(Value::Number(10.0));
    let n2 = blk.add_constant(Value::Number(8.0));
    blk.add_op(Op::Constant(n1));
    blk.add_op(Op::Constant(n2));
    blk.add_op(Op::Sub);
    blk.add_op(Op::Print);

    let mut vm = MeansVM::create();

    let ret_val = vm.run(&mut blk);

    println!("return value is {}", ret_val);
    if ret_val != 0 {
        vm.print_error();
    }
}
