mod env;
mod eval;
mod gen;
mod repl;

use std::io;
use std::env::args;
use repl::repl;


fn main() -> io::Result<()> {
    // gen::_write_wav();
    // return Ok(());
    for (i, arg) in args().enumerate() {
        if i == 1 {
            println!("{}", arg);
        }
    }
    repl()?;
    Ok(())
}
