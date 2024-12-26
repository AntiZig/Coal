mod lexer;
mod parser;

use lexer::*;
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token::{Brackets, Keywords, Number, Variable, Declaration, End, Assign, PrimFuncs, Bodys};
    use crate::lexer::Bracket;
    use crate::lexer::Body;
    use crate::lexer::PrimitiveFnc;
    use crate::lexer::Keyword;

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
        let expected_tokens = vec![
            Variable("qwe123".to_string()),
            Declaration,
            Number(42),
            End,
        ];
        test(input, expected_tokens);
    }

    #[test]
    fn test_funcs() {
        let input = String::from(
            "fnc foo() {\
            a := 5;\
            a2 := a * (42 + 13);\
            ret a +a2;\
        }\
        \
        fnc main() {\
            x := 42;\
            foo();\
            ret 0;\
        }"
        );
        let expected_tokens: Vec<Token> = vec![
            Keywords(Keyword::Fnc),
            Variable("foo".to_string()),
            Brackets(Bracket::LeftBracket),
            Brackets(Bracket::RightBracket),
            Bodys(Body::Open),

            Variable("a".to_string()),
            Declaration,
            Number(5),
            End,

            Variable("a2".to_string()),
            Declaration,
            Variable("a".to_string()),
            PrimFuncs(PrimitiveFnc::Mul),
            Brackets(Bracket::LeftBracket),
            Number(42),
            PrimFuncs(PrimitiveFnc::Sum),
            Number(13),
            Brackets(Bracket::RightBracket),
            End,

            Keywords(Keyword::Ret),
            Variable("a".to_string()),
            PrimFuncs(PrimitiveFnc::Sum),
            Variable("a2".to_string()),
            End,

            Bodys(Body::Close),

            Keywords(Keyword::Fnc),
            Variable("main".to_string()),
            Brackets(Bracket::LeftBracket),
            Brackets(Bracket::RightBracket),
            Bodys(Body::Open),

            Variable("x".to_string()),
            Declaration,
            Number(42),
            End,

            Variable("foo".to_string()),
            Brackets(Bracket::LeftBracket),
            Brackets(Bracket::RightBracket),
            End,

            Keywords(Keyword::Ret),
            Number(0),
            End,

            Bodys(Body::Close),
        ];
        test(input, expected_tokens);
    }
}
