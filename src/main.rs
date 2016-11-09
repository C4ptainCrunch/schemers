extern crate rustyline;

#[macro_use]
extern crate nom;

use rustyline::error::*;
use nom::IResult::{Done, Error, Incomplete};

named!(parse_symbol<char>, one_of!("!#$%&|*+-/:<=>?@^_~"));
named!(parse_string<Vec<char> >, delimited!(char!('"'), inside_string, char!('"')));

named!(inside_string<&[u8], std::vec::Vec<char> >, many0!(none_of!("\"")));


fn main() {
    let mut reader = rustyline::Editor::<()>::new();

    let mut i = 1;
    loop {
        let readline = reader.readline(&format!("In [{}]: ", i));
        match readline {
            Ok(line) => {
                if line == "" {
                    println!("");
                    continue;
                }
                match parse_string(line.as_bytes()) {
                    Done(_, matched) => println!("match: {:?}", matched),
                    Error(_) | Incomplete(_) => println!("error"),
                }
            }
            Err(ReadlineError::Interrupted) |
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        i += 1;
    }
}
