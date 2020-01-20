use meanslang::parser::lexer::MeansLexer;
use meanslang::parser::tokens::Token;
use meanslang::vm::code_block::CodeBlock;
use meanslang::vm::ops::Op;
use meanslang::vm::value::Value;
use meanslang::vm::MeansVM;

fn main() {
    let l = MeansLexer::create();
    let input = "3 + (4.0 * 5) - 2.134";
    println!("tokenising \n{}", input);
    let tokens: Vec<Token> = l.tokenise(&input);
    for t in &tokens {
        println!("{:?}", t);
    }

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
