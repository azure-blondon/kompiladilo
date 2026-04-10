use ir_core::{Language, Module};
use ir_core::errors::ParseError;

use crate::*;
use emitter::emit;
use parser::parse;

#[test]
fn emit_simple() {
    // +++[>++<-]>.
    // cell0 = 3, loop: cell1 += 2, cell0 -= 1, then output cell1 (= 6)
    let loop_body = vec![
        ptr_right(), incr(), incr(),
        ptr_left(), decr(),
    ];
    let instrs = vec![
        incr(), incr(), incr(),
        r#loop(loop_body),
        ptr_right(), output(),
    ];
    assert_eq!(emit(&instrs), Ok("+++[>++<-]>.".into()));
}

#[test]
fn verify_valid() {
    let lang = BrainfuckLanguage;
    let instr = r#loop(vec![incr(), decr()]);
    assert!(lang.verify(&instr).is_ok());
}

#[test]
fn verify_loop_wrong_operand() {
    use ir_core::{ValueType, errors::VerifyError};

    struct DummyVal;
    impl std::fmt::Debug for DummyVal {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "DummyVal")
        }
    }
    impl ir_core::Value for DummyVal {
        fn value_type(&self) -> ValueType { ValueType("dummy") }
        fn display(&self) -> String { "dummy".into() }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    let lang = BrainfuckLanguage;
    let bad = Instruction::new(op::LOOP, vec![Operand::Value(Box::new(DummyVal))]);
    assert!(matches!(
        lang.verify(&bad),
        Err(VerifyError::OperandTypeMismatch { position: 0, .. })
    ));
}

#[test]
fn module_roundtrip() {
    let mut module = Module::new(BrainfuckLanguage);
    module.push(incr());
    module.push(output());
    let result = emit(&module.instructions);
    assert_eq!(result, Ok("+.".into()));
}

#[test]
fn roundtrip() {
    let source = "+++[>++<-]>.";
    let instructions = parse(source).unwrap().instructions;
    assert_eq!(emit(&instructions), Ok(source.into()));
}

#[test]
fn nested_loops() {
    let source = "[[[]]]";
    let instructions = parse(source).unwrap().instructions;
    assert_eq!(emit(&instructions), Ok(source.into()));
}

#[test]
fn comments_ignored() {
    let source = "this is a comment +++. more comment";
    let instructions = parse(source).unwrap().instructions;
    assert_eq!(emit(&instructions), Ok("+++.".into()));
}

#[test]
fn empty() {
    assert_eq!(parse("").unwrap().instructions, vec![]);
}

#[test]
fn unexpected_closing_bracket() {
    assert_eq!(
        parse("]").err(),
        Some(ParseError::UnexpectedToken { token: "]".to_string(), position: 0 })
    );
}

#[test]
fn unclosed_opening_bracket() {
    assert_eq!(
        parse("[++").err(),
        Some(ParseError::UnexpectedEof { position: 3 })
    );
}

#[test]
fn unclosed_reports_first_bracket() {
    assert_eq!(
        parse("[[]").err(),
        Some(ParseError::UnexpectedEof { position: 3 })
    );
}