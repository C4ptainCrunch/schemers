extern crate rustyline;

#[macro_use]
extern crate nom;

use rustyline::error::*;
use nom::IResult::{Done, Error, Incomplete};

named!(parseSymbol<char>, one_of!("!#$%&|*+-/:<=>?@^_~"));
// named!(parseString<&[u8], Vec<&[u8]> >, delimited!(char!('"'), many0!(not!(char!('"'))), char!('"')));
named!(parseString, delimited!(char!('"'), inside_string, char!('"')));

named!(inside_string<&[u8], std::vec::Vec<&[u8]> >, many0!(not!(char!('"'))));


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
                match parseString(line.as_bytes()) {
                    Done(_, matched) => println!("match"),
                    Error(_) | Incomplete(_) => println!("error"),
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
        i += 1;
    }
}

