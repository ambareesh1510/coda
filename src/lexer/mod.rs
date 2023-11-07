// strategy for processing functions:
// traverse ast, make hashmap of function name to defun node
// create eval method for atom that takes an optional vec of args
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Number(f32),
    Boolean(bool),
    Symbol(String),
    List(Vec<Atom>),
    Error(String),
}

impl Atom {
    pub fn eval(&self) -> Self {
        match self {
            Self::List(items) => {
                match items[0] {
                    Self::Symbol(ref name) => match name.as_str() {
                        "+" | "-" | "*" | "/" => {
                            if items.len() < 2 {
                                return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected 1+, found 0)"));
                            }
                            let Atom::Number(mut result) = items[1].eval() else {
                                return Atom::Error(format!("Argument 1 of `{name}` has incorrect type: expected Number"));
                            };
                            for i in 2..items.len() {
                                if let Atom::Number(item) = items[i].eval() {
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
                            let Atom::Number(val) = items[1].eval() else {
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
                                if !(items[i].eval() == items[i + 1].eval()) {
                                    result = false;
                                }
                            }
                            Atom::Boolean(result)
                        }
                        "if" => {
                            if items.len() != 4 {
                                return Atom::Error(format!("Incorrect number of arguments to `{name}` (expected 3, found {})", items.len() - 1));
                            }
                            let Atom::Boolean(cond) = items[1].eval() else {
                                return Atom::Error(format!("Argument 1 of `{name}` has incorrect type: expected Boolean"));
                            };
                            if cond {
                                items[2].eval()
                            } else {
                                items[3].eval()
                            }
                        }
                        _ => Atom::Error(format!("Function `{name}` not found"))
                    }
                    _ => self.clone()
                }
            }
            Self::Symbol(name) => Atom::Error(format!("Symbol `{name}` not found")), // lookup in symbol table
            _ => self.clone()
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

pub struct Parser {
    source: String,
    tokens: Vec<String>,
    ast: Vec<Atom>,
}

impl Parser {
    pub fn tokenize(s: &str) -> Vec<String> {
        let s = s.replace('(', " ( ").replace(')', " ) ");
        let mut s = s.split_whitespace().map(|e| e.to_string()).collect::<Vec<String>>();
        s.retain(|e| e != "");
        s
    }

    pub fn tokenize_self(&mut self) {
        self.tokens = Self::tokenize(&self.source);
    }

    pub fn construct_ast(tokens: &[String]) -> (Vec<Atom>, usize) {
        let mut token_ptr: usize = 0;
        let mut increment = 1;
        let mut result_vec = vec![];
        while token_ptr < tokens.len() {
            // println!("read token {}", tokens[token_ptr]);
            if let Ok(num) = tokens[token_ptr].parse::<f32>() {
                result_vec.push(Atom::Number(num));
            } else if tokens[token_ptr] == ")" {
                increment = token_ptr + 1;
                break;
            } else if tokens[token_ptr] == "(" {
                let (atom_list, atom_list_len) = Self::construct_ast(tokens.split_at(token_ptr + 1).1);
                token_ptr += atom_list_len;
                result_vec.push(Atom::List(atom_list));
            } else if tokens[token_ptr] == "TRUE" {
                result_vec.push(Atom::Boolean(true));
            } else if tokens[token_ptr] == "FALSE" {
                result_vec.push(Atom::Boolean(false));
            } else {
                result_vec.push(Atom::Symbol(tokens[token_ptr].clone()));
            }
            token_ptr += 1;
        }
        (result_vec, increment)
    }
}
