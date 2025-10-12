
/*
    TODO:
    - Write tests that WILL fail alongside those that WILL succeed.
    - Have parse() return a Maybe Int, wherein, if we parsed correctly then we're None,
      otherwise we are a Some(-1), which we unwrap and return.
    - There is an error with non ascii characters when trying to construct a string.
*/


use crate::lexer;
use crate::token::Token;
use crate::token::TokenKind;

pub struct Parser<'a> {
    lexer: &'a mut lexer::Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser<'_> {
    /// Initialise a new parser.
    pub fn new(lexer: &mut lexer::Lexer) -> Parser<'_> {
        let mut temp_parser = Parser {
            lexer: lexer,
            cur_token: Token {
                text: String::new(),
                kind: TokenKind::Empty,
            },
            peek_token: Token {
                text: String::new(),
                kind: TokenKind::Empty,
            },
        };
        // Fill out out cur_token and peek_token by calling next_token().
        temp_parser.next_token();
        temp_parser.next_token();
        temp_parser
    }

    pub fn parse(&mut self) -> i32 {
        match self.cur_token.kind {
            TokenKind::Opcurlybracket => self.object(),
            TokenKind::Opsqbracket => self.array(),
            // Every other case is just checked by the lexer (???)
            _ => (),
        }
        0
    }

    fn fail_to_parse(&mut self, output_msg: &str) {
        eprintln!("Parse error: {}", output_msg);
        std::process::exit(-1);
    }

    fn finish(&mut self) {
        println!("JSON parsed successfully.");
    }

    fn object(&mut self) {
        self.token_match(TokenKind::Opcurlybracket);
        self.whitespace();
        if self.check_token(TokenKind::Clcurlybracket) {
            return;
        } else if self.string() {
            self.next_token();
            self.whitespace();
            self.token_match(TokenKind::Colon);
            self.value();
            if self.check_token(TokenKind::Comma) {
                //println!("COMMA");
                loop {
                    if self.check_token(TokenKind::Clcurlybracket) {
                        self.finish();
                        return;
                    }
                    self.next_token();
                    self.whitespace();
                    if !self.string() {
                        dbg!(self.cur_token.kind);
                        if self.check_token(TokenKind::Clcurlybracket) {
                            self.fail_to_parse("Trailing comma found at end of object.");
                        }
                        self.fail_to_parse("Expected a string.");
                    }
                    self.next_token();
                    self.whitespace();
                    self.token_match(TokenKind::Colon);
                    self.value();
                }
            } else {
                self.token_match(TokenKind::Clcurlybracket);
                self.finish();
                return;
            }
        }
        self.fail_to_parse("Expected a string.");
    }

    fn string(&mut self) -> bool {
        self.check_token(TokenKind::String)
    }

    fn value(&mut self) {
        self.whitespace();

        match self.cur_token.kind {
            TokenKind::String => (),
            TokenKind::Number => (),
            TokenKind::Opcurlybracket => self.object(),
            TokenKind::Opsqbracket => self.array(),
            TokenKind::True => (),
            TokenKind::False => (),
            TokenKind::Null => (),
            _               => self.fail_to_parse("Expected a string."),
        }
        self.next_token();

        // Whitespace
        self.whitespace();
    }

    fn array(&mut self) {
        self.token_match(TokenKind::Opsqbracket);
        if self.is_whitespace() && self.check_peek(TokenKind::Clsqbracket) {
            // Empty array, but whitespace(s) inside.
            self.whitespace();
            self.token_match(TokenKind::Clsqbracket);
        } else if self.check_token(TokenKind::Clsqbracket) {
            // We're good here, just an empty array.
            return;
        } else {
            // Array with values inside, loop so long as we have values.
            loop {
                dbg!(self.cur_token.kind);
                self.value();
                dbg!(self.cur_token.kind);
                let end_of_array =
                    !self.check_token(TokenKind::Comma) && self.check_token(TokenKind::Clsqbracket);
                if end_of_array {
                    break;
                } else {
                    self.token_match(TokenKind::Comma);
                }
            }
        }
    }

    fn whitespace(&mut self) {
        while self.cur_token.kind == TokenKind::Space || self.cur_token.kind == TokenKind::Linefeed
        {
            self.next_token();
        }
    }

    /// For when we have to conditionally check for a whitespace.
    fn is_whitespace(&mut self) -> bool {
        match self.cur_token.kind {
            TokenKind::Space
            | TokenKind::Linefeed
            | TokenKind::Horizontaltab
            | TokenKind::Carriagereturn => true,
            _ => false,
        }
    }

    /// See if our current token is one that we expect.
    pub fn check_token(&mut self, token_kind: TokenKind) -> bool {
        if self.cur_token.kind != token_kind {
            return false;
        }
        true
    }

    /// See if our peek token is one that we expect.
    pub fn check_peek(&mut self, token_kind: TokenKind) -> bool {
        if self.peek_token.kind != token_kind {
            return false;
        }
        true
    }

    pub fn token_match(&mut self, token_kind: TokenKind) {
        //println!("{}", token_kind);
        match self.check_token(token_kind) {
            true => self.next_token(),
            false => self.fail_to_parse("Expected a string."),
        };
    }

    /// Updates cur_token and peek_token.
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.match_token();
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexer;
    use crate::Parser;

    #[test]
    fn empty_object() {
        let mut lexer: Lexer = Lexer::new(String::from("{ }"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn empty_object_again() {
        let mut lexer: Lexer = Lexer::new(String::from("{}"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn fp_number() {
        let mut lexer: Lexer = Lexer::new(String::from("3.14159"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn one_value_object() {
        let mut lexer: Lexer = Lexer::new(String::from("{ \"name\": \"bob\" }"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn two_values_object() {
        let mut lexer: Lexer = Lexer::new(String::from("{ \"name\": \"bob\", \"alive\": true }"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn number() {
        let mut lexer: Lexer = Lexer::new(String::from("1"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn string() {
        let mut lexer: Lexer = Lexer::new(String::from("\"foo\""));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn long_number() {
        let mut lexer: Lexer = Lexer::new(String::from("1223334444555556666667777777888888889999999990"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn value() {
        let mut lexer: Lexer = Lexer::new(String::from("true"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn empty_array() {
        let mut lexer: Lexer = Lexer::new(String::from("[ ]"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn empty_array_again() {
        let mut lexer: Lexer = Lexer::new(String::from("[]"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn arrays() {
        let mut lexer: Lexer = Lexer::new(String::from("[1,  2, \"foo\", true,false, null]"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn object_in_object() {
        let mut lexer: Lexer = Lexer::new(String::from("{ \"inner\": { } }"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn array_in_object() {
        let mut lexer: Lexer = Lexer::new(String::from("{ \"foo\": [1,2,3,4,true,false,null]}"));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }

    #[test]
    fn non_string_values_object() {
        let mut lexer: Lexer = Lexer::new(String::from(
            "{ \"true\": true, \"false\": false, \"null\": null, \"ns\": [3, 4, 12], \"n\": 3 }",
        ));
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), 0);
    }
}
