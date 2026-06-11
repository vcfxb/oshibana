//! Parser for scryfall-syntax queries

use nom::branch::alt;
use nom::bytes::{escaped_transform, tag, take_while, take_while1};
use nom::bytes::complete::{is_a, tag_no_case};
use nom::character::{complete::multispace0, none_of};
use nom::character::complete::{alpha1, char, multispace1, one_of};
use nom::combinator::{all_consuming, value};
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::Parser;
use nom::IResult;
use thiserror::Error;
use schemas::scryfall::card::languages::Language;

#[derive(Debug)]
pub enum Expr {
    /// A word or quoted string with no operators.
    Atom(String),

    /// An atom with a `!` indicating we want an exact match rather than fuzzy.
    Exact(String),

    /// A negated expression
    Negated(Box<Expr>),


    /// lang:...
    Language(Language),

    /// ... or ...
    Union(Box<Expr>, Box<Expr>),

    /// A series of expressions seperated by whitespace
    Intersection(Vec<Expr>),

    /// Expression in parentheses
    Group(Box<Expr>),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("parsing error: {0}")]
    Nom(#[from] nom::error::Error<String>),
    #[error("{0} is not a recognized language")]
    NoSuchLanguage(String),
}

impl nom::error::ParseError<String> for Error {
    fn from_error_kind(input: String, kind: ErrorKind) -> Self {
        Self::Nom(nom::error::Error::from_error_kind(input, kind))
    }

    fn append(_: String, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> nom::error::ParseError<&'a str> for Error {
    fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
        Self::Nom(nom::error::Error::from_error_kind(input.to_owned(), kind))
    }

