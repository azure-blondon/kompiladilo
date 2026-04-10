use ir_core::{Instruction, Module, Transformation};
use language_brainfuck as bf;
use language_simp as simp;


/*
Implementation details:

the tape will contain two stacks, one for variables and one for temporary storage during computations.
The variable stack starts at cell 'center'-1 and grows downwards, while the temporary storage stack starts at cell 'center' and grows upwards.

*/

pub struct SimpToBrainfuck {
    center: i32, // the center of the coordinate system
    temp_stack_ptr: i32, // points to the next free cell for temporary storage (starts at center and grows upwards)
    variables: Vec<String>, // variable name -> cell index (starts at center-1 and goes downwards)
}

impl Transformation for SimpToBrainfuck {

    fn name(&self) -> &'static str {
        "simp-brainfuck"
    }

    fn run(&mut self, module: Module) -> Module {
        let mut new_module = Module::new(bf::BrainfuckLanguage);
        let num_vars = self.find_number_of_different_variables(&module);
        self.set_center(num_vars);
        new_module.instructions.extend(self.move_x_units(self.center));
        for instr in module.instructions {
            let new_instrs = self.simp_to_brainfuck(&instr, 0);
            for new_instr in new_instrs {
                new_module.instructions.push(new_instr);
            }
        }
        new_module
    }
}

impl SimpToBrainfuck {
    pub fn new() -> Self {
        return Self {
            variables: Vec::new(),
            center: 0,
            temp_stack_ptr: 0,
        };

    }

    fn set_center(&mut self, center: i32) {
        self.center = center;
        self.temp_stack_ptr = center;
    }

    fn push_temp(&mut self) -> i32 {
        let temp_cell = self.temp_stack_ptr;
        self.temp_stack_ptr += 1;
        temp_cell
    }

    fn push_or_get_var(&mut self, var_name: String) -> i32 {
        if let Some(index) = self.variables.iter().position(|v| v == &var_name) {
            self.center - 1 - (index as i32)
        } else {
            self.variables.push(var_name);
            let var_cell = self.center - 1 - (self.variables.len() as i32 - 1);
            var_cell
        }
        
    }

    fn get_var_cell(&self, var_name: &str) -> i32 {
        if let Some(index) = self.variables.iter().position(|v| v == var_name) {
            self.center - 1 - (index as i32)
        } else {
            panic!("Undefined variable: {var_name}");
        }
    }

