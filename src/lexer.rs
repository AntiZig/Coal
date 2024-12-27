#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    If,
    Else,
    For,
    Fnc,
    Ret,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),

    Assign,
    Declaration,

    Name(String),

    At, Lattice,
    Plus, Minus, Star, Slash, Percent,
    Backslash,
    Ampersand, Caret, Vline, Epoint, LogAnd, LogOr, Equ, Nequ,
    OpenParen, CloseParen, OpenBracket, CloseBracket, OpenCurly, CloseCurly, OpenArrow, CloseArrow,
    Keywords(Keyword),

    Dot,
    Comma,

    End,
}

struct Context {
    index: usize,
    tokens: Vec<Token>,
    str2tok: HashMap<String, Token>,
}


use std::collections::HashMap;

fn init() -> HashMap<String, Token> {
    let mut map = HashMap::from([
        (String::from(":="), Token::Declaration),
        (String::from("="), Token::Assign),

        (String::from("if"), Token::Keywords(Keyword::If)),
        (String::from("else"), Token::Keywords(Keyword::Else)),
        (String::from("ret"), Token::Keywords(Keyword::Ret)),
        (String::from("for"), Token::Keywords(Keyword::For)),
        (String::from("fnc"), Token::Keywords(Keyword::Fnc)),

        (String::from("+"), Token::Plus),
        (String::from("-"), Token::Minus),
        (String::from("*"), Token::Star),
        (String::from("/"), Token::Slash),

        (String::from("&"), Token::Ampersand),
        (String::from("|"), Token::Vline),
        (String::from("^"), Token::Caret),
        (String::from("!"), Token::Epoint),
        (String::from("&&"), Token::LogAnd),
        (String::from("||"), Token::LogOr),
        (String::from("=="), Token::Equ),
        (String::from("!="), Token::Nequ),

        (String::from("("), Token::OpenParen),
        (String::from(")"), Token::CloseParen),
        (String::from("{"), Token::OpenCurly),
        (String::from("}"), Token::CloseCurly),
        (String::from("["), Token::OpenBracket),
        (String::from("]"), Token::CloseBracket),
        (String::from("<"), Token::OpenArrow),
        (String::from(">"), Token::CloseArrow),

        (String::from("."), Token::Dot),
        (String::from(","), Token::Comma),
        (String::from(";"), Token::End),
    ]);

    map
}

fn is_alph(c: char) -> bool {
    c.is_alphabetic() || c == ' ' || c.is_digit(10)
}

fn split(string: &String) -> Vec<String> {
    let mut strs: Vec<String> = vec![];
    if string.is_empty() {
        return strs;
    }
    let mut p1 = 0;
    let mut p2 = 0;

    for (i, c) in string.chars().enumerate() {
       if is_alph(c) {
           p1 = i;
           strs.push(string[0..p1].trim().to_string());
           break;
       }
    }
    for (i, c) in string[p1..].chars().enumerate() {
        if !is_alph(c) {
            p2 = i + p1;
            strs.push(string[p1..p2].to_string());
            break;
        }
    }
    strs.extend(split(&string[p2..].to_string()));
    strs
}

pub fn lex(string: String) -> Vec<Token> {

    let mut cntxt = Context {
        index: 0,
        tokens: Vec::new(),
        str2tok: init(),
    };

    let morfems = split(&string);
    let separators = vec![":=", "-", "*", "/", "%", "==", "=", "!=", "<", ">", "&&", "||", "!", "~", "&", "|", "^"];

    for morfem in morfems {
        if is_alph(morfem[0]) {
            cntxt.tokens.push(
                match cntxt.str2tok.get(&morfem) {
                    Some(token) => token.clone(),
                    None => match morfem.parse::<i64>() {
                        Ok(number) => Token::Number(number),
                        Err(_) => Token::Name(morfem),
                    }
                }
            );
        } else {
            while cntxt.index < morfem.len() {
                for separator in separators.clone() {
                    if morfem[cntxt.index..].starts_with(separator) {
                        cntxt.tokens.push(cntxt.str2tok.get(&morfem[cntxt.index..]).unwrap().clone());
                        cntxt.index += separator.len() - 1;
                        break;
                    }
                }
                cntxt.index += 1;
            }
        }
    }
    cntxt.tokens
}
