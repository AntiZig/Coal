use crate::lexer::Keyword;
use crate::lexer::Keyword::*;
use crate::lexer::Token;
use crate::lexer::Token::*;

//enum ASTNode {
//    Leaf(Token),
//    Node(Token, Vec<ASTNode>),
//}

struct ASTNode {
    node: Option<(Token, Vec<ASTNode>)>,
}

//#[derive(Clone, Debug)]
//struct ASTNode {
//    head: Token,
//    nodes: Vec<ASTNode>,
//}

//impl ASTNode {
//    fn new(head: Token) -> Self {
//        ASTNode {
//            head,
//            nodes: Vec::new(),
//        }
//    }
//}

fn func_match(tokens: Vec<Token>) -> bool {
    if tokens.is_empty() {
        return false;
    }
    let lookahead = tokens.first().unwrap();
    let a = match lookahead {
        Keywords(Fnc) => true,
        _ => return false,
    };

    return false;
}
