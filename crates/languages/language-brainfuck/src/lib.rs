/* 
    Brainfuck

    A simple esoteric programmign language with only 8 instructions.
    It resembles a Turing machine with an infinite tape of memory cells and a data pointer.

    for more information, see https://en.wikipedia.org/wiki/Brainfuck
*/



use ir_core::{Language, Instruction, InstructionDefinition, Operand};

#[cfg(test)]
pub mod tests;

pub mod emitter;
pub mod parser;

// # Opcodes

pub mod op {
    pub const PTR_RIGHT : &str = "bf.right";
    pub const PTR_LEFT  : &str = "bf.left";
    pub const INCR      : &str = "bf.incr";
    pub const DECR      : &str = "bf.decr";
    pub const OUTPUT    : &str = "bf.output";
    pub const INPUT     : &str = "bf.input";
    pub const LOOP      : &str = "bf.loop"; // variadic: child instructions
}


// # Language

pub struct BrainfuckLanguage;

impl Language for BrainfuckLanguage {
    fn name(&self) -> &str { "brainfuck" }

    fn instruction_defs(&self) -> &[InstructionDefinition] {
        &[
            InstructionDefinition { opcode: op::PTR_RIGHT, operands: Some(&[]) },
            InstructionDefinition { opcode: op::PTR_LEFT,  operands: Some(&[]) },
            InstructionDefinition { opcode: op::INCR,      operands: Some(&[]) },
            InstructionDefinition { opcode: op::DECR,      operands: Some(&[]) },
            InstructionDefinition { opcode: op::OUTPUT,    operands: Some(&[]) },
            InstructionDefinition { opcode: op::INPUT,     operands: Some(&[]) },
            InstructionDefinition {
                opcode: op::LOOP,
                operands: None, // variable number of instructions
            },
        ]
    }
}


// # Constructors

pub fn ptr_right() -> Instruction { Instruction::leaf(op::PTR_RIGHT) }
pub fn ptr_left()  -> Instruction { Instruction::leaf(op::PTR_LEFT)  }
pub fn incr()      -> Instruction { Instruction::leaf(op::INCR)      }
pub fn decr()      -> Instruction { Instruction::leaf(op::DECR)      }
pub fn output()    -> Instruction { Instruction::leaf(op::OUTPUT)    }
pub fn input()     -> Instruction { Instruction::leaf(op::INPUT)     }


pub fn r#loop(instructions: Vec<Instruction>) -> Instruction {
    Instruction::new(op::LOOP, instructions.into_iter().map(|i| Operand::Instruction(Box::new(i))).collect())
}