// Lexer - Tokenizer for Fruti language
// Converts source code into a stream of tokens

use crate::error::{Error, ErrorKind, Result};
use crate::span::Span;
use crate::token::{Token, TokenKind};
use std::str::Chars;

pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<Chars<'a>>,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars().peekable();
        let current_char = chars.next();
        Self {
            source,
            chars,
            position: 0,
            current_char,
        }
    }

    /// Get all tokens from source
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.value, TokenKind::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace_and_comments();

        let start = self.position;

        match self.current_char {
            None => Ok(Token::new(TokenKind::Eof, Span::new(start, start))),
            Some(ch) => {
                let kind = match ch {
                    // Identifiers and keywords
                    'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(),
                    
                    // Numbers
                    '0'..='9' => self.lex_number()?,
                    
                    // String literals
                    '"' => self.lex_string()?,
                    
                    // Char literals
                    '\'' => self.lex_char()?,
                    
                    // Operators and punctuation
                    '+' => self.lex_plus(),
                    '-' => self.lex_minus(),
                    '*' => self.lex_star(),
                    '/' => self.lex_slash(),
                    '%' => self.simple_token(TokenKind::Percent),
                    
                    '=' => self.lex_equal(),
                    '!' => self.lex_bang(),
                    '<' => self.lex_less(),
                    '>' => self.lex_greater(),
                    
                    '&' => self.lex_amp(),
                    '|' => self.lex_pipe(),
                    '^' => self.simple_token(TokenKind::Caret),
                    '~' => self.simple_token(TokenKind::Tilde),
                    
                    '.' => self.lex_dot(),
                    ':' => self.lex_colon(),
                    '?' => self.simple_token(TokenKind::Question),
                    
                    '(' => self.simple_token(TokenKind::LeftParen),
                    ')' => self.simple_token(TokenKind::RightParen),
                    '{' => self.simple_token(TokenKind::LeftBrace),
                    '}' => self.simple_token(TokenKind::RightBrace),
                    '[' => self.simple_token(TokenKind::LeftBracket),
                    ']' => self.simple_token(TokenKind::RightBracket),
                    
                    ',' => self.simple_token(TokenKind::Comma),
                    ';' => self.simple_token(TokenKind::Semicolon),
                    
                    _ => {
                        self.advance();
                        return Err(Error::new(
                            ErrorKind::UnexpectedCharacter,
                            Span::new(start, self.position),
                            format!("Unexpected character: '{}'", ch),
                        ));
                    }
                };

                let end = self.position;
                Ok(Token::new(kind, Span::new(start, end)))
            }
        }
    }

    /// Advance to next character
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.current_char {
            self.position += ch.len_utf8();
            self.current_char = self.chars.next();
            Some(ch)
        } else {
            None
        }
    }

    /// Peek at next character without consuming
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Check if current char matches and advance if it does
    fn match_char(&mut self, expected: char) -> bool {
        if self.current_char == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.current_char {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                '/' if self.peek() == Some('/') => {
                    // Line comment
                    self.advance(); // '/'
                    self.advance(); // '/'
                    while self.current_char.is_some() && self.current_char != Some('\n') {
                        self.advance();
                    }
                }
                '/' if self.peek() == Some('*') => {
                    // Block comment
                    self.advance(); // '/'
                    self.advance(); // '*'
                    while self.current_char.is_some() {
                        if self.current_char == Some('*') && self.peek() == Some('/') {
                            self.advance(); // '*'
                            self.advance(); // '/'
                            break;
                        }
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    /// Create simple single-character token
    fn simple_token(&mut self, kind: TokenKind) -> TokenKind {
        self.advance();
        kind
    }

    /// Lex identifier or keyword
    fn lex_identifier(&mut self) -> TokenKind {
        let start = self.position;
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let ident = &self.source[start..self.position];
        
        // Check if it's a keyword
        TokenKind::from_keyword(ident).unwrap_or_else(|| TokenKind::Ident(ident.to_string()))
    }

    /// Lex number (integer or float)
    fn lex_number(&mut self) -> Result<TokenKind> {
        let start = self.position;
        
        // Collect digits
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if self.current_char == Some('.') && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(); // '.'
            
            // Collect fractional digits
            while let Some(ch) = self.current_char {
                if ch.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }

            // Parse as float
            let num_str = &self.source[start..self.position];
            match num_str.parse::<f64>() {
                Ok(n) => Ok(TokenKind::Float(n)),
                Err(_) => Err(Error::new(
                    ErrorKind::InvalidNumber,
                    Span::new(start, self.position),
                    format!("Invalid float: {}", num_str),
                )),
            }
        } else {
            // Parse as integer
            let num_str = &self.source[start..self.position];
            match num_str.parse::<i64>() {
                Ok(n) => Ok(TokenKind::Integer(n)),
                Err(_) => Err(Error::new(
                    ErrorKind::InvalidNumber,
                    Span::new(start, self.position),
                    format!("Invalid integer: {}", num_str),
                )),
            }
        }
    }

    /// Lex string literal
    fn lex_string(&mut self) -> Result<TokenKind> {
        let start = self.position;
        self.advance(); // Opening '"'

        let mut string = String::new();
        
        while let Some(ch) = self.current_char {
            match ch {
                '"' => {
                    self.advance(); // Closing '"'
                    return Ok(TokenKind::String(string));
                }
                '\\' => {
                    self.advance();
                    match self.current_char {
                        Some('n') => {
                            string.push('\n');
                            self.advance();
                        }
                        Some('t') => {
                            string.push('\t');
                            self.advance();
                        }
                        Some('r') => {
                            string.push('\r');
                            self.advance();
                        }
                        Some('\\') => {
                            string.push('\\');
                            self.advance();
                        }
                        Some('"') => {
                            string.push('"');
                            self.advance();
                        }
                        Some('{') => {
                            string.push('{');
                            self.advance();
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::InvalidChar,
                                Span::new(start, self.position),
                                "Invalid escape sequence",
                            ));
                        }
                    }
                }
                '\n' | '\r' => {
                    return Err(Error::new(
                        ErrorKind::UnterminatedString,
                        Span::new(start, self.position),
                        "Unterminated string literal",
                    ));
                }
                _ => {
                    string.push(ch);
                    self.advance();
                }
            }
        }

        Err(Error::new(
            ErrorKind::UnterminatedString,
            Span::new(start, self.position),
            "Unterminated string literal",
        ))
    }

    /// Lex character literal
    fn lex_char(&mut self) -> Result<TokenKind> {
        let start = self.position;
        self.advance(); // Opening '\''

        let ch = match self.current_char {
            Some('\\') => {
                self.advance();
                match self.current_char {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('\\') => '\\',
                    Some('\'') => '\'',
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidChar,
                            Span::new(start, self.position),
                            "Invalid escape sequence in char literal",
                        ));
                    }
                }
            }
            Some(ch) => ch,
            None => {
                return Err(Error::new(
                    ErrorKind::UnterminatedChar,
                    Span::new(start, self.position),
                    "Unterminated char literal",
                ));
            }
        };

        self.advance();

        if self.current_char != Some('\'') {
            return Err(Error::new(
                ErrorKind::UnterminatedChar,
                Span::new(start, self.position),
                "Unterminated char literal",
            ));
        }

        self.advance(); // Closing '\''
        Ok(TokenKind::Char(ch))
    }

    // Operator lexing methods

    fn lex_plus(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('=') {
            TokenKind::PlusEqual
        } else {
            TokenKind::Plus
        }
    }

    fn lex_minus(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('=') {
            TokenKind::MinusEqual
        } else if self.match_char('>') {
            TokenKind::Arrow
        } else {
            TokenKind::Minus
        }
    }

    fn lex_star(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('=') {
            TokenKind::StarEqual
        } else {
            TokenKind::Star
        }
    }

    fn lex_slash(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('=') {
            TokenKind::SlashEqual
        } else {
            TokenKind::Slash
        }
    }

    fn lex_equal(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('=') {
            TokenKind::EqualEqual
        } else if self.match_char('>') {
            TokenKind::FatArrow
        } else {
            TokenKind::Equal
        }
    }

    fn lex_bang(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('=') {
            TokenKind::NotEqual
        } else {
            TokenKind::Bang
        }
    }

    fn lex_less(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('=') {
            TokenKind::LessEqual
        } else if self.match_char('<') {
            TokenKind::LessLess
        } else {
            TokenKind::Less
        }
    }

    fn lex_greater(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('=') {
            TokenKind::GreaterEqual
        } else if self.match_char('>') {
            TokenKind::GreaterGreater
        } else {
            TokenKind::Greater
        }
    }

    fn lex_amp(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('&') {
            TokenKind::AmpAmp
        } else {
            TokenKind::Amp
        }
    }

    fn lex_pipe(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('|') {
            TokenKind::PipePipe
        } else {
            TokenKind::Pipe
        }
    }

    fn lex_dot(&mut self) -> TokenKind {
        self.advance();
        if self.match_char('.') {
            if self.match_char('=') {
                TokenKind::DotDotEqual
            } else {
                TokenKind::DotDot
            }
        } else {
            TokenKind::Dot
        }
    }

    fn lex_colon(&mut self) -> TokenKind {
        self.advance();
        if self.match_char(':') {
            TokenKind::ColonColon
        } else {
            TokenKind::Colon
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(source: &str) -> Result<Vec<TokenKind>> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        Ok(tokens.into_iter().map(|t| t.value).collect())
    }

    #[test]
    fn test_keywords() {
        let tokens = lex("fn let if else while for").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Fn,
                TokenKind::Let,
                TokenKind::If,
                TokenKind::Else,
                TokenKind::While,
                TokenKind::For,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let tokens = lex("hello world _test test123").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Ident("hello".to_string()),
                TokenKind::Ident("world".to_string()),
                TokenKind::Ident("_test".to_string()),
                TokenKind::Ident("test123".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let tokens = lex("42 3.14 0 100").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Integer(42),
                TokenKind::Float(3.14),
                TokenKind::Integer(0),
                TokenKind::Integer(100),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_strings() {
        let tokens = lex(r#""hello" "world\n" "test""#).unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::String("hello".to_string()),
                TokenKind::String("world\n".to_string()),
                TokenKind::String("test".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_operators() {
        let tokens = lex("+ - * / == != <= >= && ||").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::EqualEqual,
                TokenKind::NotEqual,
                TokenKind::LessEqual,
                TokenKind::GreaterEqual,
                TokenKind::AmpAmp,
                TokenKind::PipePipe,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_simple_function() {
        let source = r#"
            fn main() {
                let x = 42
                println("Hello, World!")
            }
        "#;
        let tokens = lex(source).unwrap();
        assert_eq!(tokens[0], TokenKind::Fn);
        assert_eq!(tokens[1], TokenKind::Ident("main".to_string()));
        assert_eq!(tokens[2], TokenKind::LeftParen);
    }

    #[test]
    fn test_comments() {
        let source = r#"
            // This is a comment
            let x = 42  // inline comment
            /* block comment */
            let y = 10
        "#;
        let tokens = lex(source).unwrap();
        assert_eq!(tokens[0], TokenKind::Let);
        assert_eq!(tokens[1], TokenKind::Ident("x".to_string()));
    }
}
