use crate::eval::Atom;
use std::collections::HashMap;

pub struct Env {
    // pub ast: Vec<Atom>,
    pub symbol_table: HashMap<String, SymbolDef>,
}

#[derive(Clone, Debug)]
pub struct SymbolDef {
    pub args: HashMap<String, SymbolDef>,
    pub eval: Atom,
}

impl Env {
    pub fn new(s: &str) -> Self {
        let mut new_env = Self {
            symbol_table: HashMap::new(),
        };
        new_env
    }
    pub fn parse_input(&mut self, s: &str) -> Vec<Atom> {
        let tokens = Self::tokenize(s);
        let (mut parsed_ast, _) = Self::construct_ast(&tokens);
        parsed_ast = parsed_ast.into_iter().map(|e| e.eval(self)).collect::<Vec<Atom>>();
        parsed_ast
    }
    pub fn tokenize(s: &str) -> Vec<String> {
        let s = s.replace('(', " ( ").replace(')', " ) ");
        let s = s.chars();

        let mut tokens = Vec::new();
        let mut current_string = String::new();
        let mut in_string = false;
        for c in s {
            if !in_string && c.is_whitespace() {
                    tokens.push(current_string.clone());
                    current_string = "".into();
            } else if c == '\"' {
                in_string = !in_string;
                current_string.push(c);
            } else {
                current_string.push(c);
            }
        }
        tokens.retain(|e| e != "");
        tokens
    }

    pub fn construct_ast(tokens: &[String]) -> (Vec<Atom>, usize) {
        let mut token_ptr: usize = 0;
        let mut increment = 1;
        let mut result_vec = vec![];
        while token_ptr < tokens.len() {
            if let Ok(num) = tokens[token_ptr].parse::<f32>() {
                result_vec.push(Atom::Number(num));
            } else if tokens[token_ptr] == ")" {
                increment = token_ptr + 1;
                break;
            } else if tokens[token_ptr] == "(" {
                let (atom_list, atom_list_len) =
                    Self::construct_ast(tokens.split_at(token_ptr + 1).1);
                token_ptr += atom_list_len;
                result_vec.push(Atom::List(atom_list));
            } else if tokens[token_ptr] == "TRUE" {
                result_vec.push(Atom::Boolean(true));
            } else if tokens[token_ptr] == "FALSE" {
                result_vec.push(Atom::Boolean(false));
            } else if &tokens[token_ptr][0..1] == "\"" {
                result_vec.push(Atom::String(String::from(&tokens[token_ptr][1..&tokens[token_ptr].len() - 1])));
            } else {
                result_vec.push(Atom::Symbol(tokens[token_ptr].clone()));
            }
            token_ptr += 1;
        }
        (result_vec, increment)
    }
}
