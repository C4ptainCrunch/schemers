extern crate rustyline;

#[macro_use]
extern crate nom;

use rustyline::error::*;
use nom::IResult::{Done, Error, Incomplete};

mod parsers;


fn eval(expression: parsers::LispVal) -> parsers::LispVal {
    expression
}

fn format_value(value: parsers::LispVal) -> String {
    match value {
        parsers::LispVal::Bool(val) => format!("Bool {:?}", val),
        parsers::LispVal::Atom(val) => format!("Atom: {:?}", val),
        parsers::LispVal::Number(val) => format!("Number: {:?}", val),
        parsers::LispVal::String(val) => format!("String {:?}", val),
        parsers::LispVal::List(val) => format!("List {:?}", val),
        _ => format!("Unknown value")
    }
}


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
                    Done(_, expression) => {
                        let result = eval(expression);
                        println!("{}", format_value(result));
                    },
                    Error(_) | Incomplete(_) => println!("SyntaxError"),
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
