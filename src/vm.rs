use crate::instruction::Instruction;
use crate::number::Number;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VmError {
    RegisterOutOfBounds(String),
    ProgramCounterOutOfBounds,
    CallStackEmpty,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::RegisterOutOfBounds(msg) => write!(f, "Register error: {}", msg),
            VmError::ProgramCounterOutOfBounds => write!(f, "Program counter out of bounds"),
            VmError::CallStackEmpty => write!(f, "Call stack is empty, cannot return"),
        }
    }
}

impl Error for VmError {}

#[derive(Debug)]
pub struct Frame {
    return_address: usize,
}

impl Frame {
    pub fn new(return_address: usize) -> Self {
        Self { return_address }
    }
}

/// A register–based virtual machine
pub struct VM<T: Number> {
    pub pc: usize,

    pub registers: Vec<T>,
    pub program: Vec<Instruction<T>>,
    pub call_stack: Vec<Frame>,
    pub variables: HashMap<String, T>,
}

impl<T> VM<T>
where
    T: Number + PartialOrd + From<i32>,
{
    pub fn new(program: Vec<Instruction<T>>, num_registers: usize) -> Self {
        let registers = vec![T::from(0); num_registers];

        Self {
            pc: 0,
            registers,
            program,
            call_stack: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        while self.pc < self.program.len() {
            let instr = self.program[self.pc].clone();
            self.pc += 1;
            self.execute_instruction(instr)?;
        }

        Ok(())
    }

    fn execute_instruction(&mut self, instr: Instruction<T>) -> Result<(), VmError> {
        match instr {
            Instruction::LoadImm { dest, value } => self.set_register(dest, value)?,
            Instruction::Add { dest, src1, src2 } => {
                let v1 = self.get_register(src1)?;
                let v2 = self.get_register(src2)?;

                self.set_register(dest, v1 + v2)?;
            }

            Instruction::Sub { dest, src1, src2 } => {
                let v1 = self.get_register(src1)?;
                let v2 = self.get_register(src2)?;

                self.set_register(dest, v1 - v2)?;
            }

            Instruction::Mul { dest, src1, src2 } => {
                let v1 = self.get_register(src1)?;
                let v2 = self.get_register(src2)?;

                self.set_register(dest, v1 * v2)?;
            }

            Instruction::Div { dest, src1, src2 } => {
                let v1 = self.get_register(src1)?;
                let v2 = self.get_register(src2)?;

                self.set_register(dest, v1 / v2)?;
            }

            Instruction::Print { src } => {
                let value = self.get_register(src)?;

                println!("{}", value);
            }

            Instruction::Jump(addr) => self.jump(addr)?,
            Instruction::Call(addr) => self.call(addr)?,
            Instruction::ConditionalJump { cond, target } => {
                let condition = self.get_register(cond)?;

                if condition == T::from(0) {
                    self.jump(target)?;
                }
            }

            Instruction::Return => self.ret()?,
            Instruction::Store { src, var } => {
                let value = self.get_register(src)?;

                self.variables.insert(var, value);
            }

            Instruction::Load { dest, var } => {
                let value = self.variables.get(&var).ok_or_else(|| {
                    VmError::RegisterOutOfBounds(format!("variable '{}' not found", var))
                })?;

                self.set_register(dest, *value)?;
            }

            Instruction::Equal { dest, src1, src2 } => {
                let v1 = self.get_register(src1)?;
                let v2 = self.get_register(src2)?;
                let result = if v1 == v2 { T::from(1) } else { T::from(0) };

                self.set_register(dest, result)?;
            }

            Instruction::LessThan { dest, src1, src2 } => {
                let v1 = self.get_register(src1)?;
                let v2 = self.get_register(src2)?;
                let result = if v1 < v2 { T::from(1) } else { T::from(0) };

                self.set_register(dest, result)?;
            }

            Instruction::GreaterThan { dest, src1, src2 } => {
                let v1 = self.get_register(src1)?;
                let v2 = self.get_register(src2)?;
                let result = if v1 > v2 { T::from(1) } else { T::from(0) };

                self.set_register(dest, result)?;
            }

            Instruction::Not { dest, src } => {
                let v = self.get_register(src)?;
                let result = if v == T::from(0) {
                    T::from(1)
                } else {
                    T::from(0)
                };

                self.set_register(dest, result)?;
            }

            Instruction::Halt => self.pc = self.program.len(),
        }

        Ok(())
    }

    fn get_register(&self, index: usize) -> Result<T, VmError> {
        self.registers.get(index).copied().ok_or_else(|| {
            VmError::RegisterOutOfBounds(format!("invalid register index {}", index))
        })
    }

    fn set_register(&mut self, index: usize, value: T) -> Result<(), VmError> {
        if let Some(reg) = self.registers.get_mut(index) {
            *reg = value;

            Ok(())
        } else {
            Err(VmError::RegisterOutOfBounds(format!(
                "invalid register index {}",
                index
            )))
        }
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

            for (i, frame) in self.call_stack.iter().rev().enumerate() {
                s.push_str(&format!(
                    "  {}: return to instruction {}\n",
                    i, frame.return_address
                ));
            }

            s
        }
    }
}