    fn append(_: &'a str, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> From<nom::error::Error<&'a str>> for Error {
    fn from(value: nom::error::Error<&'a str>) -> Self {
        Self::Nom(value.into())
    }
}

impl Expr {
    pub fn parse(input: &'_ str) -> Result<Self, nom::Err<Error>> {
        // allow trailing whitespace
        let mut parser = all_consuming(
            terminated(parse_intersection, multispace0)
        );

        parser.parse_complete(input).map(|(_, expr)| expr)
    }
}

#[derive(Debug)]
enum DirectiveInput {
    Unquoted(String),
    Quoted(String),
    Regex(String),
}

impl DirectiveInput {
    /// If this isn't a regex, unqwap it to a string.
    fn unwrap_atom(self) -> String {
        use DirectiveInput::*;
        match self {
            Quoted(s) | Unquoted(s) => s,
            other => panic!("{other:?} is not an atom!"),
        }
    }
}


fn parse_quoted_string(input: &str) -> IResult<&str, DirectiveInput> {
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
    ).map(|s| DirectiveInput::Quoted(s));

    parser.parse_complete(input)
}

fn parse_atom(input: &str) -> IResult<&str, DirectiveInput> {
    alt((
        // unquoted string
        take_while1(|c: char| {
            !c.is_whitespace() && !"\"()!-".contains(c)
        }).map(|output: &str| DirectiveInput::Unquoted(output.to_string())),
        // quoted string
        parse_quoted_string,
    )).parse_complete(input)
}

/// Atoms can be exact -- e.g. "!Brainstorm or !Brainfreeze"
fn parse_optionally_exact_atom(input: &str) -> IResult<&str, Expr> {
    let mut parser = preceded(char('!'), parse_atom)
        .map(|di| Expr::Exact(di.unwrap_atom()));

    parser.parse_complete(input)
}

fn parse_optionally_negated(input: &str) -> IResult<&str, Expr, Error> {
    let mut parser = preceded(char('-'), parse_single_expr)
        .map(|expr| {
            Expr::Negated(Box::new(expr))
        });

    parser.parse_complete(input)
}

fn parse_group(input: &str) -> IResult<&str, Expr, Error> {
    let opens_with = pair(char('('), multispace0);
    let ends_with = pair(multispace0, char(')'));
    let mut parser = delimited(opens_with, parse_expr, ends_with);

    parser
        .map(|expr| {
            Expr::Group(Box::new(expr))
        })
        .parse_complete(input)
}

fn parse_union_expr(input: &str) -> IResult<&str, Expr, Error> {
    let seperator = delimited(multispace1, tag_no_case("or"), multispace1);
    let mut parser = separated_pair(parse_single_expr, seperator, parse_single_expr);
    parser
        .map(|(a, b)| Expr::Union(Box::new(a), Box::new(b)))
        .parse_complete(input)
}

/// Parse an expression, ignoring trailing whitespace.
fn parse_single_expr(input: &str) -> IResult<&str, Expr, Error> {
    // Order is important here to avoid ending up in infinite loop

    // first try parsing

    // first try parsing directives, so that we don't end up parsing something like "lang"
    // as an atom without getting the ":en" after it.

    // then try parsing



    parse_optionally_exact_atom(input).map_err(|err| err.map(Error::from))
}

fn directive_parser(
    aliases: &[&'static str],
    operators: &[&'static str],
    value_parsers: Vec<fn(&str) -> IResult<&str, String>>,
    value_handler: impl Fn(String) -> Result<Expr, Error>,
) -> impl Fn(&str) -> IResult<&str, Expr, Error> {
    move |input: &str| {
        let mut value_parsers = value_parsers.clone();
        let mut alias_parsers = aliases
            .into_iter()
            .map(|s| tag_no_case(*s))
            .collect::<Vec<_>>();

        let (input, _) = alt(&mut alias_parsers[..]).parse_complete(input)?;

        let mut operator_parsers = operators
            .into_iter()
            .map(|s| tag(*s))
            .collect::<Vec<_>>();

        let (input, _) = alt(&mut operator_parsers[..]).parse_complete(input)?;
        let (input, value) = alt(&mut value_parsers[..]).parse_complete(input)
            .map_err(|err| err.map(Error::from))?;

        let handled = value_handler(value)
            // call it a failure, if we parse the directive successfully but the value
            // is not legal.
            .map_err(|err| nom::Err::Failure(err))?;

        Ok((input, handled))
    }
}



fn parse_intersection(input: &str) -> IResult<&str, Expr, Error> {
    let mut parser = separated_list1(take_while1(char::is_whitespace), parse_single_expr);

    parser
        .map(|mut exprs| {
            if exprs.len() == 1 {
                exprs.pop().unwrap()
            } else {
                Expr::Intersection(exprs)
            }
        })
        .parse_complete(input)
}

fn language_directive(input: &str) -> IResult<&str, Language, Error> {
    let value_parser = alt((
        value(Language::En, alt((
            tag_no_case::<_, _, nom::error::Error<&str>>("en"),
            tag_no_case("eng"),
            tag_no_case("English"),
        ))),

        value(Language::Es, alt((
            tag_no_case("es"),
            tag_no_case("Spanish"),
        ))),

        value(Language::Fr, alt((
            tag_no_case("fr"),
            tag_no_case("french"),
        ))),

        value(Language::De, alt((
            tag_no_case("de"),
            tag_no_case("German"),
            tag_no_case("deutsch"),
        ))),

        value(Language::It, alt((
            tag_no_case("it"),
            tag_no_case("Italian"),
        ))),

        value(Language::Pt, alt((
            tag_no_case("pt"),
            tag_no_case("Portuguese"),
        ))),

        value(Language::Ja, alt((
            tag_no_case("ja"),
            tag_no_case("jp"),
            tag_no_case("Japanese"),
        ))),

        value(Language::Ko, alt((
            tag_no_case("ko"),
            tag_no_case("kr"),
            tag_no_case("Korean"),
        ))),

        value(Language::Ru, alt((
            tag_no_case("ru"),
            tag_no_case("Russian"),
        ))),

        value(Language::Zhs, alt((
            tag_no_case("zhs"),
        ))),

        value(Language::Zht, alt((
            tag_no_case("zht"),
        ))),

        value(Language::He, alt((
            tag_no_case("he"),
            tag_no_case("Hebrew"),
        ))),

        value(Language::La, alt((
            tag_no_case("la"),
            tag_no_case("Latin"),
        ))),

        value(Language::Grc, alt((
            tag_no_case("grc"),
        ))),

        value(Language::Ar, alt((
            tag_no_case("ar"),
            tag_no_case("Arabic"),
        ))),

        value(Language::Sa, alt((
            tag_no_case("sa"),
            tag_no_case("Sanskrit"),
        ))),

        value(Language::Ph, alt((
            tag_no_case("ph"),
            tag_no_case("Phyrexian"),
        ))),

        value(Language::Qya, alt((
            tag_no_case("qya"),
            tag_no_case("Quenya"),
        ))),
    )).parse_complete(input).map_err(|err| err.map(Error::from));
}

fn parse_expr

#[cfg(test)]
mod tests {
    use crate::scryfall::search::query_parser::parse_quoted_string;

    #[test]
    fn test_escaped_strings() {
        assert_eq!(parse_quoted_string(r#""simple""#), Ok(("", "simple".to_string())));
        assert_eq!(parse_quoted_string(r#""\"quoted\"""#), Ok(("", "\"quoted\"".to_string())));
    }
}
