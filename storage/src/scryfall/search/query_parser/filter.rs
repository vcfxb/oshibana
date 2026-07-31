use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::{operator::Operator, Diagnostic, Parser};
use polars::prelude::Expr;
use schemas::scryfall::card::languages::Language;
use std::borrow::Cow;
use unescape_zero_copy::unescape_default;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::lexer::{Token, TokenTy};

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
}

#[derive(Debug)]
pub enum FilterValue<'i> {
    Text(Fragment<'i>),
    String {
        fragment: Fragment<'i>,
        content: Cow<'i, str>,
    },
    Regex {
        fragment: Fragment<'i>,
        content: &'i str
    },
}

impl<'i> Filter<'i> {
    pub fn parse(parser: &mut Parser<'i>) -> Option<Self> {
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
                                message: format!("could not match {lang_str} to language"),
                                fragment: filter_value.fragment().clone(),
                            });

                            return None;
                        };

                        match operator {
                            Operator::Colon | Operator::Eq => Some(Filter::Lang { value: lang }),
                            _ => {
                                parser.diagnostics.push(Diagnostic::Warning {
                                    message: "language filter only supports `:`, `=` operators".into(),
                                    fragment: filter_value.fragment().clone(),
                                });

                                None
                            }
                        }
                    }

                    "name" => {
                        match (operator, &filter_value) {
                            (Operator::Gte | Operator::Lte | Operator::Gt | Operator::Lt, _) => {
                                parser.diagnostics.push(Diagnostic::Warning {
                                    message: "name only supports `:`, `=`, and `!=` options".to_string(),
                                    fragment: filter_value.fragment().clone(),
                                });

                                None
                            },

                            (_, FilterValue::Regex { .. }) if operator != Operator::Colon => {
                                parser.diagnostics.push(Diagnostic::Warning {
                                    message: "name regex filter only supports `:` operator".into(),
                                    fragment: filter_value.fragment().clone(),
                                });

                                None
                            }

                            _ => Some(Filter::Name { operator, value: filter_value }),
                        }
                    },

                    "c" | "color" => {
                        unimplemented!()
                    },

                    "ci" | "id" | "identity" => {
                        unimplemented!()
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

impl<'i> MapToPolarsExpr for Filter<'i> {
    fn as_pexpr(&self) -> Expr {
        use self::Operator;
        use polars::prelude::*;

        match self {
            Filter::Lang { value } => {
                let lang_code = <Language as Into<&'static str>>::into(*value);
                col("lang").eq(lit(lang_code))
            }

            Filter::Untagged {
                value: FilterValue::Regex { content, .. },
                ..
            }
            | Filter::Name {
                value: FilterValue::Regex { content, .. },
                operator: Operator::Colon,
            } => col("name").str().contains(lit(*content), false),

            Filter::Name {
                value: FilterValue::Regex { .. },
                operator
            } => panic!("regex doesn't support {operator:?}"),

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

            Filter::Name {
                operator: Operator::Neq,
                value: value @ (FilterValue::String { .. } |  FilterValue::Text(_))
            } => {
                col("name")
                    .str()
                    .to_lowercase()
                    .neq(lit(value.as_str().unwrap()).str().to_lowercase())
            }

            Filter::Name { operator, .. } => panic!("unsupported name operator: {operator:?}"),

            Filter::Color { .. } | Filter::ColorIdentity { .. } => todo!("color filtering"),

            Filter::Type {
                value: FilterValue::Regex { content, .. }
            } => col("type_line").str().contains(lit(*content), false),

            Filter::Type { value } => col("type_line")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(value.as_str().unwrap()).str().to_lowercase()),
                
            Filter::OracleText { .. } => todo!("oracle text filtering"),
        }
    }
}

impl<'i> FilterValue<'i> {
    pub fn parse(parser: &mut Parser<'i>) -> Option<Self> {
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
                    content: unescaped,
                })
            }
            TokenTy::Regex => {
                let token = parser.pull().unwrap();
                let token_str = token.as_str();
                Some(Self::Regex {
                    fragment: token.frag.clone(),
                    content: &token_str[1..token_str.len()-1],
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

    fn fragment(&self) -> &Fragment<'i> {
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
