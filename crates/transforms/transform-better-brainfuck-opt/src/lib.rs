use ir_core::{Transformation, Module, Instruction, Operand};
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

    fn run(&mut self, module: Module) -> Module {
        Module {
            language: module.language,
            instructions: self.rewrite_block(module.instructions),
        }
    }
}


impl BBFOptMerge {
    pub fn new(opcode: &str) -> Self {
        Self {
            name: format!("bbf-opt-merge-{}", opcode.to_lowercase()),
            opcode: opcode.to_string(),
        }
    }
    fn rewrite_block(&self, instrs: Vec<Instruction>) -> Vec<Instruction> {
        let mut new_instrs = Vec::new();

        let mut current_move: Option<i64> = None;

        for instr in instrs {
            let instr = self.rewrite_instr(instr);
            match instr.opcode.as_str() {
                code if code == self.opcode.as_str() => {
                    let Operand::Value(value) = &instr.operands[0] else {
                        panic!("error: unexpected operand type");
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

        new_instrs
    }

    fn rewrite_instr(&self, mut instr: Instruction) -> Instruction {
        if instr.opcode == bbf::op::LOOP {
            let inner_instrs: Vec<Instruction> = instr.operands.into_iter().map(|op| {
                match op {
                    Operand::Instruction(inner) => *inner,
                    _ => panic!("LOOP should only contain instructions"),
                }
            }).collect();

            let rewritten = self.rewrite_block(inner_instrs);

            instr.operands = rewritten
                .into_iter()
                .map(|i| Operand::Instruction(Box::new(i)))
                .collect();

            return instr;
        }

        instr.operands = instr.operands.into_iter().map(|op| {
            match op {
                Operand::Instruction(inner) => {
                    Operand::Instruction(Box::new(self.rewrite_instr(*inner)))
                }
                v => v,
            }
        }).collect();

        instr
    }

}
