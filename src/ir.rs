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

    Store(String),
    Load(String),
    Equal,
    LessThan,
    GreaterThan,
    Dup,
    Swap,
    Pop,

    If,
    Else,
    EndIf,
    While,
    EndWhile,
    Do,
    EndDo,

    Not,

    Halt,
}

pub fn parse_ir<T: Number>(input: &str) -> Vec<IR<T>> {
    let mut ir_insts = Vec::new();

    for (lineno, line) in input.lines().enumerate() {
        let line = line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
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

            "STORE" => {
                if parts.len() != 2 {
                    panic!("L{}: STORE requires one operand", lineno + 1);
                }

                ir_insts.push(IR::Store(parts[1].to_string()));
            }

            "LOAD" => {
                if parts.len() != 2 {
                    panic!("L{}: LOAD requires one operand", lineno + 1);
                }

                ir_insts.push(IR::Load(parts[1].to_string()));
            }

            "EQUAL" => ir_insts.push(IR::Equal),
            "LT" => ir_insts.push(IR::LessThan),
            "GT" => ir_insts.push(IR::GreaterThan),
            "DUP" => ir_insts.push(IR::Dup),
            "SWAP" => ir_insts.push(IR::Swap),
            "POP" => ir_insts.push(IR::Pop),

            "LABEL" => {
                if parts.len() != 2 {
                    panic!("L{}: LABEL requires one operand", lineno + 1);
                }

                ir_insts.push(IR::Label(parts[1].to_string()));
            }

            "IF" => ir_insts.push(IR::If),
            "ELSE" => ir_insts.push(IR::Else),
            "ENDIF" => ir_insts.push(IR::EndIf),
            "WHILE" => ir_insts.push(IR::While),
            "ENDWHILE" => ir_insts.push(IR::EndWhile),
            "DO" => ir_insts.push(IR::Do),
            "ENDDO" => ir_insts.push(IR::EndDo),

            "NOT" => ir_insts.push(IR::Not),

            other => {
                panic!("L{}: unknown instruction '{}'", lineno + 1, other);
            }
        }
    }

    ir_insts
}

pub fn assemble<T: Number>(input: &str) -> Vec<crate::instruction::Instruction<T>> {
    let ir_insts = parse_ir(input);
    let lowered_ir = lower_control_flow(ir_insts);
    let mut label_map: HashMap<String, usize> = HashMap::new();
    let mut curr_index = 0;

    for inst in &lowered_ir {
        if let IR::Label(name) = inst {
            label_map.insert(name.clone(), curr_index);
        } else {
            curr_index += 1;
        }
    }

    let mut final_insts = Vec::new();
    for inst in lowered_ir {
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

            IR::Store(var) => final_insts.push(Instruction::Store(var)),
            IR::Load(var) => final_insts.push(Instruction::Load(var)),
            IR::Equal => final_insts.push(Instruction::Equal),
            IR::LessThan => final_insts.push(Instruction::LessThan),
            IR::GreaterThan => final_insts.push(Instruction::GreaterThan),
            IR::Dup => final_insts.push(Instruction::Dup),
            IR::Swap => final_insts.push(Instruction::Swap),
            IR::Pop => final_insts.push(Instruction::Pop),

            IR::Not => final_insts.push(Instruction::Not),

            IR::Label(_) => {}

            IR::If | IR::Else | IR::EndIf | IR::While | IR::EndWhile | IR::Do | IR::EndDo => {
                panic!("Unlowered control flow construct found")
            }
        }
    }

    final_insts
}

pub fn lower_control_flow<T: Number>(ir: Vec<IR<T>>) -> Vec<IR<T>> {
    let mut output = Vec::new();
    let mut control_stack: Vec<(&str, usize, String)> = Vec::new();

    for inst in ir {
        match inst {
            IR::If => {
                control_stack.push(("if", output.len(), String::new()));
                output.push(IR::ConditionalJump("".to_string())); // we patch this later
            }

            IR::Else => {
                if let Some(("if", if_index, _)) = control_stack.pop() {
                    let else_label = format!("L{}", output.len());
                    output[if_index] = IR::ConditionalJump(else_label.clone());
                    control_stack.push(("endif", output.len(), String::new()));
                    output.push(IR::Jump("".to_string())); // we patch this later
                    output.push(IR::Label(else_label));
                } else {
                    panic!("ELSE without matching IF");
                }
            }

            IR::EndIf => {
                if let Some(("endif", jump_index, _)) = control_stack.pop() {
                    let endif_label = format!("L{}", output.len());
                    output[jump_index] = IR::Jump(endif_label.clone());
                    output.push(IR::Label(endif_label));
                } else if let Some(("if", if_index, _)) = control_stack.pop() {
                    let endif_label = format!("L{}", output.len());
                    output[if_index] = IR::ConditionalJump(endif_label.clone());
                    output.push(IR::Label(endif_label));
                } else {
                    panic!("ENDIF without matching IF/ELSE");
                }
            }

            IR::While => {
                let loop_start = format!("L{}", output.len());
                output.push(IR::Label(loop_start.clone()));

                let cond_jump_index = output.len();
                output.push(IR::ConditionalJump("".to_string()));
                control_stack.push(("while", cond_jump_index, loop_start));
            }

            IR::EndWhile => {
                if let Some(("while", cond_jump_index, loop_start)) = control_stack.pop() {
                    output.push(IR::Jump(loop_start.clone()));

                    let exit_label = format!("L{}", output.len());
                    output[cond_jump_index] = IR::ConditionalJump(exit_label.clone());
                    output.push(IR::Label(exit_label));
                } else {
                    panic!("ENDWHILE without matching WHILE");
                }
            }

            IR::Do => {
                let loop_start = format!("L{}", output.len());
                output.push(IR::Label(loop_start.clone()));
                control_stack.push(("do", output.len(), loop_start));
            }

            IR::EndDo => {
                if let Some(("do", _, loop_start)) = control_stack.pop() {
                    let exit_label = format!("L{}", output.len());
                    output.push(IR::ConditionalJump(exit_label.clone()));
                    output.push(IR::Jump(loop_start.clone()));
                    output.push(IR::Label(exit_label));
                } else {
                    panic!("ENDDO without matching DO");
                }
            }

            other => {
                output.push(other);
            }
        }
    }

    if !control_stack.is_empty() {
        panic!("Mismatched control flow constructs");
    }

    output
}
