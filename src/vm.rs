use std::{collections::HashMap, error::Error, fmt};

use crate::{instruction::Instruction, number::Number};

#[derive(Debug)]
pub enum VmError {
    StackUnderflow(String),
    ProgramCounterOutOfBounds,
    CallStackEmpty,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::StackUnderflow(op) => {
                write!(f, "Stack underflow: not enough operands for {}", op)
            }

            VmError::ProgramCounterOutOfBounds => write!(f, "Program counter out of bounds"),
            VmError::CallStackEmpty => write!(f, "Call stack is empty, cannot return"),
        }
    }
}

impl Error for VmError {}

pub struct Frame {
    return_address: usize,
}

impl Frame {
    pub fn new(return_address: usize) -> Self {
        Self { return_address }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(call frame for address {})", self.return_address)
    }
}

pub struct VM<T: Number> {
    pub pc: usize,
    pub stack: Vec<T>,
    pub program: Vec<Instruction<T>>,
    pub call_stack: Vec<Frame>,
    pub variables: HashMap<String, T>,
}

impl<T> VM<T>
where
    T: Number + PartialOrd,
{
    pub fn new(program: Vec<Instruction<T>>) -> Self {
        let initial_frame = Frame::new(program.len());

        Self {
            pc: 0,
            stack: vec![],
            program,
            call_stack: vec![initial_frame],
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        while self.pc < self.program.len() {
            let instr = &self.program[self.pc].clone();

            self.pc += 1;
            self.execute_instruction(instr)?;
        }

        Ok(())
    }

    fn execute_instruction(&mut self, instr: &Instruction<T>) -> Result<(), VmError> {
        match instr {
            Instruction::Push(value) => self.stack.push(*value),
            Instruction::Add => self.binary_op(|a, b| a + b, "addition")?,
            Instruction::Subtract => self.binary_op(|a, b| b - a, "subtraction")?,
            Instruction::Divide => self.binary_op(|a, b| b / a, "division")?,
            Instruction::Multiply => self.binary_op(|a, b| a * b, "multiplication")?,
            Instruction::Print => {
                if let Some(val) = self.stack.last() {
                    println!("{}", val);
                } else {
                    println!("(empty stack)");
                }
            }

            Instruction::Store(var) => {
                let value = self
                    .stack
                    .pop()
                    .ok_or_else(|| VmError::StackUnderflow("store".to_string()))?;

                self.variables.insert(var.clone(), value);
            }

            Instruction::Load(var) => {
                let value = self.variables.get(var).ok_or_else(|| {
                    VmError::StackUnderflow(format!("variable '{}' not found", var))
                })?;

                self.stack.push(*value);
            }

            Instruction::Equal => {
                self.binary_op(|a, b| if a == b { T::from(1) } else { T::from(0) }, "equal")?
            }

            Instruction::LessThan => self.binary_op(
                |a, b| if b < a { T::from(1) } else { T::from(0) },
                "less_than",
            )?,

            Instruction::GreaterThan => self.binary_op(
                |a, b| if b > a { T::from(1) } else { T::from(0) },
                "greater_than",
            )?,

            Instruction::Dup => {
                let value = *self
                    .stack
                    .last()
                    .ok_or_else(|| VmError::StackUnderflow("dup".to_string()))?;

                self.stack.push(value);
            }

            Instruction::Swap => {
                let len = self.stack.len();

                if len < 2 {
                    return Err(VmError::StackUnderflow("swap".to_string()));
                }

                self.stack.swap(len - 1, len - 2);
            }

            Instruction::Pop => {
                self.stack
                    .pop()
                    .ok_or_else(|| VmError::StackUnderflow("pop".to_string()))?;
            }

            Instruction::Jump(addr) => self.jump(*addr)?,
            Instruction::Call(addr) => self.call(*addr)?,
            Instruction::ConditionalJump(addr) => self.conditional_jump(*addr)?,
            Instruction::Return => self.ret()?,

            Instruction::Halt => self.pc = self.program.len(),
        }

        Ok(())
    }

    fn binary_op<F>(&mut self, op: F, op_name: &str) -> Result<(), VmError>
    where
        F: Fn(T, T) -> T,
    {
        let a = self
            .stack
            .pop()
            .ok_or_else(|| VmError::StackUnderflow(op_name.to_string()))?;

        let b = self
            .stack
            .pop()
            .ok_or_else(|| VmError::StackUnderflow(op_name.to_string()))?;

        self.stack.push(op(a, b));

        Ok(())
    }

    fn jump(&mut self, addr: usize) -> Result<(), VmError> {
        if addr >= self.program.len() {
            return Err(VmError::ProgramCounterOutOfBounds);
        }

        self.pc = addr;

        Ok(())
    }

    fn call(&mut self, addr: usize) -> Result<(), VmError> {
        if addr >= self.program.len() {
            return Err(VmError::ProgramCounterOutOfBounds);
        }

        self.call_stack.push(Frame::new(self.pc));
        self.pc = addr;

        Ok(())
    }

    fn conditional_jump(&mut self, addr: usize) -> Result<(), VmError> {
        let condition = self
            .stack
            .pop()
            .ok_or_else(|| VmError::StackUnderflow("conditional_jump".to_string()))?;

        if condition != T::from(0) {
            self.jump(addr)?;
        }

        Ok(())
    }

    fn ret(&mut self) -> Result<(), VmError> {
        let frame = self.call_stack.pop().ok_or(VmError::CallStackEmpty)?;
        self.pc = frame.return_address;

        Ok(())
    }

    #[cfg(debug_assertions)]
    pub fn visualize_callstack(&self) -> String {
        if self.call_stack.is_empty() {
            "(empty call stack)".to_string()
        } else {
            let mut s = String::from("call stack (top to bottom):\n");

            for (i, addr) in self.call_stack.iter().rev().enumerate() {
                s.push_str(&format!("  {}: return to instruction {}\n", i, addr));
            }

            s
        }
    }
}
