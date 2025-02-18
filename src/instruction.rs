#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Instruction {
    Push(i32),
    Add,
    Subtract,
    Divide,
    Multiply,
    Print,

    Jump(usize),
    Call(usize),
    ConditionalJump(usize),
    Return,

    Halt,
}