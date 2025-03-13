// File: tests/reg_vm_tests.rs

use zyde::instruction::Instruction;
use zyde::vm::{VM, VmError};

#[test]
fn test_loadimm() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 42 },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[0], 42);
}

#[test]
fn test_add() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 10 },
        Instruction::LoadImm { dest: 1, value: 20 },
        Instruction::Add {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[2], 30);
}

#[test]
fn test_sub() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 50 },
        Instruction::LoadImm { dest: 1, value: 8 },
        Instruction::Sub {
            dest: 2,
            src1: 0,
            src2: 1,
        }, // 50 - 8 = 42
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[2], 42);
}

#[test]
fn test_mul() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 6 },
        Instruction::LoadImm { dest: 1, value: 7 },
        Instruction::Mul {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[2], 42);
}

#[test]
fn test_div() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 84 },
        Instruction::LoadImm { dest: 1, value: 2 },
        Instruction::Div {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[2], 42);
}

#[test]
fn test_jump() {
    // This program loads 1 into reg0, jumps to instruction 3 (skipping an instruction
    // that would load 999 into reg0), then loads 42 into reg1.
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 1 },
        Instruction::Jump(3),
        Instruction::LoadImm {
            dest: 0,
            value: 999,
        },
        Instruction::LoadImm { dest: 1, value: 42 },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[0], 1);
    assert_eq!(vm.registers[1], 42);
}

#[test]
fn test_conditional_jump_taken() {
    // Condition in register 0 is 0 so the jump is taken.
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 0 },
        Instruction::ConditionalJump { cond: 0, target: 3 },
        Instruction::LoadImm {
            dest: 1,
            value: 999,
        }, // This should be skipped.
        Instruction::LoadImm { dest: 1, value: 42 },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[1], 42);
}

#[test]
fn test_conditional_jump_not_taken() {
    // Condition in register 0 is non-zero so the jump is NOT taken.
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 1 },
        Instruction::ConditionalJump { cond: 0, target: 4 },
        Instruction::LoadImm {
            dest: 1,
            value: 999,
        },
        Instruction::LoadImm { dest: 1, value: 42 },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    // The jump is not taken, so the instructions run in sequence.
    assert_eq!(vm.registers[1], 42);
}

#[test]
fn test_call_and_return() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 10 },
        Instruction::Call(4),
        Instruction::LoadImm { dest: 1, value: 42 },
        Instruction::Halt,
        Instruction::LoadImm {
            dest: 2,
            value: 100,
        },
        Instruction::Return,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], 10);
    assert_eq!(vm.registers[1], 42);
    assert_eq!(vm.registers[2], 100);
    assert_eq!(vm.call_stack.len(), 0);
}

#[test]
fn test_store_and_load() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 123,
        },
        Instruction::Store {
            src: 0,
            var: "x".to_string(),
        },
        // Reset register 0 so we can see the load effect.
        Instruction::LoadImm { dest: 0, value: 0 },
        Instruction::Load {
            dest: 1,
            var: "x".to_string(),
        },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[1], 123);
    assert_eq!(vm.variables.get("x"), Some(&123));
}

#[test]
fn test_equal() {
    // Test equality: 5 == 5 should yield 1.
    let program_true = vec![
        Instruction::LoadImm { dest: 0, value: 5 },
        Instruction::LoadImm { dest: 1, value: 5 },
        Instruction::Equal {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];
    let mut vm_true = VM::new(program_true, 4);
    vm_true.run().unwrap();
    assert_eq!(vm_true.registers[2], 1);

    // Test inequality: 5 == 7 should yield 0.
    let program_false = vec![
        Instruction::LoadImm { dest: 0, value: 5 },
        Instruction::LoadImm { dest: 1, value: 7 },
        Instruction::Equal {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];
    let mut vm_false = VM::new(program_false, 4);
    vm_false.run().unwrap();
    assert_eq!(vm_false.registers[2], 0);
}

#[test]
fn test_less_than() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 3 },
        Instruction::LoadImm { dest: 1, value: 5 },
        Instruction::LessThan {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[2], 1);

    let program_false = vec![
        Instruction::LoadImm { dest: 0, value: 5 },
        Instruction::LoadImm { dest: 1, value: 3 },
        Instruction::LessThan {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];
    let mut vm_false = VM::new(program_false, 4);
    vm_false.run().unwrap();
    assert_eq!(vm_false.registers[2], 0);
}

#[test]
fn test_greater_than() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 7 },
        Instruction::LoadImm { dest: 1, value: 5 },
        Instruction::GreaterThan {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[2], 1);

    let program_false = vec![
        Instruction::LoadImm { dest: 0, value: 3 },
        Instruction::LoadImm { dest: 1, value: 5 },
        Instruction::GreaterThan {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];
    let mut vm_false = VM::new(program_false, 4);
    vm_false.run().unwrap();
    assert_eq!(vm_false.registers[2], 0);
}

#[test]
fn test_not() {
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 0 },
        Instruction::Not { dest: 1, src: 0 },
        Instruction::LoadImm { dest: 2, value: 1 },
        Instruction::Not { dest: 3, src: 2 },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[1], 1);
    assert_eq!(vm.registers[3], 0);
}

#[test]
fn test_halt() {
    // Ensure instructions after a Halt are not executed.
    let program = vec![
        Instruction::LoadImm { dest: 0, value: 10 },
        Instruction::Halt,
        Instruction::LoadImm {
            dest: 0,
            value: 999,
        },
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    assert_eq!(vm.registers[0], 10);
}

#[test]
fn test_invalid_register() {
    // Attempt to write to an invalid register index.
    let program = vec![
        Instruction::LoadImm {
            dest: 10,
            value: 42,
        },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    let result = vm.run();
    assert!(matches!(result, Err(VmError::RegisterOutOfBounds(_))));
}

#[test]
fn test_jump_out_of_bounds() {
    // Jump to an address outside the program.
    let program = vec![Instruction::Jump::<i32>(100), Instruction::Halt];
    let mut vm = VM::new(program, 4);
    let result = vm.run();
    assert!(matches!(result, Err(VmError::ProgramCounterOutOfBounds)));
}

#[test]
fn test_return_without_call() {
    let program = vec![Instruction::Return::<i32>, Instruction::Halt];
    let mut vm = VM::new(program, 4);
    let result = vm.run();
    assert!(matches!(result, Err(VmError::CallStackEmpty)));
}

#[test]
fn test_visualize_callstack() {
    // Create a program that performs a CALL without a RETURN so that a frame remains.
    let program = vec![
        Instruction::Call(2),
        Instruction::Halt, // This instruction will not be reached.
        Instruction::LoadImm { dest: 0, value: 42 },
        Instruction::Halt,
    ];
    let mut vm = VM::new(program, 4);
    vm.run().unwrap();
    let callstack_vis = vm.visualize_callstack();
    // We expect a non-empty string mentioning a return address.
    assert!(callstack_vis.contains("return to instruction"));
}
