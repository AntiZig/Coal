enum Component {
    Tok(Token),
    Name,
    Function,
    Expression,
    Exprs,
    Args,
    Argument,
    InfixFunc,
    Declare,
    Statement,
    Body,
    Num,
}

struct Rule {
    input: Vec<Component>,
    output: Component,
}

const RULES: Vec<Rule> = {
    vec![
        Rule {
            input: vec![
                Tok(Keywords(Fnc)),
                Name,
                Tok(OpenParen),
                Tok(CloseParen),
                Tok(OpenCurly),
                Body,
                Tok(CloseCurly),
            ],
            output: Function,
        },
        // Bodys definition
        Rule {
            input: vec![Statement, Tok(End)],
            output: Body,
        },
        Rule {
            input: vec![Expression, Tok(End)],
            output: Body,
        },
        Rule {
            input: vec![Body, Body],
            output: Body,
        },
        //Expressions Definition
        Rule {
            input: vec![Tok(OpenParen), Expression, Tok(CloseParen)],
            output: Expression,
        },
        Rule {
            input: vec![Expression, InfixFunc, Expression],
            output: Expression,
        },
        Rule {
            input: vec![Name],
            output: Expression,
        },
        Rule {
            input: vec![Num],
            output: Expression,
        },
        //Statement definitions
        Rule {
            input: vec![Declare],
            output: Statement,
        },
        //Infix definitions
        Rule {
            input: vec![Tok(Plus)],
            output: InfixFunc,
        },
        Rule {
            input: vec![Tok(Minus)],
            output: InfixFunc,
        },
    ]
};

use std::process::Command;
use crate::lexer::Keyword::Fnc;
use crate::lexer::Token;
use crate::lexer::Token::{
    CloseCurly, CloseParen, End, Keywords, Minus, Number, OpenCurly, OpenParen, Plus,
};
use crate::parser::Component::{
    Body, Declare, Expression, Function, InfixFunc, Name, Num, Statement, Tok,
};

fn equal(stack: &Vec<Component>, pattern: &Vec<Component>) -> bool {
    let j = 0;
    for i in (stack.len() - pattern.len())..stack.len() {
        if *stack[i] != *stack[j] {
            return false;
        }
    }
    true
}

fn shrdc(stack: &Vec<Component>) {
    for rule in RULES.iter() {
        let rulen = rule.input.len();

        if rulen > stack.len() {
            continue;
        }

        if equal(&stack, &rule.input) {
            stack.truncate(stack.len() - rulen);
            stack.push(*rule.output);
        }
    }

}

pub fn parse(tokens: &Vec<Token>) -> Vec<Component> {
    let mut stack = Vec::new();

    for token in tokens {
        stack.push(Tok(token.clone()));


    }
    stack
}
