extern crate std;

use std::str;
use nom::{alpha, digit};

named!(
    symbol<char>,
    one_of!("!#$%&|*+-/:<=>?@^_~")
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

// named!(
//     atom_start,
//     alt!(
//         alpha | parse_symbol
//     )
// );

// named!(
//     atom_rest,
//     alt!(
//         alpha | digit | parse_symbol
//     )
// );

// named!(
//     pub atom,
//     chain!(
//         start: parse_atom_start ~ rest: many0!(parse_atom_rest),
//         || {rest.predend(start)}
//     )
// );
