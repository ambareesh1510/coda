mod env;
mod gen;
mod lexer;
mod parser;
mod repl;

use std::io::{self, Write};

use crate::lexer::{Atom, Status};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = env::Env::new("");
    'repl: loop {
        print!("(repl) > ");
        io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("couldn't read input");
        let atoms = env.parse_input(&input);
        for atom in atoms {
            if let Atom::StatusMsg(Status::Quit) = atom {
                break 'repl;
            }
            println!("{:?}", atom);
        }
    }
    /*
    parser::parse_and_print();
    gen::write_wav();
    */
    Ok(())
}
