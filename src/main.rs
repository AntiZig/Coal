mod lexer;
mod startup;

fn main() {
    let code = startup::get_code();
    println!("{}", code);
}
