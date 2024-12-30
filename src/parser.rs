#[derive(Debug, Clone)]
enum Component {
    Tok(Token),
    Function,
    Expression,
    Exprs,
    Args,
    Argument,
    Declare,
    Statement,
    Body,
    Num,
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tok(token1), Tok(token2)) => {
                match (token1, token2) {
                    (Name(_), Name(_)) => true,
                    (_, _) => token1 == token2,
                }
            },
            (_, _) => discriminant(self) == discriminant(other),
        }
    }
}

struct Rule {
    input: Vec<Component>,
    output: Component,
    token: Option<Token>,
}

static RULES: LazyLock<Vec<Rule>> = LazyLock::new(|| {
    Vec::from([
        Rule {
            input: Vec::from([
                Tok(Keywords(Fnc)),
                Tok(Name(None)),
                Tok(OpenParen),
                Tok(CloseParen),
                Tok(OpenCurly),
                Body,
                Tok(CloseCurly),
            ]),
            output: Function,
            token: None,
        },
        // Bodys definition
        Rule {
            input: Vec::from([Statement, Tok(End)]),
            output: Body,
            token: None,
        },
        Rule {
            input: Vec::from([Expression, Tok(End)]),
            output: Body,
            token: None,
        },
        Rule {
            input: Vec::from([Body, Body]),
            output: Body,
            token: None,
        },
        //Expressions Definition
        Rule {
            input: Vec::from([Tok(OpenParen), Expression, Tok(CloseParen)]),
            output: Expression,
            token: None,
        },
        Rule {
            input: Vec::from([Expression, Tok(Plus), Expression]),
            output: Expression,
            token: Some(Plus),
        },
        Rule {
            input: Vec::from([Expression, Tok(Minus), Expression]),
            output: Expression,
            token: Some(Minus),
        },
        Rule {
            input: Vec::from([Expression, Tok(Star), Expression]),
            output: Expression,
            token: None,
        },
        Rule {
            input: Vec::from([Tok(Name(None))]),
            output: Expression,
            token: None,
        },
        Rule {
            input: Vec::from([Num]),
            output: Expression,
            token: None,
        },
        //Statement definitions
        Rule {
            input: Vec::from([Declare]),
            output: Statement,
            token: None,
        },
    ])
});

use std::mem::discriminant;
use std::sync::LazyLock;
use crate::lexer::Keyword::Fnc;
use crate::lexer::Token;
use crate::lexer::Token::{CloseCurly, CloseParen, End, Keywords, Minus, Name, OpenCurly, OpenParen, Plus, Star};
use crate::parser::Component::{Body, Declare, Expression, Function, Num, Statement, Tok};

fn equal(stack: &Vec<Component>, pattern: &Vec<Component>) -> bool {
    let mut j = 0;
    for i in (stack.len() - pattern.len())..stack.len() {
        if stack[i] != pattern[j] {
            return false;
        }
        j += 1;
    }
    true
}

fn priotok(tok: &Token) -> u8 {
    match tok {
        Token::Dot | Token::Inc | Token::Dec => 1,
        Star | Token::Slash | Token::Percent => 2,
        Plus | Minus => 3,
        Token::LeftArrow | Token::RightArrow => 4,
        Token::OpenArrow | Token::CloseArrow => 5,
        Token::Equ | Token::Nequ => 6,
        Token::Ampersand => 7,
        Token::Caret => 8,
        Token::Vline => 9,
        Token::LogAnd => 10,
        Token::LogOr => 11,
        _ => 255,
    }
}

fn priorule(rule: &Rule) -> u8 {
    match rule.token.clone() {
        None => {0}
        Some(tok) => {priotok(&tok)}
    }
}

fn reduce(stack: &mut Vec<Component>, nexttoken: &Option<Token>) {
    for rule in RULES.iter() {
        let rulen = rule.input.len();

        if rulen > stack.len() {
            continue;
        }

        if equal(&stack, &rule.input)
            && (nexttoken.is_none()
                || matches!(nexttoken, Some(tok) if priotok(tok) >= priorule(rule)))
        {
            stack.truncate(stack.len() - rulen);
            stack.push(rule.output.clone());
            reduce(stack, nexttoken);
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Component> {
    let mut stack = Vec::new();

    for i in 0..(tokens.len() - 1) {
        stack.push(Tok(tokens[i].clone()));
        reduce(&mut stack, &Some(tokens[i + 1].clone()));
    }

    stack.push(Tok(tokens[tokens.len() - 1].clone()));
    reduce(&mut stack, &None);

    stack
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    fn test(input: String, expected: Vec<Component>) {
        let tokens = lexer::lex(input);
        let compos = parse(tokens);
        assert_eq!(compos.len(), expected.len(), "Cmpnts len !=");

        for i in 0..compos.len() {
            assert_eq!(compos[i], expected[i], "oops");
        }
    }

    #[test]
    fn simple_test() {
        let input = String::from("a + b *");
        let expected = vec![Expression, Tok(Plus), Expression, Tok(Star)];
        test(input, expected);
    }
}

