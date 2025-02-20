#[derive(Debug, Clone)]
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

    Store(String),
    Load(String),
    Equal,
    LessThan,
    GreaterThan,
    Dup,
    Swap,
    Pop,

    Halt,
}