/*
    Better Brainfuck

    A more structured version of Brainfuck that is designed to be easier to optimize.


*/

use ir_core::{Instruction, InstructionDefinition, Language, Operand, OperandKind, Value, ValueType};

pub mod emitter;

// # Opcodes


pub mod op {
    pub const MOVE      : &str = "bbf.move";
    pub const ADD       : &str = "bbf.add";
    pub const OUTPUT    : &str = "bbf.output";
    pub const INPUT     : &str = "bbf.input";
    pub const LOOP      : &str = "bbf.loop"; // variadic: child instructions
}

// # Values

pub static INT: ValueType = ValueType("int");


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BetterBrainfuckValue(pub i64);

impl Value for BetterBrainfuckValue {
    fn display(&self) -> String {
        self.0.to_string()
    }
    fn value_type(&self) -> ValueType {
        INT.clone()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// # Language

pub struct BetterBrainfuckLanguage;

impl Language for BetterBrainfuckLanguage {
    fn name(&self) -> &str { "better-brainfuck" }

    fn instruction_defs(&self) -> &[InstructionDefinition] {
        &[
            InstructionDefinition {
                opcode: op::MOVE,
                operands: Some(&[OperandKind::Value(ValueType("int"))]),
            },
            InstructionDefinition {
                opcode: op::ADD,
                operands: Some(&[OperandKind::Value(ValueType("int"))]),
            },
            InstructionDefinition {
                opcode: op::OUTPUT,
                operands: Some(&[]),
            },
            InstructionDefinition {
                opcode: op::INPUT,
                operands: Some(&[]),
            },
            InstructionDefinition {
                opcode: op::LOOP,
                operands: None, // variable number of instructions
            },
        ]
    }
}

// # Constructors

pub fn move_ptr(offset: i64) -> Instruction {
    Instruction::new(op::MOVE, vec![Operand::Value(Box::new(BetterBrainfuckValue(offset)))])
}

pub fn add(value: i64) -> Instruction {
    Instruction::new(op::ADD, vec![Operand::Value(Box::new(BetterBrainfuckValue(value)))])
}

pub fn output() -> Instruction {
    Instruction::new(op::OUTPUT, vec![])
}   

pub fn input() -> Instruction {
    Instruction::new(op::INPUT, vec![])
}

pub fn r#loop(instructions: Vec<Instruction>) -> Instruction {
    Instruction::new(op::LOOP, instructions.into_iter().map(|i| Operand::Instruction(Box::new(i))).collect())
}

