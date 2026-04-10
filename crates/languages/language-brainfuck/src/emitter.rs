use crate::*;
use ir_core::errors::VerifyError;


pub fn emit(instructions: &[Instruction]) -> Result<String, VerifyError> {
    let mut out = String::new();
    for instr in instructions {
        match instr.opcode.as_str() {
            op::PTR_RIGHT => out.push('>'),
            op::PTR_LEFT  => out.push('<'),
            op::INCR      => out.push('+'),
            op::DECR      => out.push('-'),
            op::OUTPUT    => out.push('.'),
            op::INPUT     => out.push(','),
            op::LOOP => {
                let body = expect_instruction(&instr.operands[0])?;
                out.push('[');
                for operand in &body.operands {
                    out.push_str(&emit(std::slice::from_ref(expect_instruction(operand)?))?);
                }
                out.push(']');
            }
            op::BODY => return Err(VerifyError::InvalidOperand { position: 0 }),
            _  => {},
        }
    }
    Ok(out)
}

fn expect_instruction<'a>(operand: &'a Operand) -> Result<&'a Instruction, VerifyError> {
    match operand {
        Operand::Instruction(instr) => Ok(instr),
        _ => Err(VerifyError::InvalidOperand { position: 0 }),
    }
}