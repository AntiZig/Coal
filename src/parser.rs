

enum Elementary {
    Keyw(Keyword),
    Expression,
    Name,
}

enum symbol {
    Composite(Vec<symbol>),
    Elem(Elementary),
}

struct rule {
    input: Vec<symbol>,
    output: symbol,
}

const RULES: Vec<rule> = {vec![
    rule { //Function
        input: {vec![
            symbol::Elem(Elementary::Keyw(Keyword::Fnc)),
            symbol::Elem(Elementary::Name),
            symbol::
        ]}}
]};

use crate::lexer::Keyword;
use crate::lexer::Token;

pub fn parse(tokens: &Vec<Token>) {

}