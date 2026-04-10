use ir_core::{Parser};
use ir_core::errors::{CompileError, ParseError};
use ir_core::{Instruction, Module};

use crate::*;

pub struct BrainfuckParser;

impl Parser for BrainfuckParser {
    fn parse(&self, source: &str) -> Result<Module, CompileError> {
        parse(source)
    }
}

pub fn parse(source: &str) -> Result<Module, CompileError> {
    let mut module = Module::new(BrainfuckLanguage);
    let mut stack: Vec<(usize, Vec<Instruction>)> = vec![(0, Vec::new())];
    let mut length = 0;

    for (pos, ch) in source.chars().enumerate() {
        match ch {
            '>' => top(&mut stack).push(ptr_right()),
            '<' => top(&mut stack).push(ptr_left()),
            '+' => top(&mut stack).push(incr()),
            '-' => top(&mut stack).push(decr()),
            '.' => top(&mut stack).push(output()),
            ',' => top(&mut stack).push(input()),

            '[' => top(&mut stack).push(loop_start()),
            ']' => top(&mut stack).push(loop_end()),
            _ => {}
        }
        length = pos + 1;
    }

    if stack.len() > 1 {
        return Err(CompileError::ParseError(ParseError::UnexpectedEof { position: length }));
    }

    let (_, instructions) = stack.pop().unwrap();
    module.instructions = instructions;
    Ok(module)
}

fn top(stack: &mut Vec<(usize, Vec<Instruction>)>) -> &mut Vec<Instruction> {
    &mut stack.last_mut().unwrap().1
}
