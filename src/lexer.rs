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

    Name(Option<String>),

    At,
    Lattice,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Backslash,
    Ampersand,
    Caret,
    Vline,
    Epoint,
    LogAnd,
    LogOr,
    Equ,
    Nequ,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    OpenArrow,
    CloseArrow,
    LeftArrow,
    RightArrow,
    Keywords(Keyword),

    Inc,
    Dec,
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
    let map = HashMap::from([
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
        (String::from("%"), Token::Percent),
        (String::from("\\"), Token::Backslash),
        (String::from("&"), Token::Ampersand),
        (String::from("|"), Token::Vline),
        (String::from("^"), Token::Caret),
        (String::from("!"), Token::Epoint),
        (String::from("#"), Token::Lattice),
        (String::from("@"), Token::At),
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
        (String::from("<<"), Token::LeftArrow),
        (String::from(">>"), Token::RightArrow),
        (String::from(","), Token::Comma),
        (String::from(";"), Token::End),
        (String::from("++"), Token::Inc),
        (String::from("--"), Token::Dec),
    ]);

    map
}

fn is_alph(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c.is_digit(10)
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
            if p1 > 0 {
                let trimed = string[0..p1].trim();
                if !trimed.is_empty() {
                    strs.push(trimed.to_string());
                }
            }
            break;
        }
    }

    if p1 == 0 && !is_alph(string.chars().next().unwrap()) {
        strs.push(string.clone().trim().to_string());
        return strs;
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
    let separators = vec![
        "(", ")", "{", "}", "[", "]",
        ":=", "++", "--", "-", "+", "*", "/", "%", "==", "=", "!=", "<<", ">>", "<", ">", "&&", "||", "!", "~", "&", "|", "^", ";",
    ];

    for morfem in morfems {
        if is_alph(morfem.chars().next().unwrap()) {
            cntxt.tokens.push(match cntxt.str2tok.get(&morfem) {
                Some(token) => token.clone(),
                None => match morfem.parse::<i64>() {
                    Ok(number) => Token::Number(number),
                    Err(_) => Token::Name(Some(morfem)),
                },
            });
        } else {
            while cntxt.index < morfem.len() {
                for separator in separators.clone() {
                    if morfem[cntxt.index..].starts_with(separator) {
                        cntxt
                            .tokens
                            .push(cntxt.str2tok.get(separator).unwrap().clone());
                        cntxt.index += separator.len() - 1;
                        break;
                    }
                }
                cntxt.index += 1;
            }
            cntxt.index = 0;
        }
    }
    cntxt.tokens
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer::Keyword;
    use crate::lexer::Token::*;

    fn test(input: String, expected_tokens: Vec<Token>) {
        let tokens = lex(input);
        assert_eq!(
            tokens.len(),
            expected_tokens.len(),
            "Token lengths don't match."
        );
        for (i, (token, expected_token)) in tokens.iter().zip(expected_tokens.iter()).enumerate() {
            assert_eq!(token, expected_token, "Token mismatch at index {}", i);
        }
    }

    #[test]
    fn test_simple() {
        let input = String::from("qwe123 := 42 ;");
        let expected_tokens = vec![Name(Some("qwe123".to_string())), Declaration, Number(42), End];
        test(input, expected_tokens);
    }

    #[test]
    fn test_funcs() {
        let input = String::from(
            "fnc foo() {\n\
            a := 5;\n\
            a2 := a * (42 + 13);\n\
            ret a +a2;\n\
        }\
        \
        fnc main() {\
            x := 42;\
            foo();\
            ret 0;\
        }",
        );

        let expected_tokens: Vec<Token> = vec![
            Keywords(Keyword::Fnc), Name(Some("foo".to_string())), OpenParen, CloseParen, OpenCurly,
            Name(Some("a".to_string())), Declaration, Number(5), End,
            Name(Some("a2".to_string())), Declaration, Name(Some("a".to_string())), Star, OpenParen, Number(42), Plus, Number(13), CloseParen, End,
            Keywords(Keyword::Ret), Name(Some("a".to_string())), Plus, Name(Some("a2".to_string())), End,
            CloseCurly,
            Keywords(Keyword::Fnc), Name(Some("main".to_string())), OpenParen, CloseParen, OpenCurly,
            Name(Some("x".to_string())), Declaration, Number(42), End,
            Name(Some("foo".to_string())), OpenParen, CloseParen, End,
            Keywords(Keyword::Ret), Number(0), End,
            CloseCurly,
        ];
        test(input, expected_tokens);
    }

    #[test]
    fn test_arithmetic_operations() {
        let input = String::from("a := 5 * 3 + (2 - 1) / 2;");
        let expected_tokens = vec![
            Name(Some("a".to_string())), Declaration, Number(5), Star, Number(3), Plus,
            OpenParen, Number(2), Minus, Number(1), CloseParen, Slash, Number(2), End
        ];
        test(input, expected_tokens);
    }

    #[test]
    fn test_variable_assignment() {
        let input = String::from("var1if := 100; var2 := var1if + 50;");
        let expected_tokens = vec![
            Name(Some("var1if".to_string())), Declaration, Number(100), End,
            Name(Some("var2".to_string())), Declaration, Name(Some("var1if".to_string())), Plus, Number(50), End
        ];
        test(input, expected_tokens);
    }

    #[test]
    fn test_conditional_statements() {
        let input = String::from(
            "if a > 0 {\
                b := 1;\
            } else {\
                b := 2;\
            }",
        );
        let expected_tokens = vec![
            Keywords(Keyword::If), Name(Some("a".to_string())), CloseArrow, Number(0), OpenCurly,
            Name(Some("b".to_string())), Declaration, Number(1), End,
            CloseCurly, Keywords(Keyword::Else), OpenCurly,
            Name(Some("b".to_string())), Declaration, Number(2), End,
            CloseCurly,
        ];
        test(input, expected_tokens);
    }

    #[test]
    fn test_while_loop() {
        let input = String::from(
            "for forx < 10 {\
                forx := forx + 1;\
            }",
        );
        let expected_tokens = vec![
            Keywords(Keyword::For), Name(Some("forx".to_string())), OpenArrow, Number(10), OpenCurly,
            Name(Some("forx".to_string())), Declaration, Name(Some("forx".to_string())), Plus, Number(1), End,
            CloseCurly,
        ];
        test(input, expected_tokens);
    }

    #[test]
    fn test_invalid_syntax() {
        let input = String::from("a := 5 + +;\
        b := &**a;");
        let expected_tokens = vec![
            Name(Some("a".to_string())), Declaration, Number(5), Plus, Plus, End,
            Name(Some("b".to_string())), Declaration, Ampersand, Star, Star, Name(Some("a".to_string())), End,
        ];
        test(input, expected_tokens); // Or throw an error depending on lexer implementation
    }

    #[test]
    fn test_bad_names() {
        let input = String::from(
            "ifa := 5;\
             fncfor := iret + 2223;",
        );
        let expected_tokens = vec![
            Name(Some("ifa".to_string())), Declaration, Number(5), End,
            Name(Some("fncfor".to_string())), Declaration, Name(Some("iret".to_string())), Plus, Number(2223), End
        ];
        test(input, expected_tokens);
    }

    #[test]
    fn test_multiple_statements() {
        let input = String::from("a := 5; b := a + 3; c := b * 2;");
        let expected_tokens = vec![
            Name(Some("a".to_string())), Declaration, Number(5), End,
            Name(Some("b".to_string())), Declaration, Name(Some("a".to_string())), Plus, Number(3), End,
            Name(Some("c".to_string())), Declaration, Name(Some("b".to_string())), Star, Number(2), End
        ];
        test(input, expected_tokens);
    }
}
