use super::*;
use ir_core::errors::VerifyError;

pub fn emit(instructions: &[Instruction]) -> Result<String, VerifyError> {
    let mut output = String::new();
    for instr in instructions {
        output.push_str(&emit_instruction(instr)?);
        output.push('\n');
    }
    Ok(output)
}


pub fn emit_instruction(instr: &Instruction) -> Result<String, VerifyError> {
    match instr.opcode.as_str() {
        op::CONSTANT => {
            let value = match &instr.operands[0] {
                Operand::Value(v) => v.display(),
                _ => return Err(VerifyError::InvalidOperand { position: 0 }),
            };
            Ok(value)
        }
        op::VARIABLE => {
            let name = match &instr.operands[0] {
                Operand::Value(v) => v.display(),
                _ => return Err(VerifyError::InvalidOperand { position: 0 }),
            };
            Ok(name)
        }
        op::ADD | op::SUB => {
            let lhs = match &instr.operands[0] {
                Operand::Instruction(instr) if instr.opcode == op::ADD || instr.opcode == op::SUB => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::CONSTANT => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::VARIABLE => emit_instruction(instr)?,
                Operand::Value(v) => v.display(),
                _ => return Err(VerifyError::InvalidOperand { position: 0 }),
            };
            let rhs = match &instr.operands[1] {
                Operand::Instruction(instr) if instr.opcode == op::ADD || instr.opcode == op::SUB => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::CONSTANT => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::VARIABLE => emit_instruction(instr)?,
                Operand::Value(v) => v.display(),
                _ => return Err(VerifyError::InvalidOperand { position: 1 }),
            };
            Ok(format!("({} {} {})", lhs, if instr.opcode == op::ADD { "+" } else { "-" }, rhs))
        }
        op::PRINT => {
            let operand = match &instr.operands[0] {
                Operand::Instruction(instr) if instr.opcode == op::CONSTANT => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::VARIABLE => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::ADD || instr.opcode == op::SUB => emit_instruction(instr)?,
                Operand::Value(v) => v.display(),
                _ => return Err(VerifyError::InvalidOperand { position: 0 }),
            };
            Ok(format!("print({})", operand))
        }
        op::ASSIGN => {
            let var_name = match &instr.operands[0] {
                Operand::Value(v) => v.display(),
                _ => return Err(VerifyError::InvalidOperand { position: 0 }),
            };
            let expr = match &instr.operands[1] {
                Operand::Instruction(instr) if instr.opcode == op::ADD || instr.opcode == op::SUB => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::CONSTANT => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::VARIABLE => emit_instruction(instr)?,
                Operand::Value(v) => v.display(),
                _ => return Err(VerifyError::InvalidOperand { position: 1 }),
            };
            Ok(format!("{} = {}", var_name, expr))
        }
        op::LOOP => {
            let count = match &instr.operands[0] {
                Operand::Instruction(instr) if instr.opcode == op::ADD || instr.opcode == op::SUB => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::CONSTANT => emit_instruction(instr)?,
                Operand::Instruction(instr) if instr.opcode == op::VARIABLE => emit_instruction(instr)?,
                Operand::Value(v) => v.display(),
                _ => return Err(VerifyError::InvalidOperand { position: 0 }),
            };
            let body = match &instr.operands[1] {
                Operand::Instruction(instr) if instr.opcode == op::BODY => {
                    let mut body_output = String::new();
                    for stmt in &instr.operands[0..] {
                        if let Operand::Instruction(stmt_instr) = stmt {
                            body_output.push_str(&emit_instruction(stmt_instr)?);
                            body_output.push('\n');
                        } else {
                            return Err(VerifyError::InvalidOperand { position: 1 });
                        }
                    }
                    body_output
                }
                _ => return Err(VerifyError::InvalidOperand { position: 1 }),
            };
            Ok(format!("loop {} {{\n{}}}", count, body))
        }
        _ => return Err(VerifyError::UnknownOpcode(instr.opcode.clone())),
    }
}