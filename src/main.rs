mod env;
mod eval;
mod gen;
mod repl;

use std::io;
use std::env::args;
use repl::repl;


fn main() -> io::Result<()> {
    for (i, arg) in args().enumerate() {
        if i == 1 {
            println!("{}", arg);
        }
    }
    repl()?;
    /*
    gen::write_wav();
    */
    Ok(())
}
