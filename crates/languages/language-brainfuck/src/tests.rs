use ir_core::{Language, Module, Emitter};
use ir_core::errors::ParseError;

use crate::BrainfuckLanguage;
use crate::{ptr_left, ptr_right, incr, decr, output, loop_start, loop_end};
use crate::emitter::BrainfuckEmitter;
use crate::parser::parse;

#[test]
fn emit_simple() {
    // +++[>++<-]>.
    // cell0 = 3, loop: cell1 += 2, cell0 -= 1, then output cell1 (= 6)
    let instrs = vec![
        incr(), incr(), incr(),
        loop_start(), 
        ptr_right(), incr(), incr(), ptr_left(), decr(),
        loop_end(),
        ptr_right(), output(),
    ];
    assert_eq!(BrainfuckEmitter.emit(&Module { language: Box::from(BrainfuckLanguage), instructions: instrs }), Ok("+++[>++<-]>.".into()));
}

#[test]
fn verify_valid() {
    let lang = BrainfuckLanguage;
    let instr = ptr_right();
    assert!(lang.verify(&instr).is_ok());
}

#[test]
fn module_roundtrip() {
    let mut module = Module::new(BrainfuckLanguage);
    module.push(incr());
    module.push(output());
    let result = BrainfuckEmitter.emit(&Module { language: Box::from(BrainfuckLanguage), instructions: module.instructions });
    assert_eq!(result, Ok("+.".into()));
}

#[test]
fn roundtrip() {
    let source = "+++[>++<-]>.";
    let instructions = parse(source).unwrap().instructions;
    assert_eq!(BrainfuckEmitter.emit(&Module { language: Box::from(BrainfuckLanguage), instructions }), Ok(source.into()));
}

#[test]
fn nested_loops() {
    let source = "[[[]]]";
    let instructions = parse(source).unwrap().instructions;
    assert_eq!(BrainfuckEmitter.emit(&Module { language: Box::from(BrainfuckLanguage), instructions }), Ok(source.into()));
}

#[test]
fn comments_ignored() {
    let source = "this is a comment +++. more comment";
    let instructions = parse(source).unwrap().instructions;
    assert_eq!(BrainfuckEmitter.emit(&Module { language: Box::from(BrainfuckLanguage), instructions }), Ok("+++.".into()));
}

#[test]
fn empty() {
    assert_eq!(parse("").unwrap().instructions, vec![]);
}

#[test]
fn unexpected_closing_bracket() {
    assert_eq!(
        parse("]").err(),
        Some(ParseError::UnexpectedToken { token: "]".to_string(), position: 0 }.into())
    );
}

#[test]
fn unclosed_opening_bracket() {
    assert_eq!(
        parse("[++").err(),
        Some(ParseError::UnexpectedEof { position: 3 }.into())
    );
}

#[test]
fn unclosed_reports_first_bracket() {
    assert_eq!(
        parse("[[]").err(),
        Some(ParseError::UnexpectedEof { position: 3 }.into())
    );
}