use zyde::{vm::VM, ir::assemble};
use pretty_assertions::assert_eq;

#[test]
fn test_arithmetic() {
    let program = "\
            PUSH 10
            PUSH 20
            ADD
            HALT
        ";
    
    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    vm.run().unwrap();
    
    assert_eq!(vm.stack, vec![30]);
}

#[test]
fn test_variables_and_comparisons() {
    let program = "\
            PUSH 15
            STORE x
            PUSH 20
            STORE y

            LOAD x
            LOAD y
            LT
            HALT
        ";

    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![1]);
    assert_eq!(vm.variables.get("x"), Some(&15));
    assert_eq!(vm.variables.get("y"), Some(&20));
}

#[test]
fn test_stack_manipulation() {
    let program = "\
            PUSH 42
            DUP
            PUSH 99
            SWAP
            POP
            HALT
        ";
    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![42, 99]);
}
