use std::collections::HashMap;
use crate::env::{Env, SymbolDef};

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Quit,
    LoadModule(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Number(f32),
    Boolean(bool),
    Symbol(String),
    String(String),
    List(Vec<Atom>),
    Error(String),
    StatusMsg(Status),
    Arg(usize),
    None,
}

impl Atom {
    pub fn eval(&self, env: &mut Env) -> Self {
        match self {
            Self::List(items) => {
                match items[0] {
                    Self::Symbol(ref name) => {
                        match name.as_str() {
                            // TODO: deal with case where variable is first entry of list, and
                            // therefore gets parsed as a function
                            "quit" => {
                                return Atom::StatusMsg(Status::Quit);
                            }
                            "load" => {
                                if items.len() != 2 {
                                    return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected 1, found {})", items.len() - 1));
                                }
                                let Atom::String(filename) = items[1].eval(env) else {
                                    return Atom::Error(format!("Argument 1 of `{name}` has incorrect type: expected String"));
                                };
                                return Atom::StatusMsg(Status::LoadModule(filename));
                            }
                            "+" | "-" | "*" | "/" => {
                                if items.len() < 2 {
                                    return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected 1+, found 0)"));
                                }
                                let Atom::Number(mut result) = items[1].eval(env) else {
                                    return Atom::Error(format!("Argument 1 of `{name}` has incorrect type: expected Number"));
                                };
                                for i in 2..items.len() {
                                    if let Atom::Number(item) = items[i].eval(env) {
                                        match name.as_str() {
                                            "+" => result += item,
                                            "-" => result -= item,
                                            "*" => result *= item,
                                            "/" => result /= item,
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        return Atom::Error(format!("Argument {i} of `{name}` has incorrect type: expected Number"));
                                    }
                                }
                                return Atom::Number(result);
                            }
                            "sin" | "cos" | "tan" => {
                                if items.len() != 2 {
                                    return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected 1, found {})", items.len() - 1));
                                }
                                let Atom::Number(val) = items[1].eval(env) else {
                                    return Atom::Error(format!("Argument 1 of `{name}` has incorrect type: expected Number"));
                                };
                                return match name.as_str() {
                                    "sin" => Atom::Number(val.sin()),
                                    "cos" => Atom::Number(val.cos()),
                                    "tan" => Atom::Number(val.tan()),
                                    _ => unreachable!(),
                                };
                            }
                            "equals" => {
                                if items.len() < 3 {
                                    return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected 2+, found {})", items.len() - 1));
                                };
                                let mut result = true;
                                for i in 1..items.len() - 1 {
                                    if !(items[i].eval(env) == items[i + 1].eval(env)) {
                                        result = false;
                                    }
                                }
                                Atom::Boolean(result)
                            }
                            "if" => {
                                if items.len() != 4 {
                                    return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected 3, found {})", items.len() - 1));
                                }
                                let Atom::Boolean(cond) = items[1].eval(env) else {
                                    return Atom::Error(format!("Argument 1 of `{name}` has incorrect type: expected Boolean"));
                                };
                                if cond {
                                    items[2].eval(env)
                                } else {
                                    items[3].eval(env)
                                }
                            }
                            "def" => {
                                match items.len() {
                                    3 => {
                                        let Atom::Symbol(var_name) = items[1].clone() else {
                                            return Atom::Error(format!("Argument 1 of `{name}` has incorrect type: expected Symbol"));
                                        };
                                        let var_value = items[2].eval(env);
                                        env.symbol_table.insert(var_name, SymbolDef {
                                            args: HashMap::new(),
                                            eval: var_value.clone(),
                                        });
                                        var_value
                                    },
                                    4 => {
                                        let Atom::Symbol(fn_name) = items[1].clone() else {
                                            return Atom::Error(format!("Argument 1 of `{name}` has incorrect type: expected Symbol"));
                                        };
                                        let Atom::List(args) = items[2].clone() else {
                                            return Atom::Error(format!("Argument 2 of `{name}` has incorrect type: expected List"));
                                        };
                                        let mut args_map = HashMap::new();
                                        for arg in args {
                                            let arg_number = args_map.len();
                                            let Atom::Symbol(arg_name) = arg else {
                                                return Atom::Error(format!("Expected Symbols in parameter list of `{name}`"));
                                            };
                                            args_map.insert(arg_name, SymbolDef {
                                                args: HashMap::new(),
                                                eval: Atom::Arg(arg_number),
                                            });
                                        }
                                        let fn_eval = items[3].parse_args(&mut Env {
                                            symbol_table: args_map.clone(),
                                        });
                                        env.symbol_table.insert(fn_name.clone(), SymbolDef {
                                            args: args_map,
                                            eval: fn_eval,
                                        });
                                        println!("{:?}", env.symbol_table.get(&fn_name));
                                        Atom::None
                                    }
                                    _ => return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected 2, found {})", items.len() - 1)),
                                }
                            }
                            other => {
                                match env.symbol_table.get(other) {
                                    Some(symbol_def) => {
                                        if items.len() - 1 != symbol_def.args.len() {
                                            return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected {}, found {})", symbol_def.args.len(), items.len() - 1));
                                        }
                                        symbol_def.eval.substitute_args(items).eval(env)
                                    }
                                    None => Atom::Error(format!("Function `{name}` not found")),
                                }
                            }
                        }
                    }
                    _ => self.clone(),
                }
            }
            Self::Symbol(name) => match env.symbol_table.get(name) {
                Some(symbol_def) => match symbol_def.args.len() {
                    0 => return symbol_def.eval.clone(),
                    _ => todo!("Functions are not yet supported as first-class objects"),
                },
                None => Atom::Error(format!("Symbol `{name}` not found")), // lookup in symbol table
            },
            _ => self.clone(),
        }
    }

    pub fn substitute_args(&self, args: &Vec<Atom>) -> Self {
        match self {
            Atom::List(list_items) => {
                let mut return_items = vec![];
                for item in list_items  {
                    return_items.push(item.substitute_args(args));
                }
                Atom::List(return_items)
            }
            Atom::Arg(arg_num) => args[arg_num + 1].clone(),
            other => other.clone()
        }
    }

    pub fn parse_args(&self, env: &Env) -> Self {
        match self {
            Atom::List(list_items) => {
                let mut return_items = vec![];
                for item in list_items  {
                    if let Atom::Error(_) = item.parse_args(env) {
                        return item.parse_args(env);
                    }
                    return_items.push(item.parse_args(env));
                }
                Atom::List(return_items)
            }
            Atom::Symbol(name) if !self.is_reserved_keyword() => {
                match env.symbol_table.get(name) {
                    Some(symbol_def) => match symbol_def.args.len() {
                        0 => symbol_def.eval.clone(),
                        _ => todo!("Sub-functions, e.g. define function where temporary variables within function = ..."),
                    }, 
                    None => Atom::Error(format!("Argument `{name}` not found")),
                }
            }
            other => other.clone()
        }
    }

    pub fn is_reserved_keyword(&self) -> bool {
        let Atom::Symbol(name) = self else {
            return false;
        };
        match name.as_str() {
            "+" | "-" | "*" | "/"
                | "sin" | "cos" | "tan"
                | "equals" | "if" | "def" => true,
            _ => false,
        }
    }
}

