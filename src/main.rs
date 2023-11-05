mod parser;
mod lexer;
mod gen;

use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("(repl) > ");
        io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("couldn't read input");
        if input.trim() == "(quit)" {
            break;
        }
        let tokens = lexer::Parser::tokenize(&input);
        println!("{:?}", tokens);
        println!("{:?}", lexer::Parser::construct_ast(&tokens));
    }
    /*
    let a = Atom::List(vec![
        Atom::Symbol("+".into()),
        Atom::Number(1.0),
        Atom::Number(1.0),
    ]);
    let b = Atom::Boolean(true);
    println!("{:?}", b.eval());
    */
    /*
    parser::parse_and_print();
    gen::write_wav();
    */
    Ok(())
}
