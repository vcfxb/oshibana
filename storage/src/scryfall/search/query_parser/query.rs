use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::lexer::Lexer;
use crate::scryfall::search::query_parser::operator::Operator;
use crate::scryfall::search::query_parser::union::Union;


pub struct Query<'i> {
    pub union: Union<'i>
}

pub enum ParseError<'i> {
    LexerError(&'static str),
    UnrecognizedDirective(Fragment<'i>),
    IllegalOperator {
        directive: Fragment<'i>,
        operator: Operator
    }
}

pub struct ParseSuccess<'i> {
    pub query: Query<'i>,
    pub warnings: Vec<ParseError<'i>>
}

impl<'i> Query<'i> {
    pub fn parse(query: &'i str) -> Result<ParseSuccess<'i>, ParseError<'i>> {
        let tokens = Lexer::lex(query).map_err(ParseError::LexerError)?;
        
        
        
        unimplemented!()
    }
}
