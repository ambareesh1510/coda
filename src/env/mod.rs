use crate::eval::Atom;
use std::{collections::HashMap};

#[derive(Clone, Debug)]
pub struct SymbolDef {
    pub args: HashMap<String, SymbolDef>,
    pub eval: Atom,
}

pub fn update_symbols(symbol_table: &mut HashMap<String, SymbolDef>, s: &str) {
    let s = parse_input(symbol_table, s);
    for atom in s {
        atom.eval(symbol_table);
    }
}
pub fn parse_input(symbol_table: &mut HashMap<String, SymbolDef>, s: &str) -> Vec<Atom> {
    let tokens = tokenize(s);
    let (mut parsed_ast, _) = construct_ast(&tokens);
    parsed_ast = parsed_ast
        .into_iter()
        .map(|e| e.eval(symbol_table))
        .collect::<Vec<Atom>>();
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
                construct_ast(tokens.split_at(token_ptr + 1).1);
            token_ptr += atom_list_len;
            result_vec.push(Atom::List(atom_list));
        } else if tokens[token_ptr] == "TRUE" {
            result_vec.push(Atom::Boolean(true));
        } else if tokens[token_ptr] == "FALSE" {
            result_vec.push(Atom::Boolean(false));
        } else if &tokens[token_ptr][0..1] == "\"" {
            result_vec.push(Atom::String(String::from(
                &tokens[token_ptr][1..&tokens[token_ptr].len() - 1],
            )));
        } else if tokens[token_ptr].contains(":") {
            let mut note = String::from("");
            let mut octave = 0;
            let mut num_chars = 0;
            loop {
                let next_char = &tokens[token_ptr][num_chars..num_chars + 1];
                if next_char == ":" {
                    panic!("Failed to parse note");
                }
                if "ABCDEFGR#b".contains(next_char) {
                    note.push_str(next_char);
                    num_chars += 1;
                } else {
                    break;
                }
            }
            let mut chars_end = num_chars + 1;
            loop {
                let next_chars = &tokens[token_ptr][num_chars..chars_end];
                if let Ok(o) = i32::from_str_radix(next_chars, 10) {
                    octave = o;
                    chars_end += 1;
                } else if &next_chars[next_chars.len() - 1..next_chars.len()] != ":" {
                    panic!("Failed to parse octave");
                } else {
                    break;
                }
            }
            num_chars = chars_end;
            let Ok(duration) = &tokens[token_ptr][num_chars..].parse::<f32>() else {
                panic!("Failed to parse duration");
            };
            result_vec.push(Atom::List(vec![Atom::Number(calculate_frequency_from_note(&note, octave)), Atom::Number(*duration)]));
        } else {
            result_vec.push(Atom::Symbol(tokens[token_ptr].clone()));
        }
        token_ptr += 1;
    }
    (result_vec, increment)
}

const A4_FREQUENCY: f32 = 440.0;
const A4_KEY_NUMBER: i32 = 49;

fn calculate_frequency_from_note(note: &String, octave: i32) -> f32 {
    let mut key_num: i32 = match note.as_str() {
        "A" => 1,
        "A#" | "Bb" => 2,
        "B" => 3,
        // C1 starts here, so subtract by one octave
        "C" => -8,
        "C#" | "Db" => -7,
        "D" => -6,
        "D#" | "Eb" => -5,
        "E" => -4,
        "F" => -3,
        "F#" | "Gb" => -2,
        "G" => -1,
        "G#" | "Ab" => 0,
        "R" => return 0.,
        _ => panic!("Error parsing note"),
    };
    key_num += octave as i32 * 12;
    A4_FREQUENCY * (2.0f32).powf(1.0 / 12.0 * (key_num - A4_KEY_NUMBER) as f32)
}
