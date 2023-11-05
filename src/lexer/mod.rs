// strategy for processing functions:
// traverse ast, make hashmap of function name to defun node
// create eval method for atom that takes an optional vec of args
#[derive(Debug)]
pub enum Atom {
    Number(f32),
    Boolean(bool),
    Symbol(String),
    List(Vec<Atom>),
}

impl Atom {
    pub fn eval(&self) -> &Self {
        match self {
            Self::List(items) => {
                match items[0] {
                    Self::Symbol(ref name) => {
                        todo!()
                    }
                    _ => self
                }
            }
            Self::Symbol(name) => unimplemented!(), // lookup in symbol table
            _ => self
        }
    }

    pub fn is_same_type(&self, other: &Self) -> bool {
        let self_type = match self {
            Self::Number(_) => 0,
            Self::Boolean(_) => 1,
            Self::Symbol(_) => 2,
            Self::List(_) => 3,
        };

        let other_type = match other {
            Self::Number(_) => 0,
            Self::Boolean(_) => 1,
            Self::Symbol(_) => 2,
            Self::List(_) => 3,
        };
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
        let mut s = s.split(' ').map(|e| e.to_string()).collect::<Vec<String>>();
        s.retain(|e| e != "");
        s
    }
    pub fn tokenize_self(&mut self) {
        self.tokens = Self::tokenize(&self.source);
    }
    pub fn construct_ast(tokens: &[String]) -> Vec<Atom> {
        let mut token_ptr: usize = 0;
        let mut result_vec = vec![];
        while token_ptr < tokens.len() {
            if let Ok(num) = tokens[token_ptr].parse::<f32>() {
                result_vec.push(Atom::Number(num));
            } else if tokens[token_ptr] == ")" {
                /*
                token_ptr += 1;
                continue;
                */
                println!("break");
                break;
            } else if tokens[token_ptr] == "(" {
                /*
                let list_items = Vec::<Atom>::new();
                while tokens[token_ptr] != ")" {
                    list_items.push(construct_ast(tokens))
                }
                */
                let atom_list = Self::construct_ast(tokens.split_at(token_ptr + 1).1);
                let list_len = atom_list.len();
                result_vec.push(Atom::List(atom_list));
                println!("push list length {}", list_len);
                token_ptr += (list_len + 1);
            } else {
                result_vec.push(Atom::Symbol(tokens[token_ptr].clone()))
            }
            token_ptr += 1;
        }
        result_vec
    }
}
