/*
    Simp

    A simple imperative language with basic arithmetic, variables, loops, and print statements.
    It is designed to be a minimal language for testing the IR and compiler infrastructure, and is not intended for practical use.

    For more information, see the language.md file in this directory.

*/


use ir_core::{Language, Instruction, InstructionDefinition, Operand, OperandKind, Value, ValueType};

pub mod parser;
pub mod emitter;

// # Opcodes

pub mod op {
    pub const ASSIGN : &str   = "simp.assign";  // var_name, expr
    pub const ADD    : &str   = "simp.add";     // lhs, rhs
    pub const SUB    : &str   = "simp.sub";     // lhs, rhs
    pub const LOOP   : &str   = "simp.loop";    // count, body
    pub const BODY   : &str   = "simp.body";    // variadic: child instructions
    pub const PRINT  : &str   = "simp.print";   // operand
    pub const CONSTANT : &str = "simp.const";   // integer value
    pub const VARIABLE : &str = "simp.var";     // variable name
}


// # Values

pub static INT: ValueType = ValueType("int");
pub static STR: ValueType = ValueType("str");




#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpValue {
    Int(i64),
    Str(String),
}

impl Value for SimpValue {
    fn value_type(&self) -> ValueType {
        match self {
            SimpValue::Int(_) => INT.clone(),
            SimpValue::Str(_) => STR.clone(),
        }
    }

    fn display(&self) -> String {
        match self {
            SimpValue::Int(i) => i.to_string(),
            SimpValue::Str(s) => s.clone(),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// # Language

pub struct SimpLanguage;

impl Language for SimpLanguage {
    
    fn name(&self) -> &str { "simp" }

    fn instruction_defs(&self) -> &[InstructionDefinition] {
        &[
            InstructionDefinition {
                opcode: op::CONSTANT,
                operands: Some(&[OperandKind::Value(ValueType("int"))])
            },
            InstructionDefinition {
                opcode: op::VARIABLE,
                operands: Some(&[OperandKind::Value(ValueType("str"))])
            },
            InstructionDefinition {
                opcode: op::ADD,
                operands: Some(&[OperandKind::Instruction, OperandKind::Instruction]),
            },
            InstructionDefinition {
                opcode: op::SUB,
                operands: Some(&[OperandKind::Instruction, OperandKind::Instruction]),
            },
            InstructionDefinition {
                opcode: op::ASSIGN,
                operands: Some(&[OperandKind::Value(ValueType("str")), OperandKind::Instruction]),
            },
            InstructionDefinition {
                opcode: op::PRINT,
                operands: Some(&[OperandKind::Instruction]),
            },
            InstructionDefinition {
                opcode: op::LOOP,
                operands: Some(&[OperandKind::Instruction, OperandKind::Instruction]),
            },
            InstructionDefinition {
                opcode: op::BODY,
                operands: None, // variadic
            },
        ]
    }
}


// # Constructors


pub fn constant(value: i64) -> Instruction {
    Instruction::new(op::CONSTANT, vec![Operand::Value(Box::new(SimpValue::Int(value)))])
}

pub fn variable(name: impl Into<String>) -> Instruction {
    Instruction::new(op::VARIABLE, vec![Operand::Value(Box::new(SimpValue::Str(name.into())))])
}

pub fn add(lhs: Instruction, rhs: Instruction) -> Instruction {
    Instruction::new(op::ADD, vec![
        Operand::Instruction(Box::new(lhs)),
        Operand::Instruction(Box::new(rhs)),
    ])
}

pub fn sub(lhs: Instruction, rhs: Instruction) -> Instruction {
    Instruction::new(op::SUB, vec![
        Operand::Instruction(Box::new(lhs)),
        Operand::Instruction(Box::new(rhs)),
    ])
}

pub fn assign(name: impl Into<String>, expr: Instruction) -> Instruction {
    Instruction::new(op::ASSIGN, vec![
        Operand::Value(Box::new(SimpValue::Str(name.into()))),
        Operand::Instruction(Box::new(expr)),
    ])
}

pub fn print(operand: Instruction) -> Instruction {
    Instruction::new(op::PRINT, vec![Operand::Instruction(Box::new(operand))])
}

pub fn body(statements: Vec<Instruction>) -> Instruction {
    Instruction::new(
        op::BODY,
        statements.into_iter().map(|i| Operand::Instruction(Box::new(i))).collect(),
    )
}

pub fn r#loop(count: Instruction, body: Instruction) -> Instruction {
    Instruction::new(op::LOOP, vec![
        Operand::Instruction(Box::new(count)),
        Operand::Instruction(Box::new(body)),
    ])
}
