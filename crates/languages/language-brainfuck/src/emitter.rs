use ir_core::{Emitter, Instruction, Module, Operand};
use ir_core::errors::{CompileError, VerifyError};
use crate::op;

pub struct BrainfuckEmitter;

impl Emitter for BrainfuckEmitter {
    fn emit(&self, module: &Module) -> Result<String, CompileError> {
        let mut out = String::new();
        for instr in &module.instructions {
            out.push_str(&emit_instruction(instr)?);
        }
        Ok(out)
    }
}

fn emit_instruction(instruction: &Instruction) -> Result<String, VerifyError> {
    let mut out = String::new();
    match instruction.opcode.as_str() {
        op::PTR_RIGHT => out.push('>'),
        op::PTR_LEFT  => out.push('<'),
        op::INCR      => out.push('+'),
        op::DECR      => out.push('-'),
        op::OUTPUT    => out.push('.'),
        op::INPUT     => out.push(','),
        op::LOOP => {
            out.push('[');
            for operand in &instruction.operands {
                out.push_str(&emit_instruction(expect_instruction(operand)?)?);
            }
            out.push(']');
        }
        _  => {},
    }
    Ok(out)
}


fn expect_instruction<'a>(operand: &'a Operand) -> Result<&'a Instruction, VerifyError> {
    match operand {
        Operand::Instruction(instr) => Ok(instr),
        _ => Err(VerifyError::InvalidOperand { position: 0 }),
    }
}