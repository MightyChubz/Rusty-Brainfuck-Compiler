#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Add,
    Sub,
    Left,
    Right,
    Read,
    Write,
    BeginLoop,
    EndLoop
}

use self::Token::*;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Add),
            '-' => tokens.push(Sub),
            '<' => tokens.push(Left),
            '>' => tokens.push(Right),
            ',' => tokens.push(Read),
            '.' => tokens.push(Write),
            '[' => tokens.push(BeginLoop),
            ']' => tokens.push(EndLoop),
            _ => ()
        }
    }

    tokens
}