use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::{Rule, operator::Operator, unwrap_exactly_one, Parser};
use pest::iterators::Pair;
use polars::prelude::Expr;
use schemas::scryfall::card::languages::Language;
use std::borrow::Cow;
use unescape_zero_copy::unescape_default;
use schemas::scryfall::card::colors::Color;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::lexer::TokenTy;
use crate::scryfall::search::query_parser::query::ParseError;

pub enum Filter<'i> {
    Lang {
        value: Language,
    },

    Name {
        operator: Operator,
        value: FilterValue<'i>,
    },

    Color {
        operator: Operator,
        value: FilterValue<'i>,
    },

    ColorIdentity {
        operator: Operator,
        value: FilterValue<'i>
    },

    Type {
        value: FilterValue<'i>,
    },

    OracleText {
        operator: Operator,
        value: FilterValue<'i>
    },

    Untagged {
        exact: bool,
        value: FilterValue<'i>,
    },

    Unknown {
        full_filter: Fragment<'i>,
        directive: Fragment<'i>,
        op: Operator,
        value: FilterValue<'i>,
    }
}

#[derive(Debug)]
pub enum FilterValue<'i> {
    Text(&'i str),
    String(Cow<'i, str>),
    Regex(&'i str),
}

impl<'i> Filter<'i> {
    pub fn parse(parser: &mut Parser<'i>) -> Result<Self, ParseError<'i>> {
        match parser.head_kind() {
            Some(TokenTy::Text) => {
                let operator = parser.peek()
                    .and_then(|t| Operator::parse(t).ok());

                if operator.is_none() {

                }

                let value = parser.peek_n(2);

                let possible_directive_str = parser.head().unwrap().as_str();

                match possible_directive_str.to_ascii_lowercase().as_str() {
                    "lang" | "language" =>
                }

            }
        }
    }

    pub(super) fn consume(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::filter => Self::consume(unwrap_exactly_one(pair)),

            Rule::lang_filter => Self::Lang {
                value: pair_to_language(unwrap_exactly_one(pair)),
            },

            Rule::name_filter => {
                let (operator, value) = pair_to_operator_value(pair);
                Self::Name { operator, value }
            }

            Rule::color_filter => {
                let (operator, value) = pair_to_operator_value(pair);
                Self::Color { operator, value }
            }

            Rule::color_identity_filter => {
                let (operator, value) = pair_to_operator_value(pair);
                Self::ColorIdentity { operator, value }
            }

            Rule::type_filter => Self::Type {
                value: FilterValue::consume(unwrap_exactly_one(pair)),
            },

            Rule::optionally_exact_filter_no_directive => {
                let is_exact = pair.as_str().starts_with('!');
                let value = FilterValue::consume(unwrap_exactly_one(pair));

                Self::Untagged {
                    exact: is_exact,
                    value,
                }
            }

            other => panic!("`{other:?}` does not match an implemented filter"),
        }
    }
}

impl<'i> MapToPolarsExpr for Filter<'i> {
    fn as_pexpr(&self) -> Expr {
        use self::Operator;
        use polars::prelude::*;

        match self {
            Filter::Lang { value } => {
                let lang_code = <Language as Into<&'static str>>::into(*value);
                col("lang").eq(lit(lang_code))
            }

            // ignore exactness for regexes
            Filter::Untagged {
                value: FilterValue::Regex(re),
                ..
            }
            | Filter::Name {
                value: FilterValue::Regex(re),
                ..
            } => col("name").str().contains(lit(*re), false),

            Filter::Name {
                operator: Operator::Colon,
                value,
            }
            | Filter::Untagged {
                exact: false,
                value,
            } => col("name")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(value.as_str().unwrap()).str().to_lowercase()),

            Filter::Name {
                operator: Operator::Eq,
                value,
            }
            | Filter::Untagged { exact: true, value } => {
                col("name")
                    .str()
                    .to_lowercase()
                    .eq(lit(value.as_str().unwrap()).str().to_lowercase())
            }

            Filter::Name { operator, .. } => panic!("unsupported name operator: {operator:?}"),

            Filter::Color { operator, value } => {

            },

            Filter::Type {
                value: FilterValue::Regex(re),
            } => col("type_line").str().contains(lit(*re), false),

            Filter::Type { value } => col("type_line")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(value.as_str().unwrap()).str().to_lowercase()),
        }
    }
}

impl<'i> FilterValue<'i> {
    pub fn parse(parser: &mut Parser<'i>) -> Result<Self, ()> {
        match parser.head_kind() {
            Some(TokenTy::Text) => Ok(Self::Text(parser.pull().as_str())),
            Some(TokenTy::String) => {
                let token_str = parser.pull().as_str();
                let inner = &token_str[1..token_str.len() - 1];
                let unescaped = unescape_default(inner).unwrap_or(Cow::Borrowed(inner));
                Ok(Self::String(unescaped))
            }
            Some(TokenTy::Regex) => Ok(Self::Regex(parser.pull().as_str())),
            _ => Err(())
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            FilterValue::Text(i) => Some(i),
            FilterValue::String(cow) => Some(cow.as_ref()),
            FilterValue::Regex(_) => None,
        }
    }
}

fn pair_to_operator_value(pair: Pair<Rule>) -> (Operator, FilterValue) {
    let mut inner = pair.into_inner();
    (
        Operator::consume(inner.next().unwrap()),
        FilterValue::consume(inner.next().unwrap())
    )
}

fn pair_to_language(pair: Pair<Rule>) -> Language {
    match pair.as_rule() {
        Rule::lang_value => pair_to_language(unwrap_exactly_one(pair)),
        Rule::lang_value_english => Language::En,
        Rule::lang_value_spanish => Language::Es,
        Rule::lang_value_french => Language::Fr,
        Rule::lang_value_german => Language::De,
        Rule::lang_value_italian => Language::It,
        Rule::lang_value_portuguese => Language::Pt,
        Rule::lang_value_japanese => Language::Ja,
        Rule::lang_value_korean => Language::Ko,
        Rule::lang_value_russian => Language::Ru,
        Rule::lang_value_zhs => Language::Zhs,
        Rule::lang_value_zht => Language::Zht,
        Rule::lang_value_hebrew => Language::He,
        Rule::lang_value_latin => Language::La,
        Rule::lang_value_grc => Language::Grc,
        Rule::lang_value_arabic => Language::Ar,
        Rule::lang_value_sanskrit => Language::Sa,
        Rule::lang_value_phyrexian => Language::Ph,
        Rule::lang_value_quenya => Language::Qya,
        Rule::lang_value_dwarvish => Language::Dw,
        other => panic!("{other:?} does not map to a language"),
    }
}

enum ColorValue {
    Multicolor,
    Colors(Vec<Color>),
}

fn filter_value_to_colors(value: FilterValue) -> ColorValue {
    let as_str = value.as_str().expect("color values should not include regexes");

}