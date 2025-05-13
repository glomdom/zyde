use zyde::instruction::Instruction;
use zyde::vm::{VM, VmError};

#[test]
fn test_loadimm() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 42.0,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], 42.0);
}

#[test]
fn test_add() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 10.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 20.0,
        },
        Instruction::Add {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[2], 30.0);
}

#[test]
fn test_sub() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 50.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 8.0,
        },
        Instruction::Sub {
            dest: 2,
            src1: 0,
            src2: 1,
        }, // 50 - 8 = 42.0
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[2], 42.0);
}

#[test]
fn test_mul() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 6.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 7.0,
        },
        Instruction::Mul {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[2], 42.0);
}

#[test]
fn test_div() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 84.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 2.0,
        },
        Instruction::Div {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[2], 42.0);
}

#[test]
fn test_jump() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 1.0,
        },
        Instruction::Jump(3),
        Instruction::LoadImm {
            dest: 0,
            value: 999.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 42.0,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], 1.0);
    assert_eq!(vm.registers[1], 42.0);
}

#[test]
fn test_conditional_jump_taken() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 0.0,
        },
        Instruction::ConditionalJump { cond: 0, target: 3 },
        Instruction::LoadImm {
            dest: 1,
            value: 999.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 42.0,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[1], 42.0);
}

#[test]
fn test_conditional_jump_not_taken() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 1.0,
        },
        Instruction::ConditionalJump { cond: 0, target: 4 },
        Instruction::LoadImm {
            dest: 1,
            value: 999.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 42.0,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[1], 42.0);
}

#[test]
fn test_call_and_return() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 10.0,
        },
        Instruction::Call { addr: 4 },
        Instruction::LoadImm {
            dest: 1,
            value: 42.0,
        },
        Instruction::Halt,
        Instruction::LoadImm {
            dest: 2,
            value: 100.0,
        },
        Instruction::Return,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], 10.0);
    assert_eq!(vm.registers[1], 42.0);
    assert_eq!(vm.registers[2], 100.0);
    assert_eq!(vm.call_stack.len(), 0);
}

#[test]
fn test_store_and_load() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 123.0,
        },
        Instruction::Store {
            src: 0,
            var: "x".to_string(),
        },
        Instruction::LoadImm {
            dest: 0,
            value: 0.0,
        },
        Instruction::Load {
            dest: 1,
            var: "x".to_string(),
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[1], 123.0);
    assert_eq!(vm.variables.get("x"), Some(&123.0));
}

#[test]
fn test_equal() {
    let program_true = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 5.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 5.0,
        },
        Instruction::Equal {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];

    let mut vm_true = VM::new(program_true, 4);
    vm_true.run().unwrap();

    assert_eq!(vm_true.registers[2], 1.0);

    let program_false = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 5.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 7.0,
        },
        Instruction::Equal {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];

    let mut vm_false = VM::new(program_false, 4);
    vm_false.run().unwrap();

    assert_eq!(vm_false.registers[2], 0.0);
}

#[test]
fn test_less_than() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 3.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 5.0,
        },
        Instruction::LessThan {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[2], 1.0);

    let program_false = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 5.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 3.0,
        },
        Instruction::LessThan {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];

    let mut vm_false = VM::new(program_false, 4);
    vm_false.run().unwrap();

    assert_eq!(vm_false.registers[2], 0.0);
}

#[test]
fn test_greater_than() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 7.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 5.0,
        },
        Instruction::GreaterThan {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[2], 1.0);

    let program_false = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 3.0,
        },
        Instruction::LoadImm {
            dest: 1,
            value: 5.0,
        },
        Instruction::GreaterThan {
            dest: 2,
            src1: 0,
            src2: 1,
        },
        Instruction::Halt,
    ];

    let mut vm_false = VM::new(program_false, 4);
    vm_false.run().unwrap();

    assert_eq!(vm_false.registers[2], 0.0);
}

#[test]
fn test_not() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 0.0,
        },
        Instruction::Not { dest: 1, src: 0 },
        Instruction::LoadImm {
            dest: 2,
            value: 1.0,
        },
        Instruction::Not { dest: 3, src: 2 },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[1], 1.0);
    assert_eq!(vm.registers[3], 0.0);
}

#[test]
fn test_halt() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 10.0,
        },
        Instruction::Halt,
        Instruction::LoadImm {
            dest: 0,
            value: 999.0,
        },
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], 10.0);
}

#[test]
fn test_invalid_register() {
    let program = vec![
        Instruction::LoadImm {
            dest: 10,
            value: 42.0,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    let result = vm.run();

    assert!(matches!(result, Err(VmError::RegisterOutOfBounds(_))));
}

#[test]
fn test_jump_out_of_bounds() {
    let program = vec![Instruction::Jump(100), Instruction::Halt];
    let mut vm = VM::new(program, 4);
    let result = vm.run();

    assert!(matches!(result, Err(VmError::ProgramCounterOutOfBounds)));
}

#[test]
fn test_return_without_call() {
    let program = vec![Instruction::Return, Instruction::Halt];
    let mut vm = VM::new(program, 4);
    let result = vm.run();

    assert!(matches!(result, Err(VmError::CallStackEmpty)));
}

#[test]
fn test_visualize_callstack() {
    let program = vec![
        Instruction::Call { addr: 2 },
        Instruction::Halt,
        Instruction::LoadImm {
            dest: 0,
            value: 42.0,
        },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    let callstack_vis = vm.visualize_callstack();
    assert!(callstack_vis.contains("return address"));
}

#[test]
fn test_mov() {
    let program = vec![
        Instruction::LoadImm {
            dest: 0,
            value: 123.0,
        },
        Instruction::Mov { dest: 1, src: 0 },
        Instruction::Halt,
    ];

    let mut vm = VM::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[1], 123.0);
}
