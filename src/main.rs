use clap::Parser;
use zyde::{instruction::Instruction, vm::VM};

#[derive(Parser)]
#[command(author, version, about = "Assembles IR code into zyde instructions", long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 10 },
        Instruction::Call(4),
        Instruction::Print { src: 0 },
        Instruction::Halt,
        Instruction::LoadImm { dest: 1, value: 42 },
        Instruction::Print { src: 1 },
        Instruction::Return,
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 8);
    if let Err(e) = vm.run() {
        eprintln!("VM error: {}", e);
    }

    #[cfg(debug_assertions)]
    println!("{}", vm.visualize_callstack());
}
