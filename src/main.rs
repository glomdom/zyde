mod instruction;
mod vm;
mod ir;
mod number;

use std::fs;

use clap::Parser;
use ir::assemble;
use vm::VM;

#[derive(Parser)]
#[command(author, version, about = "Assembles IR code into zyde instructions", long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(&args.input)
        .expect("failed to read input IR file");

    let instructions = assemble::<i32>(&content);
    // for (i, inst) in instructions.iter().enumerate() {
    //     println!("{}: {:?}", i, inst);
    // }

    let mut vm = VM::new(instructions);
    if let Err(e) = vm.run() {
        eprintln!("VM error: {}", e);
    }

    #[cfg(debug_assertions)]
    {
        println!("{}", vm.visualize_callstack());
    }
}