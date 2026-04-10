use std::fmt::Display;

use crate::errors::VerifyError;

// # Define modules
pub mod errors;


// # Value

pub trait Value: std::fmt::Debug {
    fn value_type(&self) -> ValueType;
    fn display(&self) -> String;
    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueType(pub &'static str);


// # Operand

#[derive(Debug)]
pub enum Operand {
    Value(Box<dyn Value>),
    Instruction(Box<Instruction>),
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Operand::Instruction(a), Operand::Instruction(b)) => a == b,
            (Operand::Value(a), Operand::Value(b)) => a.value_type() == b.value_type(),
            _ => false,
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Value(v) => write!(f, "{}", v.display()),
            Operand::Instruction(instr) => {
                let operands = instr.operands.iter().map(|o| format!("{}", o)).collect::<Vec<_>>().join(", ");
                write!(f, "({}: {})", instr.opcode, operands)
            }
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperandKind {
    Value(ValueType),
    Instruction,
}


// # Instruction

pub struct InstructionDefinition {
    pub opcode: &'static str,
    pub operands: Option<&'static [OperandKind]>,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<Operand>,
}

impl Instruction {
    pub fn new(opcode: impl Into<String>, operands: Vec<Operand>) -> Self {
        Self {
            opcode: opcode.into(),
            operands,
        }
    }
 
    pub fn leaf(opcode: impl Into<String>) -> Self {
        Self::new(opcode, vec![])
    }
}




// # Language

pub trait Language {
    fn name(&self) -> &str;
    fn instruction_defs(&self) -> &[InstructionDefinition];

    fn get_def(&self, opcode: &str) -> Option<&InstructionDefinition> {
        self.instruction_defs().iter().find(|d| d.opcode == opcode)
    }

    fn verify(&self, instr: &Instruction) -> Result<(), VerifyError> {
        let def = self
            .get_def(&instr.opcode)
            .ok_or_else(|| VerifyError::UnknownOpcode(instr.opcode.clone()))?;
        
        let Some(expected_operands) = def.operands else {
            return Ok(());
        };

        if instr.operands.len() != expected_operands.len() {
            return Err(VerifyError::ArityMismatch {
                opcode: instr.opcode.clone(),
                expected: expected_operands.len(),
                got: instr.operands.len(),
            });
        }
 
        for (i, (operand, expected_kind)) in
            instr.operands.iter().zip(expected_operands.iter()).enumerate()
        {
            let got_kind = match operand {
                Operand::Instruction(_) => OperandKind::Instruction,
                Operand::Value(v) => OperandKind::Value(v.value_type()),
            };
            if got_kind != *expected_kind {
                return Err(VerifyError::OperandTypeMismatch {
                    opcode: instr.opcode.clone(),
                    position: i,
                    expected: expected_kind.clone(),
                    got: got_kind,
                });
            }
        }
 
        Ok(())
    }
}


// # Module

pub struct Module {
    pub language: Box<dyn Language>,
    pub instructions: Vec<Instruction>,
}

impl Module {
    pub fn new(language: impl Language + 'static) -> Self {
        Self {
            language: Box::new(language),
            instructions: Vec::new(),
        }
    }
 
    pub fn push(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n", self.language.name())?;
        for instr in &self.instructions {
            let operands = instr.operands.iter().map(|o| format!("{}", o)).collect::<Vec<_>>().join(", ");
            write!(f, "  {} {}\n", instr.opcode, operands)?;
        }
        Ok(())
    }
}


// # Transformation

pub trait Transformation {
    fn name(&self) -> &str;
    fn run(&mut self, module: Module) -> Module;
}

// # Pipeline

pub struct Pipeline {
    transformations: Vec<Box<dyn Transformation>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            transformations: Vec::new(),
        }
    }
    pub fn run(&mut self, module: Module) -> Module {
        self.transformations.iter_mut().fold(module, |m, t| t.run(m))
    }

    pub fn add_transformation(&mut self, transformation: impl Transformation + 'static) {
        self.transformations.push(Box::new(transformation));
    }
}