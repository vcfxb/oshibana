pub mod color;

use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::{operator::Operator, Diagnostic, Parser};
use polars::prelude::Expr;
use schemas::scryfall::card::languages::Language;
use std::borrow::Cow;
use unescape_zero_copy::unescape_default;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::lexer::{Token, TokenTy};

#[derive(Debug)]
pub enum Filter {
    Lang {
        operator: Operator,
        value: Language,
    },

    Name {
        operator: Operator,
        value: FilterValue,
    },

    CollectorNumber {
        operator: Operator,
        value: FilterValue,
    },

    Set {
        operator: Operator,
        value: FilterValue,
    },

    Color {
        operator: Operator,
        value: FilterValue,
    },

    ColorIdentity {
        operator: Operator,
        value: FilterValue
    },

    Type {
        value: FilterValue,
    },

    OracleText {
        operator: Operator,
        value: FilterValue
    },

    Untagged {
        exact: bool,
        value: FilterValue,
    },
}

#[derive(Debug)]
pub enum FilterValue {
    Text(Fragment),
    String {
        fragment: Fragment,
        content: String,
    },
    Regex {
        fragment: Fragment,
        content: String
    },
}

impl Filter {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        match parser.peek()?.clone() {
            Token { kind: TokenTy::Bang, frag: bang_frag } => {
                parser.pull();

                match FilterValue::parse(parser) {
                    None => {
                        parser.diagnostics.push(Diagnostic::Warning {
                            message: "`!` expects a filter value to follow it".to_string(),
                            fragment: bang_frag.clone(),
                        });

                        None
                    }

                    Some(fv) => Some(Self::Untagged { exact: true, value: fv })
                }
            }

            Token { kind: TokenTy::String | TokenTy::Regex, .. } => {
                Some(Self::Untagged {
                    exact: false,
                    value: FilterValue::parse(parser).expect("peeked string/regex")
                })
            }