    fn pop_temp(&mut self) {
        if self.temp_stack_ptr == self.center {
            panic!("Temp stack underflow");
        }
        self.temp_stack_ptr -= 1;
    }


    
    fn simp_to_brainfuck(&mut self, instr: &Instruction, target_cell: i32) -> Vec<Instruction> {
        let mut new_instrs = Vec::new();
        let opcode = instr.opcode.as_str();
        match opcode {
            simp::op::ASSIGN => {
                let var_name = match &instr.operands[0] {
                    ir_core::Operand::Value(v) => v.display(),
                    _ => panic!("Invalid operand for ASSIGN"),
                };
                let temp_cell = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell));

                let temp_cell2 = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell2));

                let var_index: i32 = self.push_or_get_var(var_name);
                let value_instr = match &instr.operands[1] {
                    ir_core::Operand::Instruction(i) => i,
                    _ => panic!("Invalid operand for ASSIGN"),
                };
                // compute the value and store it in the variable's cell
                new_instrs.extend(self.simp_to_brainfuck(value_instr, temp_cell));
                new_instrs.extend(self.clear_cell(var_index));
                new_instrs.extend(self.copy_cell(temp_cell, temp_cell2, var_index));
                self.pop_temp();
                self.pop_temp();
            }
            simp::op::CONSTANT => {
                let value = match &instr.operands[0] {
                    ir_core::Operand::Value(v) => v,
                    _ => panic!("Invalid operand for CONSTANT"),
                };
                let simp_value = match value.value_type().0 {
                    "int" => {
                        let inner_value = value.as_any().downcast_ref::<simp::SimpValue>().expect("error: unable to parse int value");
                        match inner_value {
                            simp::SimpValue::Int(i) => simp::SimpValue::Int(*i),
                            _ => panic!("Expected int value"),
                        }
                    }
                    _ => panic!("Unsupported value type for CONSTANT"),
                };
                let temp_cell = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell)); // clear temp cell before use
                
                new_instrs.extend(self.compute_value(&simp_value, temp_cell, target_cell));
                self.pop_temp();
            }
            simp::op::VARIABLE => {
                let var_name = match &instr.operands[0] {
                    ir_core::Operand::Value(v) => v.display(),
                    _ => panic!("Invalid operand for VARIABLE"),
                };
                if !self.variables.contains(&var_name.to_string()) {
                    panic!("Undefined variable: {var_name}");
                }
                let temp_cell = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell)); // clear temp cell before use
                let var_index: i32 = self.get_var_cell(&var_name);
                new_instrs.extend(self.copy_cell(var_index, temp_cell, target_cell));
                self.pop_temp();
            }
            simp::op::ADD | simp::op::SUB => {
                // push the first operand to the stack
                let first_instr = match &instr.operands[0] {
                    ir_core::Operand::Instruction(i) => i,
                    _ => panic!("Invalid operand for ADD/SUB"),
                };
                let second_instr = match &instr.operands[1] {
                    ir_core::Operand::Instruction(i) => i,
                    _ => panic!("Invalid operand for ADD/SUB"),
                };
                let temp_cell1 = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell1)); // clear temp cell before use

                new_instrs.extend(self.simp_to_brainfuck(first_instr, temp_cell1));

                let temp_cell2 = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell2)); // clear temp cell before use
                
                new_instrs.extend(self.simp_to_brainfuck(second_instr, temp_cell2));

                let temp_cell3 = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell3)); // clear temp cell before use

                if opcode == simp::op::ADD {
                    new_instrs.extend(self.add_two_cells(temp_cell1, temp_cell2, temp_cell3, target_cell));
                } else {
                    new_instrs.extend(self.substract_first_from_second(temp_cell2, temp_cell1, temp_cell3, target_cell));
                }
                self.pop_temp();
                self.pop_temp();
                self.pop_temp();
            }
            simp::op::PRINT => {
                // generate brainfuck code to compute the value and print it
                let value_instr = match &instr.operands[0] {
                    ir_core::Operand::Instruction(i) => i,
                    _ => panic!("Invalid operand for PRINT"),
                };
                let temp_cell = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell)); // clear temp cell before use

                new_instrs.extend(self.simp_to_brainfuck(value_instr, temp_cell));
                
                new_instrs.extend(self.move_from_to(self.center, temp_cell));
                new_instrs.push(bf::output());
                new_instrs.extend(self.move_from_to(temp_cell, self.center));

                self.pop_temp();
            }
            simp::op::LOOP => {
                let value_instr = match &instr.operands[0] {
                    ir_core::Operand::Instruction(i) => i,
                    _ => panic!("Invalid operand for LOOP"),
                };
                let body_instr = match &instr.operands[1] {
                    ir_core::Operand::Instruction(i) => i,
                    _ => panic!("Invalid operand for LOOP"),
                };

                
                
                let temp_cell = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell));

                let temp_cell2 = self.push_temp();
                new_instrs.extend(self.clear_cell(temp_cell2));


                new_instrs.extend(self.simp_to_brainfuck(value_instr, temp_cell));

                let mut loop_body = Vec::new();
                loop_body.extend(self.simp_to_brainfuck(body_instr, temp_cell2));
                
                loop_body.extend(self.move_from_to(self.center, temp_cell));
                loop_body.push(bf::decr());
                loop_body.extend(self.move_from_to(temp_cell, self.center));

                new_instrs.push(bf::r#loop(loop_body));

                self.pop_temp();
                self.pop_temp();
            }
            simp::op::BODY => {
                for operand in &instr.operands {
                    let body_instr = match operand {
                        ir_core::Operand::Instruction(i) => i,
                        _ => panic!("Invalid operand for BODY"),
                    };
                    new_instrs.extend(self.simp_to_brainfuck(body_instr, target_cell));
                }
            }
            _ => panic!("Unsupported opcode: {opcode}"),
        }

        new_instrs
    }

    fn move_x_units(&self, x: i32) -> Vec<Instruction> {
        let mut instrs = Vec::new();
        if x > 0 {
            for _ in 0..x {
                instrs.push(bf::ptr_right());
            }
        } else {
            for _ in 0..(-x) {
                instrs.push(bf::ptr_left());
            }
        }
        instrs
    }

    fn move_from_to(&self, from: i32, to: i32) -> Vec<Instruction> {
        let mut instrs = Vec::new();
        instrs.extend(self.move_x_units(to - from));
        instrs
    }

    fn copy_cell(&self, from: i32, temp: i32, to: i32) -> Vec<Instruction> {
        let mut instrs = Vec::new();

        // copy value from 'from' to 'temp' and 'to'
        instrs.extend(self.move_from_to(self.center, from));
        let mut body = vec![];
        body.extend(self.move_from_to(from, to));
        body.push(bf::incr()); // increment to
        body.extend(self.move_from_to(to, temp));
        body.push(bf::incr()); // increment temp
        body.extend(self.move_from_to(temp, from)); // move back to from
        body.push(bf::decr()); // decrement from

        instrs.push(bf::r#loop( body));
        instrs.extend(self.move_from_to(from, temp));

        let mut body = vec![];
        body.extend(self.move_from_to(temp, from));
        body.push(bf::incr()); // increment from
        body.extend(self.move_from_to(from, temp));
        body.push(bf::decr()); // decrement temp
        
        instrs.push(bf::r#loop(body));
        
        instrs.extend(self.move_from_to(temp, self.center));
        

        instrs
    }

    fn compute_value(&self, value: &simp::SimpValue, temp_cell: i32, target_cell: i32) -> Vec<Instruction> {
        let mut instrs = Vec::new();
        match value {
            simp::SimpValue::Int(i) => {
                instrs.extend(self.move_from_to(self.center, target_cell));
                for _ in 0..*i {
                    instrs.push(bf::incr());
                }
                instrs.extend(self.move_from_to(target_cell, self.center));
            }
            simp::SimpValue::Str(var_name) => {
                let var_index: i32 = self.variables.iter().position(|v| v == var_name).unwrap() as i32 + 16; // offset by 16
                instrs.extend(self.clear_cell(target_cell));
                instrs.extend(self.copy_cell(var_index, temp_cell, target_cell));
            }
        }
        instrs
    }

    fn add_two_cells(&self, first_cell: i32, second_cell: i32, temp_cell: i32, target_cell: i32) -> Vec<Instruction> {
        let mut instrs = Vec::new();
        
        instrs.extend(self.copy_cell(first_cell, temp_cell, target_cell));
        instrs.extend(self.copy_cell(second_cell, temp_cell, target_cell));
        instrs
    }

    fn substract_first_from_second(&self, first_cell: i32, second_cell: i32, temp_cell: i32, target_cell: i32) -> Vec<Instruction> {
        let mut instrs = Vec::new();

        instrs.extend(self.clear_cell(target_cell));
        instrs.extend(self.copy_cell(second_cell, temp_cell, target_cell));

        instrs.extend(self.move_from_to(self.center, first_cell));
        let mut body = vec![];
        body.extend(self.move_from_to(first_cell, target_cell));
        body.push(bf::decr()); // decrement temp (which holds the result)
        body.extend(self.move_from_to(target_cell, first_cell)); // move back to first_cell
        body.push(bf::decr()); // decrement first_cell
        instrs.push(bf::r#loop(body));

        instrs.extend(self.move_from_to(first_cell, self.center));
        instrs
    }


    fn clear_cell(&self, cell: i32) -> Vec<Instruction> {
        let mut instrs = Vec::new();
        instrs.extend(self.move_from_to(self.center, cell));
        let body = vec![
            bf::decr(),
        ];
        instrs.push(bf::r#loop(body));
        instrs.extend(self.move_from_to(cell, self.center));
        instrs
    }


    fn find_number_of_different_variables(&self, module: &Module) -> i32 {
        let mut variables = Vec::new();
        for instr in &module.instructions {
            if instr.opcode == simp::op::ASSIGN {
                let var_name = match &instr.operands[0] {
                    ir_core::Operand::Value(v) => v.display(),
                    _ => panic!("Invalid operand for ASSIGN"),
                };
                if !variables.contains(&var_name) {
                    variables.push(var_name);
                }
            }
        }
        variables.len() as i32
    }

}