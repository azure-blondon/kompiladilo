use ir_core::{Transformation, Module, Instruction, Operand};
use language_better_brainfuck as bbf;
use language_brainfuck as bf;


pub struct BFToBBF;



impl Transformation for BFToBBF {
    
    fn name(&self) -> &str {
        "bf-to-bbf"
    }

    fn run(&mut self, module: Module) -> Module {
        let mut new_module = Module::new(bbf::BetterBrainfuckLanguage);
        for instr in module.instructions {
            self.transform_instruction(&instr, &mut new_module.instructions);
        }
        new_module
    }
}


impl BFToBBF {
    pub fn new() -> Self {
        Self
    }
    fn transform_instruction(&mut self, instr: &Instruction, instrs: &mut Vec<Instruction>) {
        match instr.opcode.as_str() {
            bf::op::PTR_RIGHT => instrs.push(bbf::move_ptr(1)),
            bf::op::PTR_LEFT => instrs.push(bbf::move_ptr(-1)),
            bf::op::INCR => instrs.push(bbf::add(1)),
            bf::op::DECR => instrs.push(bbf::add(-1)),
            bf::op::OUTPUT => instrs.push(bbf::output()),
            bf::op::INPUT => instrs.push(bbf::input()),
            bf::op::LOOP => {
                let body = instr.operands.get(0).expect("error: expected body in loop");
                let mut body_instrs: Vec<Instruction> = Vec::new();
                if let Operand::Instruction(body_instr) = body {
                    body_instr.operands
                            .iter()
                            .for_each(|o| {
                                if let Operand::Instruction(instr) = o {
                                    self.transform_instruction(&instr, &mut body_instrs);
                                }
                            });
                } else {
                    panic!("error: expected instruction operand in loop body");
                }

                instrs.push(bbf::r#loop(body_instrs));
            },
            _ => panic!("error: unknown opcode {}", instr.opcode),
        }
    }
}