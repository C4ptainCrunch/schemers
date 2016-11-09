extern crate rustyline;

use rustyline::error::*;


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
                println!("Out[{}]: {}\n",i, line);
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

