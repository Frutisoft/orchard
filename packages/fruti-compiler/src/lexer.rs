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
    last_token: Option<TokenKind>,
    pending_semicolon: Option<()>,
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
            last_token: None,
            pending_semicolon: None,
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
        // Remember position before skipping whitespace for newline detection
        let before_skip = self.position;
        self.skip_whitespace_and_comments();
        let had_newline = self.source[before_skip..self.position].contains('\n');

        let start = self.position;

        match self.current_char {
            None => {
                // Insert semicolon before EOF if last token could end a statement
                if self.should_insert_semicolon_before_eof() {
                    // Clear last_token so we don't insert another semicolon
                    self.last_token = Some(TokenKind::Semicolon);
                    // Set pending to ensure EOF comes next
                    self.pending_semicolon = Some(());
                    return Ok(Token::new(TokenKind::Semicolon, Span::new(start, start)));
                }
                Ok(Token::new(TokenKind::Eof, Span::new(start, start)))
            }
            Some(ch) => {
                // Check if we should insert semicolon before this token
                if had_newline
                    && self.pending_semicolon.is_none()
                    && self.should_insert_semicolon_before(ch)
                {
                    // Mark that we inserted a semicolon so we don't insert another
                    self.last_token = Some(TokenKind::Semicolon);
                    // Mark pending so we process the actual token next
                    self.pending_semicolon = Some(());
                    return Ok(Token::new(TokenKind::Semicolon, Span::new(start, start)));
                }

                // Clear pending flag now that we're processing the real token
                self.pending_semicolon = None;

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
                self.last_token = Some(kind.clone());
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

    /// Check if we should insert a semicolon before this character
    /// Based on Go's automatic semicolon insertion rules
    fn should_insert_semicolon_before(&self, _next_char: char) -> bool {
        // Insert if last token could end a statement
        // The rule is simple: did the previous token end a statement?
        // We don't care what comes next (except we check for newlines elsewhere)
        self.last_token_can_end_statement()
    }

    /// Check if we should insert a semicolon before EOF
    fn should_insert_semicolon_before_eof(&self) -> bool {
        self.last_token_can_end_statement()
    }

    /// Check if the last token could end a statement
    /// Per spec: identifiers, literals, return/break/continue, closing delimiters
    fn last_token_can_end_statement(&self) -> bool {
        match &self.last_token {
            None => false,
            Some(kind) => match kind {
                // Identifiers and literals
                TokenKind::Ident(_)
                | TokenKind::Integer(_)
                | TokenKind::Float(_)
                | TokenKind::String(_)
                | TokenKind::Char(_)
                | TokenKind::True
                | TokenKind::False => true,

                // Keywords that end statements
                TokenKind::Return | TokenKind::Break | TokenKind::Continue => true,

                // Closing delimiters
                TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace => true,

                // Everything else doesn't trigger ASI
                _ => false,
            },
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
        if self.current_char == Some('.') && self.peek().is_some_and(|c| c.is_ascii_digit()) {
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
        let tokens = lex("fn let mut if else while for").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Fn,
                TokenKind::Let,
                TokenKind::Mut,
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
                TokenKind::Semicolon, // Auto-inserted at EOF
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let tokens = lex("42 3.15 0 100").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Integer(42),
                TokenKind::Float(3.15),
                TokenKind::Integer(0),
                TokenKind::Integer(100),
                TokenKind::Semicolon, // Auto-inserted at EOF
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
                TokenKind::Semicolon, // Auto-inserted at EOF
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

    #[test]
    fn test_automatic_semicolon_insertion() {
        // Test ASI after identifiers and literals
        let source = r#"
let x = 42
let y = 100
        "#;
        let tokens = lex(source).unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Ident("x".to_string()),
                TokenKind::Equal,
                TokenKind::Integer(42),
                TokenKind::Semicolon, // Auto-inserted!
                TokenKind::Let,
                TokenKind::Ident("y".to_string()),
                TokenKind::Equal,
                TokenKind::Integer(100),
                TokenKind::Semicolon, // Auto-inserted!
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_asi_with_return() {
        // Test ASI after return keyword
        let source = r#"
fn test() -> i32 {
    return 42
}
        "#;
        let tokens = lex(source).unwrap();
        // Debug: print all tokens
        println!("Tokens: {:?}", tokens);
        // Find the return keyword
        let return_idx = tokens.iter().position(|t| *t == TokenKind::Return).unwrap();
        // Check that semicolon was inserted after the integer
        assert_eq!(tokens[return_idx + 1], TokenKind::Integer(42));
        assert_eq!(tokens[return_idx + 2], TokenKind::Semicolon); // Auto-inserted!
    }

    #[test]
    fn test_no_double_asi_on_braces() {
        // Test that RightBrace itself gets ASI after it, but content before it also gets ASI
        let source = r#"
fn test() {
    let x = 42
}
        "#;
        let tokens = lex(source).unwrap();
        println!("Tokens: {:?}", tokens);
        // Should have semicolon after 42
        let int_idx = tokens
            .iter()
            .position(|t| matches!(t, TokenKind::Integer(42)))
            .unwrap();
        assert_eq!(tokens[int_idx + 1], TokenKind::Semicolon); // After 42
        assert_eq!(tokens[int_idx + 2], TokenKind::RightBrace); // Then }
                                                                // RightBrace also ends a statement, so it gets a semicolon too
        assert_eq!(tokens[int_idx + 3], TokenKind::Semicolon); // After }
    }

    #[test]
    fn test_explicit_semicolons_still_work() {
        // Test that explicit semicolons still work
        let source = r#"
let x = 42;
let y = 100;
        "#;
        let tokens = lex(source).unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Ident("x".to_string()),
                TokenKind::Equal,
                TokenKind::Integer(42),
                TokenKind::Semicolon, // Explicit
                TokenKind::Let,
                TokenKind::Ident("y".to_string()),
                TokenKind::Equal,
                TokenKind::Integer(100),
                TokenKind::Semicolon, // Explicit
                TokenKind::Eof,
            ]
        );
    }
}
