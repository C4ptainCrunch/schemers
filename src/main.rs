extern crate rustyline;

#[macro_use]
extern crate nom;

use rustyline::error::*;
use nom::IResult::{Done, Error, Incomplete};

mod parsers;


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
                match parsers::command(line.trim().as_bytes()) {
                    Done(_, matched) => {
                        match matched {
                            parsers::LispVal::Bool(val) => println!("Bool {:?}", val),
                            parsers::LispVal::Atom(val) => println!("Atom: {:?}", val),
                            parsers::LispVal::Number(val) => println!("Number: {:?}", val),
                            parsers::LispVal::String(val) => println!("String {:?}", val),
                            parsers::LispVal::List(val) => println!("List {:?}", val),
                            _ => println!("WAT ?")
                        }
                    },
                    Error(_) | Incomplete(_) => println!("error"),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        i += 1;
    }
}
