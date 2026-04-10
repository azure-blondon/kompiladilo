use crate::{OperandKind};

// # VerifyError

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    UnknownOpcode(String),
    ArityMismatch {
        opcode: String,
        expected: usize,
        got: usize,
    },
    OperandTypeMismatch {
        opcode: String,
        position: usize,
        expected: OperandKind,
        got: OperandKind,
    },
    InvalidOperand {
        position: usize,
    },
}
 
impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::UnknownOpcode(op) => {
                write!(f, "unknown opcode '{op}'")
            }
            VerifyError::ArityMismatch { opcode, expected, got } => {
                write!(f, "'{opcode}' expects {expected} operand(s), got {got}")
            }
            VerifyError::OperandTypeMismatch { opcode, position, expected, got } => {
                write!(
                    f,
                    "'{opcode}' operand {position}: expected {expected:?}, got {got:?}"
                )
            }
            VerifyError::InvalidOperand { position } => {
                write!(f, "invalid operand at position {position}")
            }
        }
    }
}
 
impl std::error::Error for VerifyError {}


// # ParseError


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedToken { token: String, position: usize },
    UnexpectedEof { position: usize },
    InvalidInstruction(VerifyError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { token, position } => {
                write!(f, "unexpected token '{token}' at position {position}")
            }
            ParseError::InvalidInstruction(e) => {
                write!(f, "invalid instruction: {e}")
            }
            ParseError::UnexpectedEof { position } => {
                write!(f, "unexpected end of file at position {position}")
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::InvalidInstruction(e) => Some(e),
            _ => None,
        }
    }
}

impl From<VerifyError> for ParseError {
    fn from(e: VerifyError) -> Self {
        ParseError::InvalidInstruction(e)
    }
}