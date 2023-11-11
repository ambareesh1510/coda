use crate::env::Env;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Quit,
    LoadModule(String),
}

// strategy for processing functions:
// traverse ast, make hashmap of function name to defun node
// create eval method for atom that takes an optional vec of args
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Number(f32),
    Boolean(bool),
    Symbol(String),
    String(String),
    List(Vec<Atom>),
    Error(String),
    StatusMsg(Status),
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
                            "defvar" => {
                                if items.len() != 3 {
                                    return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected 2, found {})", items.len() - 1));
                                }
                                let Atom::Symbol(var_name) = items[1].clone() else {
                                    return Atom::Error(format!("Argument 1 of `{name}` has incorrect type: expected Symbol"));
                                };
                                let var_value = items[2].eval(env);
                                env.symbol_table.insert(var_name, var_value);
                                items[2].eval(env)
                            }
                            _ => Atom::Error(format!("Function `{name}` not found")),
                        }
                    }
                    _ => self.clone(),
                }
            }
            Self::Symbol(name) => match env.symbol_table.get(name) {
                Some(atom) => atom.clone(),
                None => Atom::Error(format!("Symbol `{name}` not found")), // lookup in symbol table
            },
            _ => self.clone(),
        }
    }

    pub fn is_same_type(&self, other: &Self) -> bool {
        let self_type = std::mem::discriminant(self);
        let other_type = std::mem::discriminant(other);
        self_type == other_type
    }
}

pub struct Arg {
    identifier: String,
    arg_type: Atom,
}

