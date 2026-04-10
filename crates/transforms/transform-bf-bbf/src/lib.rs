use ir_core::{Instruction, Module, Transformation, errors::{CompileError, ParseError}};
use language_better_brainfuck as bbf;
use language_brainfuck as bf;


pub struct BFToBBF;



impl Transformation for BFToBBF {
    
    fn name(&self) -> &str {
        "bf-to-bbf"
    }

    fn run(&mut self, module: Module) -> Result<Module, CompileError> {
        let mut new_module = Module::new(bbf::BetterBrainfuckLanguage);
        
        let mut stack: Vec<(usize, Vec<Instruction>)> = vec![(0, Vec::new())];
        let mut length = 0;

        for (pos, instr) in module.instructions.iter().enumerate() {
            
            match instr.opcode.as_str() {
                bf::op::PTR_RIGHT => top(&mut stack).push(bbf::move_ptr(1)),
                bf::op::PTR_LEFT => top(&mut stack).push(bbf::move_ptr(-1)),
                bf::op::INCR => top(&mut stack).push(bbf::add(1)),
                bf::op::DECR => top(&mut stack).push(bbf::add(-1)),
                bf::op::OUTPUT => top(&mut stack).push(bbf::output()),
                bf::op::INPUT => top(&mut stack).push(bbf::input()),
                bf::op::LOOP_START => stack.push((pos, Vec::new())),
                bf::op::LOOP_END => {
                    let (_, body_instrs) = stack.pop().filter(|_| !stack.is_empty()).expect("error: unmatched loop end at position {pos}");

                    let loop_instr = bbf::r#loop(body_instrs);
                    top(&mut stack).push(loop_instr);

                }
                _ => panic!("error: unknown opcode {}", instr.opcode),
            }
            length = pos + 1;
        }
        if stack.len() > 1 {
            return Err(CompileError::ParseError(ParseError::UnexpectedEof { position: length }));
        }
        let (_, instrs) = stack.pop().unwrap();
        
        new_module.instructions = instrs;
        Ok(new_module)
    }
}


impl BFToBBF {
    pub fn new() -> Self {
        Self
    }
}

fn top(stack: &mut Vec<(usize, Vec<Instruction>)>) -> &mut Vec<Instruction> {
    &mut stack.last_mut().unwrap().1
}