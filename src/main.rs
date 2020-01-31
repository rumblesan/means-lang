use std::process;

use meanslang::compiler::Compiler;
use meanslang::parser::lexer::new_lexer_builder;
use meanslang::parser::MeansParser;
use meanslang::vm::MeansVM;

fn main() {
    let program = r"
foo = 3 + (4.0 * 5);
bar = foo * 2 / foo;
";
    println!("compiling and running {}", program);

    let lb = new_lexer_builder();
    let lexer = lb.build(program);
    let mut parser = MeansParser::new(lexer);
    let mut vm = MeansVM::create();
    let mut compiler = Compiler::new();

    let ast = match parser.parse() {
        Err(errs) => {
            for e in errs {
                println!("{}", e);
            }
            process::exit(1)
        }
        Ok(ast) => ast,
    };

    let code = match compiler.compile(ast) {
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
        Ok(ast) => ast,
    };

    let ret_val = vm.run(&code);

    println!("return value is {}", ret_val);
    if ret_val != 0 {
        vm.print_error();
    }
    match vm.peek_stack() {
        None => println!("No Stack!"),
        Some(v) => println!("Top of stack: {}", v),
    }
}
