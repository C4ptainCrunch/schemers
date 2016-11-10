extern crate std;

use std::str;

enum LispVal {
    Atom(String),
    List(Box<Vec<LispVal>>),
    DottedList(Box<Vec<LispVal>>, Box<LispVal>),
    Number(i32),
    String(String),
    Bool(bool),
}

named!(
    symbol<char>,
    one_of!("!#$%&|*+-/:<=>?@^_~")
);

named!(
    alpha<char>,
    one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
);

named!(
    digit<char>,
    one_of!("0123456789")
);

named!(
    pub string<&str>,
    map_res!(
        delimited!(
            char!('\"'),
            is_not!("\""),
            char!('\"')
        ),
        str::from_utf8
    )
);

named!(
    atom_start<char>,
    alt!(
        alpha  |
        symbol
    )
);

named!(
    atom_rest<char>,
    alt!(
        alpha | digit | symbol
    )
);

named!(
    pub atom<String>,
    chain!(
        start: atom_start ~ rest: many0!(atom_rest),
        || {
            let mut atom_content = String::new();
            atom_content.push(start);
            for c in rest {
                atom_content.push(c);
            }
            // LispVal::Atom(atom_content)
            atom_content
        }

    )
);