            Token { kind: TokenTy::Text, frag: text_frag} => {
                parser.pull();

                let Some(operator) = parser.peek().and_then(Operator::parse) else {
                    return Some(Self::Untagged {
                        exact: false,
                        value: FilterValue::Text(text_frag.clone()),
                    });
                };

                parser.pull();
                let directive_frag = text_frag;
                let Some(filter_value) = FilterValue::parse(parser) else {
                    parser.diagnostics.push(Diagnostic::Warning {
                        message: "filter value expected after operator".to_string(),
                        fragment: directive_frag.clone(),
                    });

                    return None;
                };

                match directive_frag.as_str().to_ascii_lowercase().as_str() {
                    "lang" | "language" => {
                        let Some(lang_str) = filter_value.as_str() else {
                            parser.diagnostics.push(Diagnostic::Warning {
                                message: "languages cannot be regexes".to_string(),
                                fragment: filter_value.fragment().clone(),
                            });

                            return None;
                        };

                        let Some(lang) = str_to_language(lang_str) else {
                            parser.diagnostics.push(Diagnostic::Warning {
                                message: format!("could not match \"{lang_str}\" to language"),
                                fragment: filter_value.fragment().clone(),
                            });

                            return None;
                        };

                        match operator {
                            Operator::Colon | Operator::Eq | Operator::Neq => Some(Filter::Lang {
                                operator,
                                value: lang 
                            }),
                            
                            _ => {
                                parser.diagnostics.push(Diagnostic::Warning {
                                    message: "language filter only supports `:`, `=`, `!=` operators".into(),
                                    fragment: filter_value.fragment().clone(),
                                });

                                None
                            }
                        }
                    }

                    "name" => {
                        match (operator, &filter_value) {
                            (_, FilterValue::Regex { .. }) if operator != Operator::Colon => {
                                parser.diagnostics.push(Diagnostic::Warning {
                                    message: "name regex filter only supports `:` operator".into(),
                                    fragment: directive_frag.clone(),
                                });

                                None
                            }

                            _ => Some(Filter::Name { operator, value: filter_value }),
                        }
                    },

                    "cn" | "number" => match &filter_value {
                        FilterValue::Regex { .. } => {
                            parser.diagnostics.push(Diagnostic::Warning {
                                message: "collector number does not support regex".into(),
                                fragment: filter_value.fragment().clone(),
                            });

                            None
                        },

                        _ => {
                            let str_comparison = [
                                Operator::Eq,
                                Operator::Neq,
                                Operator::Colon
                            ].contains(&operator);

                            if !str_comparison &&
                                filter_value.as_str().unwrap().contains(|c| c > '9' || c < '0') {
                                parser.diagnostics.push(Diagnostic::Warning {
                                    message: "ordered comparison on collector numbers requires numeric value".into(),
                                    fragment: filter_value.fragment().clone(),
                                });
                            }

                            Some(Filter::CollectorNumber {
                                operator,
                                value: filter_value,
                            })
                        }
                    },

                    "s" | "set" | "edition" | "e" => match (operator, &filter_value) {
                        (_, FilterValue::Regex { .. }) => {
                            parser.diagnostics.push(Diagnostic::Warning {
                                message: "set/edition does not support regex".into(),
                                fragment: directive_frag.clone(),
                            });

                            None
                        }

                        (Operator::Neq | Operator::Eq | Operator::Colon, _) => Some(Filter::Set {
                            operator,
                            value: filter_value,
                        }),

                        _ => {
                            parser.diagnostics.push(Diagnostic::Warning {
                                message: "set/edition only supports '=', '!=', ':' operators".into(),
                                fragment: directive_frag.clone(),
                            });

                            None
                        }
                    },

                    "c" | "color" => {
                        parser.diagnostics.push(Diagnostic::Error {
                            message: format!("{} not yet implemented", directive_frag.as_str()),
                            fragment: Some(directive_frag.clone()),
                        });

                        None
                    },

                    "ci" | "id" | "identity" => {
                        parser.diagnostics.push(Diagnostic::Error {
                            message: format!("{} not yet implemented", directive_frag.as_str()),
                            fragment: Some(directive_frag.clone()),
                        });

                        None
                    },

                    "t" | "type" => match operator {
                        Operator::Colon => Some(Filter::Type {
                            value: filter_value,
                        }),

                        _ => {
                            parser.diagnostics.push(Diagnostic::Warning {
                                message: "type filter only supports `:` operator".into(),
                                fragment: directive_frag.clone(),
                            });

                            None
                        }
                    },

                    "o" | "oracle" => match operator {
                        Operator::Colon => Some(Filter::OracleText { operator, value: filter_value }),
                        _ => {
                            parser.diagnostics.push(Diagnostic::Warning {
                                message: "oracle text filter only supports `:` operator".into(),
                                fragment: directive_frag.clone(),
                            });

                            None
                        }
                    },

                    other => {
                        parser.diagnostics.push(Diagnostic::Error {
                            message: format!("unrecognized filter: {other}"),
                            fragment: Some(directive_frag.clone()),
                        });

                        None
                    },
                }
            }

            _ => None,
        }
    }
}

