//! Parser for scryfall-syntax queries

use nom::Parser;
use nom::{IResult};
use nom::branch::alt;
use nom::bytes::{escaped_transform, is_not, tag, take_while1};
use nom::character::complete::alphanumeric1;
use nom::character::none_of;
use nom::combinator::value;
use nom::sequence::delimited;

pub enum Expr {
    /// A word or quoted string with no operators.
    Atom(String),

    /// A series of expressions seperated by whitespace
    Intersection(Vec<Expr>),

    // Group(Box<Expr>),
    // Negated(Box<Expr>),

}

pub enum Error<'a> {
    Nom(nom::error::Error<&'a str>)
}

impl Expr {
    pub fn parse(input: &'_ str) -> IResult<&'_ str, String, Error<'_>> {
        unimplemented!()
    }
}

fn parse_string(input: &str) -> IResult<&str, String> {
    let transform = escaped_transform(
        none_of("\\\""),
        '\\',
        alt((
            value("\n", tag("n")),
            value("\t", tag("t")),
            value("\\", tag("\\")),
            value("\"", tag("\"")),
            value("\'", tag("\'")),
        ))
    );

    let mut parser = delimited(
        tag("\""),
        transform,
        tag("\""),
    );

    parser.parse_complete(input)
}

fn parse_atom(input: &str) -> IResult<&str, String> {
    alt((
        take_while1(|c: char| {
            !c.is_whitespace() && !"\"()".contains(c)
        }).map(|output: &str| output.to_string()),
    )).parse_complete(input)
}

#[cfg(test)]
mod tests {
    use crate::scryfall::search::query_parser::parse_string;

    #[test]
    fn test_escaped_strings() {
        assert_eq!(parse_string(r#""simple""#), Ok(("", "simple".to_string())));
        assert_eq!(parse_string(r#""\"quoted\"""#), Ok(("", "\"quoted\"".to_string())));
    }
}
