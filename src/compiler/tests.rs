use super::*;

#[test]
fn test_scope() {
    let mut compiler = Compiler::new();
    compiler.declare_local(String::from("var1"));
    compiler.declare_local(String::from("var2"));
    compiler.push_scope();
    compiler.declare_local(String::from("var3"));
    assert_eq!(compiler.locals.len(), 3);
    let push_ops = compiler.pop_scope();
    assert_eq!(push_ops, vec![Op::Pop]);
    assert_eq!(compiler.locals.len(), 2);
}
