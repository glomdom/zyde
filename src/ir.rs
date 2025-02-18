use std::collections::HashMap;

use crate::{instruction::Instruction, number::Number};

#[derive(Debug, Clone)]
pub enum IR<T: Number> {
    Push(T),
    Add,
    Subtract,
    Multiply,
    Divide,
    Print,

    Jump(String),
    Call(String),
    ConditionalJump(String),
    Label(String),
    Return,

    Halt,
}

pub fn parse_ir<T: Number>(input: &str) -> Vec<IR<T>> {
    let mut ir_insts = Vec::new();

    for (lineno, line) in input.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0].to_uppercase().as_str() {
            "PUSH" => {
                if parts.len() != 2 {
                    panic!("L{}: PUSH requires one operand", lineno + 1);
                }

                let num = i32::from_str_radix(parts[1], 10)
                    .unwrap_or_else(|_| panic!("L{}: invalid number for PUSH", lineno + 1));

                let value = T::from(num);

                ir_insts.push(IR::Push(value));
            }

            "ADD" => ir_insts.push(IR::Add),
            "SUBTRACT" => ir_insts.push(IR::Subtract),
            "MULTIPLY" => ir_insts.push(IR::Multiply),
            "DIVIDE" => ir_insts.push(IR::Divide),
            "PRINT" => ir_insts.push(IR::Print),

            "JUMP" => {
                if parts.len() != 2 {
                    panic!("L{}: JUMP requires one operand", lineno + 1);
                }

                ir_insts.push(IR::Jump(parts[1].to_string()));
            }

            "CALL" => {
                if parts.len() != 2 {
                    panic!("L{}: CALL requires one operand", lineno + 1);
                }

                ir_insts.push(IR::Call(parts[1].to_string()));
            }

            "CJUMP" => {
                if parts.len() != 2 {
                    panic!("L{}: CJUMP requires one operand", lineno + 1);
                }

                ir_insts.push(IR::ConditionalJump(parts[1].to_string()));
            }

            "RETURN" => ir_insts.push(IR::Return),
            "HALT" => ir_insts.push(IR::Halt),
            "LABEL" => {
                if parts.len() != 2 {
                    panic!("L{}: LABEL requires one operand", lineno + 1);
                }

                ir_insts.push(IR::Label(parts[1].to_string()));
            }

            other => {
                panic!("L{}: unknown instruction '{}'", lineno + 1, other);
            }
        }
    }

    ir_insts
}

pub fn assemble<T: Number>(input: &str) -> Vec<crate::instruction::Instruction<T>> {
    let ir_insts = parse_ir(input);

    let mut label_map: HashMap<String, usize> = HashMap::new();
    let mut curr_index = 0;

    for inst in &ir_insts {
        if let IR::Label(name) = inst {
            label_map.insert(name.clone(), curr_index);
        } else {
            curr_index += 1;
        }
    }

    let mut final_insts = Vec::new();
    for inst in ir_insts {
        match inst {
            IR::Push(value) => final_insts.push(Instruction::Push(value)),
            IR::Add => final_insts.push(Instruction::Add),
            IR::Subtract => final_insts.push(Instruction::Subtract),
            IR::Multiply => final_insts.push(Instruction::Multiply),
            IR::Divide => final_insts.push(Instruction::Divide),
            IR::Print => final_insts.push(Instruction::Print),

            IR::Jump(label) => {
                let target = label_map
                    .get(&label)
                    .unwrap_or_else(|| panic!("undefined label: {}", label));

                final_insts.push(Instruction::Jump(*target));
            }

            IR::Call(label) => {
                let target = label_map
                    .get(&label)
                    .unwrap_or_else(|| panic!("undefined label: {}", label));

                final_insts.push(Instruction::Call(*target));
            }

            IR::ConditionalJump(label) => {
                let target = label_map
                    .get(&label)
                    .unwrap_or_else(|| panic!("undefined label: {}", label));

                final_insts.push(Instruction::ConditionalJump(*target));
            }

            IR::Return => final_insts.push(Instruction::Return),
            IR::Halt => final_insts.push(Instruction::Halt),
            IR::Label(_) => {}
        }
    }

    final_insts
}
