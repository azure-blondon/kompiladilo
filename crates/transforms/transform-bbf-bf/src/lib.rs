use ir_core::{Instruction, Module, Operand, Transformation};
use language_better_brainfuck::{self as bbf, BetterBrainfuckValue};
use language_brainfuck as bf;


pub struct BBFToBF;




impl Transformation for BBFToBF {
    fn name(&self) -> &str {
        "bbf-to-bf"
    }

    fn run(&mut self, module: Module) -> Module {
        let mut new_module = Module::new(bf::BrainfuckLanguage);
        for instr in module.instructions {
            self.transform_instruction(&instr, &mut new_module.instructions);
        }
        new_module
    }
}

impl BBFToBF {
    pub fn new() -> Self {
        Self
    }
    fn transform_instruction(&mut self, instr: &Instruction, instrs: &mut Vec<Instruction>) {
        match instr.opcode.as_str() {
            bbf::op::MOVE => {
                let Some(Operand::Value(value)) = instr.operands.get(0) else {
                    panic!("error: expected value operand for move instruction");
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
                    panic!("error: expected value operand for add instruction");
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
                instr.operands
                        .iter()
                        .for_each(|o| {
                            if let Operand::Instruction(instr) = o {
                                self.transform_instruction(&instr, &mut body_instrs);
                            }
                        });
                
                instrs.push(bf::r#loop(body_instrs));
            },
            _ => panic!("error: unknown opcode {}", instr.opcode),
        }
    }
}
