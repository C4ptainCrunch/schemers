extern crate std;

use std::str;
use nom::digit;

pub enum LispVal {
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
    my_digit<char>,
    one_of!("0123456789")
);

named!(
    _string<String>,
    map!(
        map_res!(
            delimited!(
                char!('\"'),
                is_not!("\""),
                char!('\"')
            ),
            str::from_utf8
        ),
        String::from
    )
);

named!(string<LispVal>,
    map!(
        _string,
        LispVal::String
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
        alpha | my_digit | symbol
    )
);

named!(
    atom<LispVal>,
    chain!(
        start: atom_start ~ rest: many0!(atom_rest),
        || {
            let mut atom_content = String::new();
            atom_content.push(start);
            for c in rest {
                atom_content.push(c);
            }
            match atom_content.as_ref() {
                "#t" => return LispVal::Bool(true),
                "#f" => return LispVal::Bool(false),
                _ => return LispVal::Atom(atom_content),
            }
        }

    )
);

named!(_number<i32>,
    map_res!(
        map_res!(
            digit,
            std::str::from_utf8
        ),
        std::str::FromStr::from_str
    )
);

named!(number<LispVal>,
    map!(
        _number,
        LispVal::Number
    )
);

named!(
    pub expression<LispVal>,
    alt!(
        number | atom | string
    )
);
