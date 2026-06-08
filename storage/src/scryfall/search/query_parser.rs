//! Parser for scryfall-syntax queries

use nom::branch::alt;
use nom::bytes::{escaped_transform, tag, take_while1};
use nom::character::{complete::multispace0, none_of};
use nom::combinator::{all_consuming, value};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, terminated};
use nom::Parser;
use nom::IResult;
use thiserror::Error;

#[derive(Debug)]
pub enum Expr {
    /// A word or quoted string with no operators.
    Atom(String),

    /// A series of expressions seperated by whitespace
    Intersection(Vec<Expr>),

    /// A negated expression
    Negated(Box<Expr>),

    /// An atom with a `!` indicating we want an exact match rather than fuzzy.
    Exact(String),

    // Group(Box<Expr>),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("parsing error: {0}")]
    Nom(#[from] nom::Err<nom::error::Error<String>>)
}

impl Expr {
    pub fn parse(input: &'_ str) -> Result<Self, Error> {
        // allow trailing whitespace
        let mut parser = all_consuming(
            terminated(parse_intersection, multispace0)
        );

        parser.parse_complete(input)
            .map(|(_, expr)| expr)
            .map_err(|err| Error::Nom(err.to_owned()))
    }
}

fn parse_quoted_string(input: &str) -> IResult<&str, String> {
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
        // unquoted string
        take_while1(|c: char| {
            !c.is_whitespace() && !"\"()!-".contains(c)
        }).map(|output: &str| output.to_string()),
        // quoted string
        parse_quoted_string,
    )).parse_complete(input)
}

fn parse_intersection(input: &str) -> IResult<&str, Expr> {
    let parse_option = alt((
        preceded(tag("-"), parse_atom).map(|string| {
            Expr::Negated(Box::new(Expr::Atom(string)))
        }),
        preceded(tag("!"), parse_atom).map(Expr::Exact),
        parse_atom.map(Expr::Atom),
    ));

    let compound = separated_list1(
        take_while1(char::is_whitespace),
        parse_option
    );

    compound
        .map(|mut exprs| {
            if exprs.len() == 1 {
                exprs.pop().unwrap()
            } else {
                Expr::Intersection(exprs)
            }
        })
        .parse_complete(input)
}

#[cfg(test)]
mod tests {
    use crate::scryfall::search::query_parser::parse_quoted_string;

    #[test]
    fn test_escaped_strings() {
        assert_eq!(parse_quoted_string(r#""simple""#), Ok(("", "simple".to_string())));
        assert_eq!(parse_quoted_string(r#""\"quoted\"""#), Ok(("", "\"quoted\"".to_string())));
    }
}
