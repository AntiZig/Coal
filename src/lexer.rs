#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),

    Sum,
    Plus,
    Mul,
    And,
    Or,
    Xor,
    Not,
    LogAnd,
    LogOr,

    Assign,
    Declaration,

    Variable(String),
    Function(String),

    LeftBracket,
    RightBracket,

    Dot,
    Comma,

    If,
    Else,
    For,
    Fnc,
    Ret,

    End,
}

struct Context {
    startoftoken: usize,
    canbenumber: bool,
    canbevar: bool,
    index: usize,
    tokens: Vec<Token>,
}

impl Context {
    pub fn parse(&mut self, string: &str, newsubstr: &str) {
        let strofoldtoken: String = string[self.startoftoken..self.index].to_string();
        if self.canbenumber {
            self.tokens
                .push(Token::Number(strofoldtoken.parse::<i64>().unwrap()));
        } else if self.canbevar {
            self.tokens.push(Token::Variable(strofoldtoken));
        } else {
            self.tokens.push(Token::Function(strofoldtoken));
        }
        self.index += newsubstr.len() - 1;
        self.canbenumber = true;
        self.canbevar = false;
        self.startoftoken = self.index + 1;
    }
}
const IF: &str = "if";
const DECL: &str = ":=";
const ASSIGN: &str = "=";
const END: &str = ";";

//qwe123:=45;
//0123456789
pub fn lex(str: &str) -> Vec<Token> {
    let mut cntxt = Context {
        startoftoken: 0,
        canbevar: false,
        canbenumber: true,
        index: 0,
        tokens: Vec::new(),
    };

    while cntxt.index < str.len() {
        let c = str.chars().nth(cntxt.index).unwrap();
        match c {
            _ if c.is_ascii_alphabetic() => {
                cntxt.canbenumber = false;
                cntxt.canbevar = true;
            }
            _ if c.is_ascii_digit() => {
                cntxt.index += 1;
                continue;
            }
            ' ' => {
                cntxt.canbenumber = false;
                cntxt.canbevar = false;
            }
            ';' => {
                if cntxt.index != cntxt.startoftoken {
                    cntxt.parse(str, END);
                }
                cntxt.tokens.push(Token::End)
            }
            _ => match str[cntxt.index..].to_string() {
                s if s.starts_with(DECL) => {
                    cntxt.parse(str, DECL);
                    cntxt.tokens.push(Token::Declaration);
                }
                s if s.starts_with(ASSIGN) => {
                    cntxt.parse(str, ASSIGN);
                    cntxt.tokens.push(Token::Assign);
                }
                _ => {}
            }
        }
        cntxt.index += 1;
    }
    cntxt.tokens
}
