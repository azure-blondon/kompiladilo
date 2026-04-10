use ir_core::{Transformation, Module, Instruction, Operand};
use ir_core::errors::{CompileError, VerifyError};
use language_better_brainfuck as bbf;

// # Merge Consecutive Moves or Adds

pub struct BBFOptMerge {
    name: String,
    opcode: String,
}



impl Transformation for BBFOptMerge {
    
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&mut self, module: Module) -> Result<Module, CompileError> {
        Ok(Module {
            language: module.language,
            instructions: self.rewrite_block(module.instructions)?,
        })
    }
}


impl BBFOptMerge {
    pub fn new(opcode: &str) -> Self {
        Self {
            name: format!("bbf-opt-merge-{}", opcode.to_lowercase()),
            opcode: opcode.to_string(),
        }
    }
    fn rewrite_block(&self, instrs: Vec<Instruction>) -> Result<Vec<Instruction>, CompileError> {
        let mut new_instrs = Vec::new();

        let mut current_move: Option<i64> = None;

        for instr in instrs {
            let instr = self.rewrite_instr(instr)?;
            match instr.opcode.as_str() {
                code if code == self.opcode.as_str() => {
                    let Operand::Value(value) = &instr.operands[0] else {
                        return Err(CompileError::VerifyError(VerifyError::InvalidOperand { position: 0 }));
                    };
                    if let Some(curent_amount) = current_move {
                        current_move = Some(curent_amount + value.as_any().downcast_ref::<bbf::BetterBrainfuckValue>().expect("error: unable to parse move amount").0);
                    } else {
                        current_move = Some(value.as_any().downcast_ref::<bbf::BetterBrainfuckValue>().expect("error: unable to parse move amount").0);
                    }  
                }
                _ => {
                    if let Some(amount) = current_move  && amount != 0 {
                        new_instrs.push(Instruction {
                            opcode: self.opcode.to_string(),
                            operands: vec![Operand::Value(Box::from(bbf::BetterBrainfuckValue(amount)))],
                        });
                    }
                    current_move = None;
                    new_instrs.push(instr);
                }
            }
        }

        if let Some(amount) = current_move && amount != 0 {
            new_instrs.push(Instruction {
                opcode: self.opcode.to_string(),
                operands: vec![Operand::Value(Box::from(bbf::BetterBrainfuckValue(amount)))],
            });
        }

        Ok(new_instrs)
    }

    fn rewrite_instr(&self, mut instr: Instruction) -> Result<Instruction, CompileError> {
        if instr.opcode == bbf::op::LOOP {
            let inner_instrs: Vec<Instruction> = instr.operands.into_iter().map(|op| {
                match op {
                    Operand::Instruction(inner) => Ok(*inner),
                    _ => return Err(CompileError::VerifyError(VerifyError::InvalidOperand { position: 0 })),
                }
            }).collect::<Result<Vec<Instruction>, CompileError>>()?;

            let rewritten = self.rewrite_block(inner_instrs)?;

            instr.operands = rewritten
                .into_iter()
                .map(|i| Operand::Instruction(Box::new(i)))
                .collect();

            return Ok(instr);
        }

        instr.operands = instr.operands.into_iter().map(|op| {
            match op {
                Operand::Instruction(inner) => {
                    Ok(Operand::Instruction(Box::new(self.rewrite_instr(*inner)?)))
                }
                v => Ok(v),
            }
        }).collect::<Result<Vec<Operand>, CompileError>>()?;

        Ok(instr)
    }

}
