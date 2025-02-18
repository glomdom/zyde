#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Instruction<T: Copy> {
    Push(T),
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