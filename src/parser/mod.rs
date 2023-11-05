use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until, take_while, tag_no_case};
use nom::character::complete::{multispace0, multispace1, digit0, digit1, u16};
use nom::character::complete::{alpha1, alphanumeric1, line_ending};
use nom::character::{is_alphanumeric, is_newline};
use nom::combinator::opt;
use nom::multi::many0;
use nom::number::complete::{be_f32, float};
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Pitch {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(Debug, Clone)]
enum Intonation {
    DoubleSharp,
    Sharp,
    None,
    Flat,
    DoubleFlat,
}

/// A note with pitch, intonation (sharp/flat), octave, and duration.
#[derive(Debug, Clone)]
struct Note(Pitch, Intonation, Option<u16>, f32);

#[derive(Debug, Clone)]
struct Pattern {
    tempo: f32,
    base: u16,
    notes: Vec<Note>,
}

#[derive(Debug)]
enum Statement {
    Blank,
    Comment,
    Declaration { name: String, value: Pattern },
}

// Blank lines:

fn parse_blank_line(input: &str) -> IResult<&str, Statement> {
    let result = multispace1(input)?;
    Ok((result.0, Statement::Blank))
}

// Comments:

fn parse_single_line_comment(input: &str) -> IResult<&str, Statement> {
    let result = delimited(tag("//"), take_until("\n"), line_ending)(input)?;
    Ok((result.0, Statement::Comment))
}

fn parse_multi_line_comment(input: &str) -> IResult<&str, Statement> {
    let result = delimited(tag("/*"), take_until("*/"), tag("*/"))(input)?;
    Ok((result.0, Statement::Comment))
}

// Declarations:

fn parse_note(input: &str) -> IResult<&str, Note> {
    let result = tuple((
        alt((
            tag_no_case("c"),
            tag_no_case("d"),
            tag_no_case("e"),
            tag_no_case("f"),
            tag_no_case("g"),
            tag_no_case("a"),
            tag_no_case("b"),
        )),
        opt(alt((
            tag("##"),
            tag("bb"),
            tag("#"),
            tag("b")),
        )),
        opt(u16),
        opt(preceded(tag(":"), float)),
    ))(input)?;

    let pitch = match result.1.0.to_lowercase().as_str() {
        "c" => Pitch::C,
        "d" => Pitch::D,
        "e" => Pitch::E,
        "f" => Pitch::F,
        "g" => Pitch::G,
        "a" => Pitch::A,
        "b" => Pitch::B,
        _ => panic!("Unrecognized pitch")
    };

    let intonation = match result.1.1 {
        None => Intonation::None,
        Some(i) => match i {
            "##" => Intonation::DoubleSharp,
            "bb" => Intonation::DoubleFlat,
            "#" => Intonation::Sharp,
            "b" => Intonation::Flat,
            _ => panic!("Unrecognized intonation")
        }
    };

    let octave = result.1.2;
    
    let duration = result.1.3.unwrap_or(1.0f32);

    Ok((result.0, Note(pitch, intonation, octave, duration)))
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let result = delimited(tag("{"), many0(delimited(multispace0, parse_note, multispace0)), tag("}"))(input)?;
    Ok((result.0, Pattern {
        tempo: 60.0f32,
        base: 4,
        notes: result.1,
    }))
}

fn parse_declaration(input: &str) -> IResult<&str, Statement> {
    let result = separated_pair(
        preceded(
            tuple((tag("let"), multispace0)),
            alphanumeric1,
        ),
        tuple((multispace0, tag("="), multispace0)),
        parse_pattern
    )(input)?;

    Ok((result.0, Statement::Declaration {
        name: result.1.0.to_owned(),
        value: result.1.1
    }))
}

fn parse_function(input: &str) -> IResult<&str, Statement> {
    unimplemented!()
}

fn parse_function_matrix(input: &str) -> IResult<&str, String> {
    let result = delimited(
        tuple((tag("["), multispace0)),
        alphanumeric1,
        tuple((tag("]"), multispace0)),
    )(input)?;

    Ok((result.0, result.1.to_owned()))
}

pub fn parse_and_print() {
    let mut variables = HashMap::<String, Pattern>::new();
    let functions = HashMap::<String, Pattern>::new();
    let syn_file = std::fs::read_to_string("test/test.syn").unwrap();
    let statements = many0(alt((
        parse_single_line_comment,
        parse_multi_line_comment,
        parse_declaration,
        parse_blank_line,
    )))(&syn_file).unwrap();

    for statement in statements.1 {
        println!("{:?}", statement);
        match statement {
            Statement::Declaration { name, value } =>  {
                if !variables.contains_key(&name) {
                    variables.insert(name, value);
                } else {
                    println!("Variable already exists!");
                }
            }
            _ => ()
        };
    }

    println!("{:?}", variables);
}
