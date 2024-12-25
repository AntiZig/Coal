mod lexer;
mod parser;

use lexer::Token;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "qwe123:=42;";
        let expected_tokens = vec![
            Token::Variable("qwe123".to_string()),
            Token::Declaration,
            Token::Number(42),
            Token::End
        ];

        let tokens = lexer::lex(input);

        assert_eq!(tokens.len(), expected_tokens.len(), "Token lengths don't match.");

        for (i, (token, expected_token)) in tokens.iter().zip(expected_tokens.iter()).enumerate() {
            assert_eq!(token, expected_token, "Token mismatch at index {}", i);
        }
    }
}

