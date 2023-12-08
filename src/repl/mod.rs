use core::fmt;
use std::{io::{self, Write}, fs, collections::HashMap};
use crate::env::{update_symbols, parse_input, SymbolDef};
use crate::eval::{Atom, Status};

pub fn repl() -> io::Result<()> {
    let mut env = HashMap::<String, SymbolDef>::new();
    let mut cell_count = 0;
    'repl: loop {
        print!(">>>>> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("couldn't read input");
        let atoms = parse_input(&mut env, &input);
        for atom in atoms {
            cell_count += 1;
            match atom {
                Atom::StatusMsg(Status::Quit) => {
                    break 'repl
                    println!("{: >4}: {}", "REPL", atom);
                }
                Atom::StatusMsg(Status::LoadModule(ref module_name)) => {
                    println!("{: >4}: {}", "REPL", atom);
                    if let Ok(module_contents) = fs::read_to_string(module_name) {
                        println!("{: >4}: {}", "REPL", "Successfully loaded module");
                        update_symbols(&mut env, module_contents.as_str());
                    } else {
                        println!("{: >4}: {}", "REPL", "Failed to load module");
                        continue;
                    };
                }
                _ => {
                    println!("{: >4}: {}", cell_count, atom);
                }
            }
        }
        println!("");
    }
    Ok(())
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Quit => write!(f, "quitting"),
            Status::LoadModule(m) => write!(f, "loading module {}", m),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::None => { write!(f, "None") }
            Atom::Number(n) => { write!(f, "{}", n) }
            Atom::Boolean(b) => { if *b == true { write!(f, "TRUE") } else { write!(f, "FALSE") } }
            Atom::String(s) => { write!(f, "{}", s) }
            Atom::Frequency(fr, d) => write!(f, "frequency {}, duration {}", fr, d),
            Atom::Symbol(s) => { write!(f, "{}", s) }
            Atom::List(items) => {
                let mut s = "(".into();
                for (index, item) in items.into_iter().enumerate() {
                    if index == 0 {
                        s = format!("{}{}", s, item.to_string());
                    } else {
                        s = format!("{} {}", s, item.to_string());
                    }
                }
                s = format!("{})", s);
                write!(f, "{}", s)
            }
            Atom::Error(e) => write!(f, "ERROR: {}", e),
            Atom::StatusMsg(m) => write!(f, "{}", m),
            Atom::Arg(_) => panic!("Atom::Arg should never be displayed"),
        }
    }
}
