mod instruction;
mod vm;

use instruction::Instruction;
use vm::VM;

fn main() {
    let program = vec![
        Instruction::Push(10),
        Instruction::Push(59),
        Instruction::Call(5),
        Instruction::Print,     // expected 69
        Instruction::Halt,

        // function foo:
        Instruction::Add,
        Instruction::Return,
    ];

    let mut vm = VM::new(program);
    if let Err(e) = vm.run() {
        eprintln!("VM error: {}", e);
    }
}
