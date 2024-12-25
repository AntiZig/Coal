enum Token {
    Number(i64),
    Assign,
    Declaration,
    Variable(String),
}

pub fn lex(str: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let startoftoken = 0;
    let mut canbenumber = false;
    let mut canbevar = false;

    for mut i in 0..str.len() {
        let c = str.chars().nth(i).unwrap();
        match c {
            _ if c.is_ascii_alphabetic() => {
                canbenumber = false;
                canbevar = true;
            }
            ':' => if str.chars().nth(i + 1).unwrap() == '=' {
                if canbevar {
                    tokens.push(Token::Variable(str[startoftoken..i].to_string()));
                } else {
                    panic!("number can not be Declareted")
                }
                tokens.push(Token::Declaration);
                i += 1;
                canbenumber = true;
                canbevar = true;
            }
            '=' => {
                if canbevar {
                    tokens.push(Token::Variable(str[startoftoken..i].to_string()));
                } else {
                    panic!("number can not be Assigned");
                }
                tokens.push(Token::Assign);
                canbenumber = true;
                canbevar = true;
            }
            _ => {}
        }
    }
    tokens
}