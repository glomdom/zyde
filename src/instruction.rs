#[derive(Debug, Clone)]
pub enum Instruction {
    /// Load an immediate constant into register `dest`
    LoadImm { dest: usize, value: f64 },

    /// dest = src1 + src2
    Add {
        dest: usize,
        src1: usize,
        src2: usize,
    },

    /// dest = src1 - src2
    Sub {
        dest: usize,
        src1: usize,
        src2: usize,
    },

    /// dest = src1 * src2
    Mul {
        dest: usize,
        src1: usize,
        src2: usize,
    },

    /// dest = src1 / src2
    Div {
        dest: usize,
        src1: usize,
        src2: usize,
    },

    /// Print the contents of register `src`
    Print { src: usize },

    /// Unconditional jump to instruction at `addr`
    Jump(usize),

    /// Call a subroutine at instruction `addr`
    Call { addr: usize },

    /// If the value in register `cond` equals 0, jump to `target`
    ConditionalJump { cond: usize, target: usize },

    /// Return from a subroutine
    Return,

    /// Store the value from register `src` into variable `var`
    Store { src: usize, var: String },

    /// Load the value of variable `var` into register `dest`
    Load { dest: usize, var: String },

    /// Copy the value from register `src` to `dest`
    Mov { dest: usize, src: usize },

    /// Set register `dest` to 1 if reg[src1] == reg[src2], else 0
    Equal {
        dest: usize,
        src1: usize,
        src2: usize,
    },

    /// Set register `dest` to 1 if reg[src1] < reg[src2], else 0
    LessThan {
        dest: usize,
        src1: usize,
        src2: usize,
    },

    /// Set register `dest` to 1 if reg[src1] > reg[src2], else 0
    GreaterThan {
        dest: usize,
        src1: usize,
        src2: usize,
    },

    /// Set register `dest` to the logical NOT of reg[src]
    Not { dest: usize, src: usize },

    /// Stop execution
    Halt,
}
