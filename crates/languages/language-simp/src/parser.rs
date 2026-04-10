use super::*;
use ir_core::errors::{CompileError, ParseError};
use ir_core::{Instruction, Module, Parser};


pub struct SimpParser;

impl Parser for SimpParser {
    fn parse(&self, source: &str) -> Result<Module, CompileError> {
        parse(source)
    }
}

pub fn parse(source: &str) -> Result<Module, CompileError> {
    let mut tokens = tokenize(source);
    let mut module = Module::new(SimpLanguage);
    module.instructions = parse_program(&mut tokens)?;
    Ok(module)
}


// # Tokenizer

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Integer(i64),
    Equals,
    Plus,
    Minus,
    LBrace,
    RBrace,
    Loop,
    Print,
}

fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\r' | '\n' => { chars.next(); }
            '=' => { chars.next(); tokens.push(Token::Equals); }
            '+' => { chars.next(); tokens.push(Token::Plus); }
            '-' => { chars.next(); tokens.push(Token::Minus); }
            '{' => { chars.next(); tokens.push(Token::LBrace); }
            '}' => { chars.next(); tokens.push(Token::RBrace); }
            '0'..='9' => {
                let mut num = String::new();
                while chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                    num.push(chars.next().unwrap());
                }
                tokens.push(Token::Integer(num.parse().unwrap()));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while chars.peek().map_or(false, |c| c.is_alphanumeric() || *c == '_') {
                    ident.push(chars.next().unwrap());
                }
                tokens.push(match ident.as_str() {
                    "loop"  => Token::Loop,
                    "print" => Token::Print,
                    _       => Token::Ident(ident),
                });
            }
            _ => { chars.next(); } // skip unknown characters
        }
    }

    tokens
}


// # Recursive descent

fn parse_program(tokens: &mut Vec<Token>) -> Result<Vec<Instruction>, ParseError> {
    // We'll consume from the front, so reverse for efficient pop()
    tokens.reverse();
    let mut statements = Vec::new();
    while !tokens.is_empty() {
        statements.push(parse_statement(tokens)?);
    }
    Ok(statements)
}

fn parse_statement(tokens: &mut Vec<Token>) -> Result<Instruction, ParseError> {
    match tokens.last() {
        Some(Token::Loop)  => parse_loop(tokens),
        Some(Token::Print) => parse_print(tokens),
        Some(Token::Ident(_)) => parse_assignment(tokens),
        Some(tok) => Err(ParseError::UnexpectedToken { token: format!("{tok:?}"), position: tokens.len() - 1 }),
        None => Err(ParseError::UnexpectedEof { position: tokens.len() }),
    }
}

fn parse_assignment(tokens: &mut Vec<Token>) -> Result<Instruction, ParseError> {
    let name = expect_ident(tokens)?;
    expect(tokens, Token::Equals)?;
    let expr = parse_expression(tokens)?;
    Ok(assign(name, expr))
}

fn parse_expression(tokens: &mut Vec<Token>) -> Result<Instruction, ParseError> {
    let mut lhs = parse_operand(tokens)?;

    while matches!(tokens.last(), Some(Token::Plus) | Some(Token::Minus)) {
        let op = tokens.pop().unwrap();
        let rhs = parse_operand(tokens)?;
        lhs = match op {
            Token::Plus  => add(lhs, rhs),
            Token::Minus => sub(lhs, rhs),
            _ => unreachable!(),
        };
    }

    Ok(lhs)
}

fn parse_operand(tokens: &mut Vec<Token>) -> Result<Instruction, ParseError> {
    match tokens.pop() {
        Some(Token::Integer(n)) => Ok(constant(n)),
        Some(Token::Ident(name)) => Ok(variable(name)),
        Some(tok) => Err(ParseError::UnexpectedToken { token: format!("{tok:?}"), position: tokens.len() }),
        None => Err(ParseError::UnexpectedEof { position: tokens.len() }),
    }
}

fn parse_loop(tokens: &mut Vec<Token>) -> Result<Instruction, ParseError> {
    expect(tokens, Token::Loop)?;
    let count = parse_operand(tokens)?;
    expect(tokens, Token::LBrace)?;
    let mut body_stmts = Vec::new();
    while !matches!(tokens.last(), Some(Token::RBrace) | None) {
        body_stmts.push(parse_statement(tokens)?);
    }
    expect(tokens, Token::RBrace)?;
    Ok(r#loop(count, body(body_stmts)))
}

fn parse_print(tokens: &mut Vec<Token>) -> Result<Instruction, ParseError> {
    expect(tokens, Token::Print)?;
    let operand = parse_operand(tokens)?;
    Ok(print(operand))
}


fn expect(tokens: &mut Vec<Token>, expected: Token) -> Result<(), ParseError> {
    match tokens.pop() {
        Some(tok) if tok == expected => Ok(()),
        Some(tok) => Err(ParseError::UnexpectedToken { token: format!("{tok:?}"), position: tokens.len() }),
        None => Err(ParseError::UnexpectedEof { position: tokens.len() }),
    }
}

fn expect_ident(tokens: &mut Vec<Token>) -> Result<String, ParseError> {
    match tokens.pop() {
        Some(Token::Ident(name)) => Ok(name),
        Some(tok) => Err(ParseError::UnexpectedToken { token: format!("{tok:?}"), position: tokens.len() }),
        None => Err(ParseError::UnexpectedEof { position: tokens.len() }),
    }
}