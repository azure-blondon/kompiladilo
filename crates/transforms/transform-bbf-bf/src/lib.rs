use ir_core::{Instruction, Module, Operand, Transformation};
use ir_core::errors::*;
use language_better_brainfuck::{self as bbf, BetterBrainfuckValue};
use language_brainfuck::{self as bf, loop_start};


pub struct BBFToBF;




impl Transformation for BBFToBF {
    fn name(&self) -> &str {
        "bbf-to-bf"
    }

    fn run(&mut self, module: Module) -> Result<Module, CompileError> {
        let mut new_module = Module::new(bf::BrainfuckLanguage);
        for instr in module.instructions {
            self.transform_instruction(&instr, &mut new_module.instructions)?;
        }
        Ok(new_module)
    }
}

impl BBFToBF {
    pub fn new() -> Self {
        Self
    }
    fn transform_instruction(&mut self, instr: &Instruction, instrs: &mut Vec<Instruction>) -> Result<(), CompileError> {
        match instr.opcode.as_str() {
            bbf::op::MOVE => {
                let Some(Operand::Value(value)) = instr.operands.get(0) else {
                    return Err(CompileError::VerifyError(VerifyError::InvalidOperand { position: 0 }));
                };
                let value = value.as_any().downcast_ref::<BetterBrainfuckValue>().expect("error: expected integer value for move instruction").0;
                if value > 0 {
                    for _ in 0..value {
                        instrs.push(bf::ptr_right());
                    }
                } else {
                    for _ in 0..(-value) {
                        instrs.push(bf::ptr_left());
                    }
                }
            },
            bbf::op::ADD => {
                let Some(Operand::Value(value)) = instr.operands.get(0) else {
                    return Err(CompileError::VerifyError(VerifyError::InvalidOperand { position: 0 }));
                };
                let value = value.as_any().downcast_ref::<BetterBrainfuckValue>().expect("error: expected integer value for add instruction").0;
                if value > 0 {
                    for _ in 0..value {
                        instrs.push(bf::incr());
                    }
                } else {
                    for _ in 0..(-value) {
                        instrs.push(bf::decr());
                    }
                }
            },
            bbf::op::OUTPUT => instrs.push(bf::output()),
            bbf::op::INPUT => instrs.push(bf::input()),
            bbf::op::LOOP => {
                let mut body_instrs = Vec::new();
                for o in &instr.operands {
                    if let Operand::Instruction(instr) = o {
                        self.transform_instruction(&instr, &mut body_instrs)?;
                    }
                }
                instrs.push(loop_start());
                instrs.extend(body_instrs);
                instrs.push(bf::loop_end());
            },
            _ => return Err(CompileError::VerifyError(VerifyError::UnknownOpcode(instr.opcode.clone()))),
        }
        Ok(())
    }
}
