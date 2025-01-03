use lexer::lex;

mod code_gen;
pub mod ir;
mod lexer;
mod startup;

fn main() {
    let code = startup::get_code();
    let lexems = lex(code);
    println!("{:?}", lexems);
}
