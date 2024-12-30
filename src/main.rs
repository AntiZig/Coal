use lexer::lex;

mod code_gen_try;
mod lexer;
mod parser;
mod startup;

fn main() {
    let code = startup::get_code();
    let lexems = lex(code);
    println!("{:?}", lexems);
}
