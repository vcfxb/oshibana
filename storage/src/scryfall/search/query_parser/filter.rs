use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::{Rule, operator::Operator, unwrap_exactly_one};
use pest::iterators::Pair;
use polars::prelude::Expr;
use schemas::scryfall::card::languages::Language;
use std::borrow::Cow;
use unescape_zero_copy::unescape_default;
use schemas::scryfall::card::colors::Color;

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

    Untagged {
        exact: bool,
        value: FilterValue<'i>,
    },
}

pub enum FilterValue<'i> {
    Identifier(&'i str),
    String(Cow<'i, str>),
    Regex(&'i str),
}

impl<'i> Filter<'i> {
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
    fn consume(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::filter_value => Self::consume(unwrap_exactly_one(pair)),
            Rule::identifier => Self::Identifier(pair.as_str()),
            Rule::regex => Self::consume(unwrap_exactly_one(pair)),
            Rule::regex_inner => Self::Regex(pair.as_str()),
            Rule::string => Self::consume(unwrap_exactly_one(pair)),
            Rule::string_inner => {
                Self::String(unescape_default(pair.as_str()).unwrap_or(pair.as_str().into()))
            }
            Rule::color_filter_value => Self::consume(unwrap_exactly_one(pair)),
            _ => panic!("{pair:?} is not a filtervalue"),
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            FilterValue::Identifier(i) => Some(i),
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