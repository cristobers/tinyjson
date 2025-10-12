use std::process;

use crate::token::KEYWORD_MAX;
use crate::token::KEYWORD_MIN;
use crate::token::Token;
use crate::token::TokenKind;

use strum::IntoEnumIterator;

pub struct Lexer {
    pub source: String,
    pub cur_char: char,
    pub cur_pos: i32,
    pub newline_count: u64,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut temp_lexer = Lexer {
            source,
            cur_char: ' ',
            cur_pos: -1,
            newline_count: 0,
        };
        temp_lexer.source.push('\n');
        // Initalise cur_char
        temp_lexer.next_char();
        temp_lexer
    }

    /// Get the next character in the source.
    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        if self.cur_pos as usize >= self.source.len() {
            // We've gone further than the source input, we have reached EOF.
            self.cur_char = '\0';
        } else {
            // Otherwise, set self.cur_char to (essentially) self.source[self.cur_char].
            self.cur_char = self
                .source
                .chars()
                .nth(self.cur_pos as usize)
                .expect(&format!(
                    "Failed to get character at position {}, OOB.",
                    self.cur_pos
                ));
        }
    }

    fn parse_error(&mut self, output_msg: &str) {
        eprintln!("{}", output_msg);
        std::process::exit(-1);
    }

    /// peek :: String -> Maybe char
    pub fn peek(&mut self) -> Option<char> {
        if (self.cur_pos as usize) + 1 >= self.source.len() {
            return None;
        }
        Some(
            self.source
                .chars()
                .nth((self.cur_pos as usize) + 1)
                .expect("OOB"),
        )
    }

    /// Capitalize the first letter of the given String.
    fn capitalise(str: String) -> String {
        let first: String = str.chars().nth(0).unwrap().to_uppercase().to_string();

        let snd: &String = &str.get(1..).unwrap().to_string();

        let mut f = String::from(first);
        f.push_str(snd);
        f
    }

    fn is_keyword(&mut self, token_text: &String) -> Option<Token> {
        let capitalised_text = Lexer::capitalise(token_text.clone());
        for token_kind in TokenKind::iter() {
            // Check and see if our token_text is equal to a TokenKind enum.
            let as_string: String = Lexer::capitalise(token_kind.to_string());
            if as_string == capitalised_text {
                // When its an enum, it should be in the range for a keyword also.
                let token_as_i32 = token_kind.clone() as i32;
                if token_as_i32 >= KEYWORD_MIN && token_as_i32 < KEYWORD_MAX {
                    return Some(Token {
                        text: token_text.to_string(),
                        kind: token_kind,
                    });
                }
            }
        }
        None
    }

    fn valid_fp_number(&mut self, potential: &str) -> bool {
        // Do we have more than one decimal point in our number? if so, this is bad.
        let point_count: usize = potential
            .chars()
            .filter(|x| *x == '.')
            .collect::<Vec<char>>()
            .len();
        if point_count > 1 {
            return false;
        }
        // TODO: Should check if the decimal point has atleast one number ahead of it,
        //       or at least one number behind it.

        // We have one decimal point now
        let point_pos: usize = potential.find('.').unwrap();
        if point_pos < 1 || point_pos == potential.len() - 1 {
            return false;
        }
        true
    }

    pub fn match_token(&mut self) -> Token {
        let token = match self.cur_char {
            ' ' => Token {
                text: String::from(' '),
                kind: TokenKind::Space,
            },
            '\n' => {
                // Increment out newline count, for showing better error messages
                self.newline_count += 1;
                Token {
                    text: String::from('\n'),
                    kind: TokenKind::Linefeed,
                }
            },
            '\r' => Token {
                text: String::from('\r'),
                kind: TokenKind::Carriagereturn,
            },
            '\t' => Token {
                text: String::from('\t'),
                kind: TokenKind::Horizontaltab,
            },
            '\0' => Token {
                text: String::from('\0'),
                kind: TokenKind::Eof,
            },
            '[' => Token {
                text: String::from('['),
                kind: TokenKind::Opsqbracket,
            },
            ']' => Token {
                text: String::from(']'),
                kind: TokenKind::Clsqbracket,
            },
            ',' => Token {
                text: String::from(','),
                kind: TokenKind::Comma,
            },
            '{' => Token {
                text: String::from('{'),
                kind: TokenKind::Opcurlybracket,
            },
            '}' => Token {
                text: String::from('}'),
                kind: TokenKind::Clcurlybracket,
            },
            ':' => Token {
                text: String::from(':'),
                kind: TokenKind::Colon,
            },
            '.' => {
                if let Some(c) = self.peek() {
                    if c.is_numeric() {
                        self.parse_error(&format!("Malformed floating point number at line number: {}", self.newline_count));
                    }
                    Token {
                        text: String::from('.'),
                        kind: TokenKind::Decimalpoint,
                    }
                } else {
                    self.parse_error("Failed to peek after \".\"");
                    // Literally doesnt matter, we've exited by now
                    Token { text: String::new(), kind: TokenKind::Empty }
                }
            }
            '\"' => {
                // Goto the next char, ignore the quote at the start of the String.
                self.next_char();
                // This is where the string starts.
                let start_pos = self.cur_pos as usize;
                // While we havent hit the ending quote.
                while self.cur_char != '\"' {
                    // If we end up never finding another quote, throw an error and quit.
                    if self.cur_pos as usize > self.source.len() {
                        self.parse_error(&format!("Missing quotation mark for string at line number: {}", self.newline_count));
                    }
                    self.next_char();
                }
                let end_pos = self.cur_pos as usize;
                // Construct the string from the start and end pos.
                let constructed: String = self
                    .source
                    .get(start_pos..end_pos)
                    .expect("Failed to construct a string, range led to OOB.")
                    .to_owned();
                Token {
                    text: constructed.to_string(),
                    kind: TokenKind::String,
                }
            }
            _ => {
                // TODO: Floating point numbers
                // If we're currently numeric.
                if self.cur_char.is_numeric() {
                    let start_pos = self.cur_pos as usize;
                    while let Some(c) = self.peek() {
                        if c == '.' || c.is_numeric() {
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    let end_pos = self.cur_pos as usize + 1;
                    let constructed: String = self
                        .source
                        .get(start_pos..end_pos)
                        .expect("Failed to construct a number, range led to OOB.")
                        .to_owned();
                    // TODO:
                    // If we have a decimalpoint in our floating point number,
                    // make sure that it follows the right rules.
                    if constructed.contains('.') {
                        if !self.valid_fp_number(&constructed) {
                            self.parse_error("Invalid Floating point number.");
                        }
                    }
                    Token {
                        text: constructed,
                        kind: TokenKind::Number,
                    }
                } else if self.cur_char.is_alphabetic() {
                    // We could get a `true`, `false`, or `,null` keyword.
                    let start_pos = self.cur_pos as usize;
                    while let Some(c) = self.peek()
                        && c.is_alphabetic()
                    {
                        self.next_char();
                    }
                    let end_pos = self.cur_pos as usize + 1;
                    let constructed: String = self
                        .source
                        .get(start_pos..end_pos)
                        .expect("Failed to construct a keyword, range led to OOB.")
                        .to_owned();
                    match self.is_keyword(&constructed) {
                        Some(t) => t,
                        _ => {
                            self.parse_error(&format!("Unknown keyword: {}", &constructed));
                            Token { text: String::new(), kind: TokenKind::Empty }
                        }
                    }
                } else {
                    self.parse_error(&format!("Unknown character: {}", self.cur_char));
                    Token { text: String::new(), kind: TokenKind::Empty }
                }
            }
        };
        self.next_char();
        token
    }
}
