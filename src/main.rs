mod env;
mod gen;
mod eval;
mod repl;

use std::io::{self, Write};

use crate::{
    eval::{Atom, Status},
    env::Env,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = Env::new("");
    'repl: loop {
        print!("(repl) > ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("couldn't read input");
        let atoms = env.parse_input(&input);
        for atom in atoms {
            match atom {
                Atom::StatusMsg(Status::Quit) => break 'repl,
                Atom::StatusMsg(Status::LoadModule(module_name)) => todo!("Load a module from source file"),
                _ => println!("{:?}", atom),
            }
        }
    }
    /*
    gen::write_wav();
    */
    Ok(())
}
