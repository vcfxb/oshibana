use crate::scryfall::search::query_parser::fragment::Fragment;
use std::iter::Peekable;
use std::str::Chars;
use std::sync::Arc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenTy {
    LParen,
    RParen,
    String,
    Text,
    Colon,
    Eq,
    GtEq,
    LtEq,
    Gt,
    Lt,
    Regex,
    Or,
    And,
    Bang,
    BangEq,
    Whitespace,
    Neg,
}

pub struct Lexer {
    input: Arc<String>,
    byte_index: usize,
    tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub frag: Fragment,
    pub kind: TokenTy,
}

impl Lexer {
    fn chars(&self) -> Peekable<Chars<'_>> {
        self.input[self.byte_index..].chars().peekable()
    }

    pub fn lex(input: Arc<String>) -> Result<Vec<Token>, &'static str> {
        let mut lexer = Lexer {
            input,
            byte_index: 0,
            tokens: Vec::new(),
        };

        while let Some(peek) = lexer.chars().peek() {
            match *peek {
                ':' => {
                    lexer.push_token(1, TokenTy::Colon);
                }

                '=' => {
                    lexer.push_token(1, TokenTy::Eq);
                }

                '(' => {
                    lexer.push_token(1, TokenTy::LParen);
                }

                ')' => {
                    lexer.push_token(1, TokenTy::RParen);
                }

                '-' => {
                    lexer.push_token(1, TokenTy::Neg);
                }

                '!' => match lexer.chars().next_if_eq(&'=') {
                    Some(_) => lexer.push_token(2, TokenTy::BangEq),
                    None => lexer.push_token(1, TokenTy::Bang),
                },

                '>' => match lexer.chars().next_if_eq(&'=') {
                    Some(_) => lexer.push_token(2, TokenTy::GtEq),
                    None => lexer.push_token(1, TokenTy::Gt),
                },

                '<' => match lexer.chars().next_if_eq(&'=') {
                    Some(_) => lexer.push_token(2, TokenTy::LtEq),
                    None => lexer.push_token(1, TokenTy::Lt),
                },

                '"' => {
                    lexer.delimited('"', '\\', TokenTy::String, "unclosed double quote")?;
                }

                '\'' => {
                    lexer.delimited('\'', '\\', TokenTy::String, "unclosed single quote")?;
                }

                '/' => {
                    lexer.delimited('/', '\\', TokenTy::Regex, "unclosed regex")?;
                }

                c if c.is_whitespace() => {
                    let mut consumed = 0;
                    let mut chars = lexer.chars();

                    while let Some(c) = chars.next_if(|c| c.is_whitespace()) {
                        consumed += c.len_utf8();
                    }

                    lexer.push_token(consumed, TokenTy::Whitespace);
                }

                _ => {
                    let continue_text =
                        |c: &char| !"()=!<>\"\'/:-".contains(*c) && !c.is_whitespace();

                    let mut consumed = 0;
                    let mut chars = lexer.chars();
                    while let Some(c) = chars.next_if(continue_text) {
                        consumed += c.len_utf8();
                    }

                    match &lexer.input[lexer.byte_index..lexer.byte_index + consumed] {
                        "or" | "OR" => lexer.push_token(consumed, TokenTy::Or),
                        "and" | "AND" => lexer.push_token(consumed, TokenTy::And),
                        _ => lexer.push_token(consumed, TokenTy::Text),
                    }
                }
            }
        }

        Ok(lexer.tokens)
    }

    fn push_token(&mut self, bytes: usize, kind: TokenTy) {
        let start = self.byte_index;
        let end = self.byte_index + bytes;

        assert!(end <= self.input.len(), "end of token is out of bounds");

        self.byte_index += bytes;

        self.tokens.push(Token {
            frag: Fragment {
                full_query: Arc::clone(&self.input),
                byte_range: start..end,
            },
            kind,
        });
    }

    fn delimited(
        &mut self,
        delimiter: char,
        escape: char,
        kind: TokenTy,
        err: &'static str,
    ) -> Result<usize, &'static str> {
        // opening delimiter
        let mut chars = self.chars();
        if chars.next_if_eq(&delimiter).is_none() {
            return Ok(0);
        }
        let mut consumed = 1;
        let mut escaped = false;

        loop {
            let next = chars.next();
            next.inspect(|c| consumed += c.len_utf8());

            match next {
                Some(c) if c == delimiter && escaped => escaped = false,
                Some(c) if c == escape => escaped = !escaped,
                Some(c) if c == delimiter && !escaped => break,
                Some(_) => escaped = false,
                None => return Err(err),
            }
        }

        self.push_token(consumed, kind);
        Ok(consumed)
    }
}

impl Token {
    pub fn as_str(&self) -> &str {
        self.frag.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::scryfall::search::query_parser::fragment::Fragment;
    use crate::scryfall::search::query_parser::lexer::{Lexer, Token, TokenTy};
    use std::sync::Arc;

    #[test]
    fn lex_plaintext() {
        let input = Arc::new("plaintext".to_string());

        let tokens = Lexer::lex(input.clone()).unwrap();
        assert_eq!(
            tokens,
            vec![Token {
                frag: Fragment {
                    full_query: input,
                    byte_range: 0.."plaintext".len()
                },
                kind: TokenTy::Text
            }]
        );
    }
}
