use ir_core::{Emitter, Instruction, Module, Operand, errors::{CompileError, VerifyError}};
use crate::{BetterBrainfuckValue, op};


pub struct BetterBrainfuckEmitter;

impl Emitter for BetterBrainfuckEmitter {
    fn emit(&self, module: &Module) -> Result<String, CompileError> {
        let mut out = String::new();
        for instr in &module.instructions {
            out.push_str(&emit_instruction(instr, 0)?);
        }
        Ok(out)
    }
}

fn emit_instruction(instruction: &Instruction, spaces: usize) -> Result<String, VerifyError> {
    let mut out = String::new();
    out.push_str(&"  ".repeat(spaces));
    match instruction.opcode.as_str() {
        op::OUTPUT     => out.push('.'),
        op::INPUT      => out.push(','),
        op::ADD        => {
            let amount = match instruction.operands.get(0) {
                Some(Operand::Value(val)) => {
                    let val = val.as_any().downcast_ref::<BetterBrainfuckValue>().ok_or_else(|| VerifyError::InvalidOperand { position: 0 })?;
                    val.0
                },
                _ => return Err(VerifyError::InvalidOperand { position: 0 }),
            };
            if amount > 0 {
                out.push('+');
                out.push_str(amount.to_string().as_str());
                out.push(' ');
            } else {
                out.push('-');
                out.push_str((-amount).to_string().as_str());
                out.push(' ');
            }
        },
        op::MOVE       => {
            let amount = match instruction.operands.get(0) {
                Some(Operand::Value(val)) => {
                    let val = val.as_any().downcast_ref::<BetterBrainfuckValue>().ok_or_else(|| VerifyError::InvalidOperand { position: 0 })?;
                    val.0
                },
                _ => return Err(VerifyError::InvalidOperand { position: 0 }),
            };
            if amount > 0 {
                out.push('>');
                out.push_str(amount.to_string().as_str());
                out.push(' ');
            } else {
                out.push('<');
                out.push_str((-amount).to_string().as_str());
                out.push(' ');
            }
        },
        op::LOOP => {
            out.push('\n');
            out.push_str(&"  ".repeat(spaces));
            out.push('[');
            out.push('\n');
            for child in &instruction.operands {
                let Operand::Instruction(child_instr) = child else {
                    return Err(VerifyError::InvalidOperand { position: 0 });
                };
                out.push_str(&emit_instruction(child_instr, spaces + 1)?);
                out.push(' ');
            }
            out.push('\n');
            out.push_str(&"  ".repeat(spaces));
            out.push(']');
            out.push('\n');
        },
        _  => {},
    }
    Ok(out)
}