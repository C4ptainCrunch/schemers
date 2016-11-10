extern crate std;
use nom::{alpha, digit};

named!(
    symbol<char>,
    one_of!("!#$%&|*+-/:<=>?@^_~")
);

named!(
    inside_string<&[u8], std::vec::Vec<char> >,
    many0!(none_of!("\""))
);

named!(
    pub string<Vec<char> >,
    delimited!(
        char!('"'),
        inside_string,
        char!('"')
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
//         parse_atom_start ~ many0!(parse_atom_rest)
//     )
// );
