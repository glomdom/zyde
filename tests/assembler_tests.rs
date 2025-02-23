use std::collections::HashMap;

use pretty_assertions::assert_eq;
use zyde::{
    instruction::Instruction,
    ir::{IR, assemble, lower_control_flow, parse_ir},
    number::Number,
    vm::{VM, VmError},
};

#[test]
fn test_arithmetic() {
    let program = "\
            PUSH 10
            PUSH 20
            ADD
            HALT
        ";

    let lowered = lower_control_flow(parse_ir::<i32>(program));
    let final_insts = assemble_lowered(lowered);
    let mut vm = VM::new(final_insts);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![30]);
}

#[test]
fn test_if_else_true() {
    let program = "\
        PUSH 10
        PUSH 10
        EQUAL
        IF
          PUSH 42
        ELSE
          PUSH 0
        ENDIF
        HALT
    ";

    let lowered = lower_control_flow(parse_ir::<i32>(program));
    let final_insts = assemble_lowered(lowered);
    let mut vm = VM::new(final_insts);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![42]);
}

#[test]
fn test_if_else_false() {
    let program = "\
            PUSH 10
            PUSH 20
            EQUAL
            IF
              PUSH 1
            ELSE
              PUSH 99
            ENDIF
            HALT
        ";

    let lowered = lower_control_flow(parse_ir::<i32>(program));
    let final_insts = assemble_lowered(lowered);
    let mut vm = VM::new(final_insts);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![99]);
}

#[test]
fn test_variables_and_comparisons() {
    let program = "\
            PUSH 15
            STORE x
            PUSH 20
            STORE y

            LOAD x
            LOAD y
            LT
            HALT
        ";

    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![1]);
    assert_eq!(vm.variables.get("x"), Some(&15));
    assert_eq!(vm.variables.get("y"), Some(&20));
}

#[test]
fn test_stack_manipulation() {
    let program = "\
            PUSH 42
            DUP
            PUSH 99
            SWAP
            POP
            HALT
        ";

    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![42, 99]);
}

#[test]
fn test_ir_lowering_debug() {
    let program = "\
            PUSH 10 ; push 10
            PUSH 20 ; push 20
            ADD     ; add them
            HALT
        ";

    let ir = parse_ir::<i32>(program);
    let lowered = lower_control_flow(ir);

    if let Some(last) = lowered.last() {
        match last {
            IR::Halt => (),

            _ => panic!("Expected HALT at end of lowered IR"),
        }
    } else {
        panic!("Lowered IR is empty");
    }
}

#[test]
fn test_function_call() {
    let program = "\
        CALL func
        HALT
        LABEL func
        PUSH 42
        RETURN";

    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![42]);
}

#[test]
fn test_stack_underflow() {
    let program = "ADD";
    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    let result = vm.run();

    assert!(matches!(result, Err(VmError::StackUnderflow(_))));
}

#[test]
fn test_not_instruction() {
    let program = "\
        PUSH 0
        NOT
        PUSH 1
        NOT
        HALT";

    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![1, 0]);
}

#[test]
fn test_invalid_return() {
    let program = "RETURN";
    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    let result = vm.run();

    assert!(matches!(result, Err(VmError::CallStackEmpty)));
}

#[test]
fn test_do_loop() {
    let program = "\
        PUSH 3
        DO
            DUP
            PRINT
            PUSH 1
            SUBTRACT
            DUP
            PUSH 0
            GT
        ENDDO
        HALT";

    let instructions = assemble::<i32>(program);
    let mut vm = VM::new(instructions);
    vm.run().unwrap();

    assert_eq!(vm.stack, vec![3, 2, 1, 0]);
}

fn assemble_lowered<T: Number>(lowered: Vec<IR<T>>) -> Vec<Instruction<T>> {
    let mut label_map: HashMap<String, usize> = HashMap::new();
    let mut curr_index = 0;

    for inst in &lowered {
        if let IR::Label(name) = inst {
            label_map.insert(name.clone(), curr_index);
        } else {
            curr_index += 1;
        }
    }

    let mut final_insts = Vec::new();

    for inst in lowered {
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
            IR::Not => final_insts.push(Instruction::Not),
            IR::Equal => final_insts.push(Instruction::Equal),
            IR::Label(_) => {}

            other => panic!("Unexpected IR instruction in lowered IR: {:?}", other),
        }
    }
    final_insts
}
