use std::iter::Peekable;
use std::str::Chars;
use crate::scryfall::search::query_parser::fragment::Fragment;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'i> {
    frag: Fragment<'i>,
    kind: TokenTy,
}

pub struct Lexer<'i> {
    input: &'i str,
    chars: Peekable<Chars<'i>>,
    byte_index: usize,
    tokens: Vec<Token<'i>>,
}

impl<'i> Lexer<'i> {
    pub fn lex(input: &'i str) -> Result<Vec<Token<'i>>, &'static str> {
        let mut lexer = Lexer {
            input,
            chars: input.chars().peekable(),
            byte_index: 0,
            tokens: Vec::new(),
        };

        while let Some(peek) = lexer.chars.peek() {
            match *peek {
                ':' => {
                    lexer.chars.next();
                    lexer.push_token(1, TokenTy::Colon);
                }

                '=' => {
                    lexer.chars.next();
                    lexer.push_token(1, TokenTy::Eq);
                }

                '(' => {
                    lexer.chars.next();
                    lexer.push_token(1, TokenTy::LParen);
                }

                ')' => {
                    lexer.chars.next();
                    lexer.push_token(1, TokenTy::RParen);
                }

                '!' => {
                    lexer.chars.next();
                    lexer.push_token(1, TokenTy::Bang);
                }

                '>' => {
                    lexer.chars.next();
                    match lexer.chars.next_if_eq(&'=') {
                        Some(_) => lexer.push_token(2, TokenTy::GtEq),
                        None => lexer.push_token(1, TokenTy::Gt),
                    }
                }

                '<' => {
                    lexer.chars.next();
                    match lexer.chars.next_if_eq(&'=') {
                        Some(_) => lexer.push_token(2, TokenTy::LtEq),
                        None => lexer.push_token(1, TokenTy::Lt),
                    }
                }

                '"' => {
                    lexer.delimited('"', '\\', TokenTy::String, "unclosed double quote")?;
                }

                '\'' => {
                    lexer.delimited('\'', '\\', TokenTy::String, "unclosed single quote")?;
                }

                '/' => {
                    lexer.delimited('/', '\\', TokenTy::Regex, "unclosed regex")?;
                }

                other => {
                    let continue_text = |c: &char| {
                        !"()=!<>\"/:".contains(*c)
                    };

                    let mut consumed = 0;
                    while let Some(c) = lexer.chars.next_if(continue_text) {
                        consumed += c.len_utf8();
                    }

                    lexer.push_token(consumed, TokenTy::Text);
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
                full_query: self.input,
                byte_range: start..end,
            },
            kind,
        });
    }

    fn delimited(&mut self, delimiter: char, escape: char, kind: TokenTy, err: &'static str) -> Result<usize, &'static str> {
        // opening delimiter
        if self.chars.next_if_eq(&delimiter).is_none() { return Ok(0); }
        let mut consumed = 1;
        let mut escaped = false;

        loop {
            let next = self.chars.next();
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

#[cfg(test)]
mod tests {
    use crate::scryfall::search::query_parser::fragment::Fragment;
    use crate::scryfall::search::query_parser::lexer::{Lexer, Token, TokenTy};

    #[test]
    fn lex_plaintext() {
        let tokens = Lexer::lex("plaintext").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    frag: Fragment {
                        full_query: "plaintext",
                        byte_range: 0.."plaintext".len()
                    },
                    kind: TokenTy::Text
                }
            ]
        );
    }
}