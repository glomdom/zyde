use crate::instruction::Instruction;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VmError {
    RegisterOutOfBounds(String),
    ProgramCounterOutOfBounds,
    CallStackEmpty,
    VariableNotFound(String),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::RegisterOutOfBounds(msg) => write!(f, "Register error: {}", msg),
            VmError::ProgramCounterOutOfBounds => write!(f, "Program counter out of bounds"),
            VmError::CallStackEmpty => write!(f, "Call stack is empty, cannot return"),
            VmError::VariableNotFound(name) => write!(f, "Variable '{}' not found", name),
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

/// A register–based virtual machine using f64 for all values
pub struct VM {
    pub pc: usize,
    pub registers: Vec<f64>,
    pub program: Vec<Instruction>,
    pub call_stack: Vec<Frame>,
    pub variables: HashMap<String, f64>,
}

impl VM {
    pub fn new(program: Vec<Instruction>, num_registers: usize) -> Self {
        Self {
            pc: 0,
            registers: vec![0.0; num_registers],
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

    fn execute_instruction(&mut self, instr: Instruction) -> Result<(), VmError> {
        use Instruction::*;
        match instr {
            LoadImm { dest, value } => self.set_register(dest, value)?,
            Add { dest, src1, src2 } => {
                let v = self.get_register(src1)? + self.get_register(src2)?;
                self.set_register(dest, v)?;
            }
            Sub { dest, src1, src2 } => {
                let v = self.get_register(src1)? - self.get_register(src2)?;
                self.set_register(dest, v)?;
            }
            Mul { dest, src1, src2 } => {
                let v = self.get_register(src1)? * self.get_register(src2)?;
                self.set_register(dest, v)?;
            }
            Div { dest, src1, src2 } => {
                let v = self.get_register(src1)? / self.get_register(src2)?;
                self.set_register(dest, v)?;
            }
            Print { src } => println!("{}", self.get_register(src)?),
            Jump(addr) => self.jump(addr)?,
            Call { addr } => self.call(addr)?,
            ConditionalJump { cond, target } => {
                if self.get_register(cond)? == 0.0 {
                    self.jump(target)?;
                }
            }
            Return => self.ret()?,
            Store { src, var } => {
                let val = self.get_register(src)?;
                self.variables.insert(var, val);
            }
            Load { dest, var } => {
                let val = *self
                    .variables
                    .get(&var)
                    .ok_or(VmError::VariableNotFound(var))?;
                self.set_register(dest, val)?;
            }
            Mov { dest, src } => {
                let val = self.get_register(src)?;
                self.set_register(dest, val)?;
            }
            Equal { dest, src1, src2 } => {
                let v = if self.get_register(src1)? == self.get_register(src2)? {
                    1.0
                } else {
                    0.0
                };
                self.set_register(dest, v)?;
            }
            LessThan { dest, src1, src2 } => {
                let v = if self.get_register(src1)? < self.get_register(src2)? {
                    1.0
                } else {
                    0.0
                };
                self.set_register(dest, v)?;
            }
            GreaterThan { dest, src1, src2 } => {
                let v = if self.get_register(src1)? > self.get_register(src2)? {
                    1.0
                } else {
                    0.0
                };
                self.set_register(dest, v)?;
            }
            Not { dest, src } => {
                let v = if self.get_register(src)? == 0.0 {
                    1.0
                } else {
                    0.0
                };
                self.set_register(dest, v)?;
            }
            Halt => self.pc = self.program.len(),
        }
        Ok(())
    }

    fn get_register(&self, index: usize) -> Result<f64, VmError> {
        self.registers.get(index).copied().ok_or_else(|| {
            VmError::RegisterOutOfBounds(format!("invalid register index {}", index))
        })
    }

    fn set_register(&mut self, index: usize, value: f64) -> Result<(), VmError> {
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
            Err(VmError::ProgramCounterOutOfBounds)
        } else {
            self.pc = addr;
            Ok(())
        }
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
            let mut s = String::from("call stack:\n");
            for (i, frame) in self.call_stack.iter().rev().enumerate() {
                s.push_str(&format!(
                    "  frame {}: return address -> {}\n",
                    i, frame.return_address
                ));
            }
            s
        }
    }
}
