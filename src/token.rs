use std::fmt;
use strum_macros::EnumIter;

use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
}

pub const KEYWORD_MIN: i32 = 200;
pub const KEYWORD_MAX: i32 = 300;

#[derive(Copy, PartialEq, PartialOrd, Display, Clone, Debug, EnumIter, EnumString)]
pub enum TokenKind {
    Empty = -999,
    Eof = -1,
    Linefeed = 0,
    Space = 1,
    Carriagereturn = 2,
    Horizontaltab = 3,

    Rparen = 100,
    Lparen = 101,
    Opcurlybracket = 102,
    Clcurlybracket = 103,
    Opsqbracket = 104,
    Clsqbracket = 105,
    Colon = 106,
    Comma = 107,
    Decimalpoint = 108,

    True = 200,
    False = 201,
    Null = 202,

    Number = 301,
    String = 302,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}: {:?})", self.text, self.kind)
    }
}