impl MapToPolarsExpr for Filter {
    fn as_pexpr(&self) -> Expr {
        use self::Operator;
        use polars::prelude::*;

        match self {
            Filter::Lang { operator, value } => {
                let lang_code = <Language as Into<&'static str>>::into(*value);
                let op_fn = operator.polars_fn();
                op_fn(col("lang"), lit(lang_code))
            }

            // card names get some special handling cause they can be untagged
            
            Filter::Untagged {
                value: FilterValue::Regex { content, .. },
                ..
            }
            | Filter::Name {
                value: FilterValue::Regex { content, .. },
                operator: Operator::Colon,
            } => col("name").str().contains(lit(content.as_str()), false),

            Filter::Name {
                value: FilterValue::Regex { .. },
                operator
            } => panic!("name regex doesn't support {operator:?}"),

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

            Filter::Name { operator, value } => {
                let op_fn = operator.polars_fn();
                let lhs = col("name").str().to_lowercase();
                let rhs = lit(value.as_str().unwrap()).str().to_lowercase();
                op_fn(lhs, rhs)
            },

            Filter::CollectorNumber { operator, value } => {
                let op_fn = operator.polars_fn();

                match operator {
                    Operator::Colon | Operator::Neq | Operator::Eq => {
                        let lhs = col("collector_number")
                            .str()
                            .to_lowercase();

                        let rhs = lit(value.as_str().unwrap()).str().to_lowercase();

                        op_fn(lhs, rhs)
                    }

                    _ => {
                        let lhs = col("collector_number")
                            .cast(DataType::Int32);

                        let rhs = lit(value.as_str().unwrap()).cast(DataType::Int32);
                        op_fn(lhs, rhs)
                    }
                }
            }

            Filter::Set { operator, value } => {
                let op_fn = operator.polars_fn();
                let value = lit(value.as_str().unwrap()).str().to_lowercase();

                Expr::or(
                    op_fn(col("set").str().to_lowercase(), value.clone()),
                    op_fn(col("set_name").str().to_lowercase(), value),
                )
            }

            Filter::Color { .. } | Filter::ColorIdentity { .. } => todo!("color filtering"),

            Filter::Type {
                value: FilterValue::Regex { content, .. }
            } => col("type_line").str().contains(lit(content.as_str()), false),

            Filter::Type { value } => col("type_line")
                .str()
                .to_lowercase()
                .str()
                .split_regex(lit(r"\s+"), true)
                .list()
                .contains(lit(value.as_str().unwrap()).str().to_lowercase(), false),

            Filter::OracleText {
                operator: Operator::Colon,
                value: FilterValue::Regex { content, .. }
            } => col("oracle_text")
                .str()
                .contains(lit(content.as_str()), false),
            
            Filter::OracleText {
                operator: Operator::Colon,
                value
            } => col("oracle_text")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(value.as_str().unwrap()).str().to_lowercase()),

            Filter::OracleText { operator, .. } => {
                panic!("unsupported oracle text operator: {operator:?}")
            },
        }
    }
}

impl FilterValue {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        let token = parser.peek()?;
        match token.kind {
            TokenTy::Text => {
                let token = parser.pull().unwrap();
                Some(Self::Text(token.frag.clone()))
            }
            TokenTy::String => {
                let token = parser.pull().unwrap();
                let token_str = token.as_str();
                let inner = &token_str[1..token_str.len() - 1];
                let unescaped = unescape_default(inner).unwrap_or(Cow::Borrowed(inner));
                Some(Self::String {
                    fragment: token.frag.clone(),
                    content: unescaped.to_string(),
                })
            }
            TokenTy::Regex => {
                let token = parser.pull().unwrap();
                let token_str = token.as_str();
                Some(Self::Regex {
                    fragment: token.frag.clone(),
                    content: token_str[1..token_str.len()-1].to_string(),
                })
            }
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            FilterValue::Text(frag) => Some(frag.as_str()),
            FilterValue::String { content, .. } => Some(content.as_ref()),
            FilterValue::Regex { .. } => None,
        }
    }

    fn fragment(&self) -> &Fragment {
        match self {
            FilterValue::Text(f) => f,
            FilterValue::String { fragment, .. } => fragment,
            FilterValue::Regex { fragment, .. } => fragment
        }
    }
}

fn str_to_language(s: &str) -> Option<Language> {
    match s.to_ascii_lowercase().as_str() {
        "english" | "en" => Some(Language::En),
        "spanish" | "es" | "sp" => Some(Language::Es),
        "french" | "fr" => Some(Language::Fr),
        "german" | "de" => Some(Language::De),
        "italian" | "it" => Some(Language::It),
        "portuguese" | "pt" => Some(Language::Pt),
        "japanese" | "ja" | "jp" => Some(Language::Ja),
        "korean" | "ko" | "kr" => Some(Language::Ko),
        "russian" | "ru" => Some(Language::Ru),
        "simplified chinese" | "zhs" | "cs" => Some(Language::Zhs),
        "traditional chinese" | "zht" | "ct" => Some(Language::Zht),
        "hebrew" | "he" => Some(Language::He),
        "latin" | "la" => Some(Language::La),
        "ancient greek" | "grc" => Some(Language::Grc),
        "arabic" | "ar" => Some(Language::Ar),
        "sanskrit" | "sa" => Some(Language::Sa),
        "phyrexian" | "ph" => Some(Language::Ph),
        "quenya" | "qya" => Some(Language::Qya),
        "dwarvish" | "dw" => Some(Language::Dw),
        _ => None,
    }
}
