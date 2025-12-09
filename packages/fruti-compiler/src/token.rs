// Token - Lexical tokens for Fruti language
// Based on Language Design Decisions specification

use std::fmt;

/// A token with its location in source code
pub type Token = crate::span::Spanned<TokenKind>;

/// All token types in Fruti language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    True,
    False,

    // Identifiers and Keywords
    Ident(String),

    // Keywords - Control Flow
    If,
    Else,
    While,
    For,
    In,
    Loop,
    Break,
    Continue,
    Return,
    Match,

    // Keywords - Definitions
    Fn,
    Let,
    Mut,   // Mutable modifier: let mut x = 5
    Const, // Constants
    Struct,
    Enum,
    Trait,
    Impl,
    Type,
    Import,    // Import from module
    SelfLower, // self (lowercase)
    SelfUpper, // Self (uppercase type)

    // Keywords - Ownership (Phase 2)
    Own, // Explicit ownership transfer
    Ref, // Reference (will be inferred in most cases)

    // Keywords - Other
    As,
    Is, // Type checking and boolean properties
    Pub,
    Async,
    Await,

    // Boolean operators - Natural language
    And, // Natural: and
    Or,  // Natural: or
    Not, // Natural: not

    // Operators - Arithmetic
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %

    // Operators - Comparison
    EqualEqual,   // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // Operators - Logical (symbolic)
    AmpAmp,   // && (allowed alongside 'and')
    PipePipe, // || (allowed alongside 'or')
    Bang,     // ! (allowed alongside 'not')

    // Operators - Bitwise
    Amp,            // &
    Pipe,           // |
    Caret,          // ^
    Tilde,          // ~
    LessLess,       // <<
    GreaterGreater, // >>

    // Operators - Assignment
    Equal,      // =
    PlusEqual,  // +=
    MinusEqual, // -=
    StarEqual,  // *=
    SlashEqual, // /=

    // Operators - Other
    Dot,         // .
    DotDot,      // .. (exclusive range)
    DotDotEqual, // ..= (inclusive range)
    Colon,       // :
    ColonColon,  // ::
    Question,    // ? (error propagation)
    Arrow,       // -> (return type)
    FatArrow,    // => (match arms)

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // Punctuation
    Comma,     // ,
    Semicolon, // ;

    // Special
    Eof,           // End of file
    Error(String), // Lexical error
}

impl TokenKind {
    /// Check if token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::If
                | TokenKind::Else
                | TokenKind::While
                | TokenKind::For
                | TokenKind::In
                | TokenKind::Loop
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Return
                | TokenKind::Match
                | TokenKind::Fn
                | TokenKind::Let
                | TokenKind::Mut
                | TokenKind::Const
                | TokenKind::Struct
                | TokenKind::Enum
                | TokenKind::Trait
                | TokenKind::Impl
                | TokenKind::Type
                | TokenKind::Import
                | TokenKind::SelfLower
                | TokenKind::SelfUpper
                | TokenKind::Own
                | TokenKind::Ref
                | TokenKind::As
                | TokenKind::Is
                | TokenKind::Pub
                | TokenKind::Async
                | TokenKind::Await
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::Not
                | TokenKind::True
                | TokenKind::False
        )
    }

    /// Get keyword from string
    pub fn from_keyword(s: &str) -> Option<TokenKind> {
        match s {
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "while" => Some(TokenKind::While),
            "for" => Some(TokenKind::For),
            "in" => Some(TokenKind::In),
            "loop" => Some(TokenKind::Loop),
            "break" => Some(TokenKind::Break),
            "continue" => Some(TokenKind::Continue),
            "return" => Some(TokenKind::Return),
            "match" => Some(TokenKind::Match),
            "fn" => Some(TokenKind::Fn),
            "let" => Some(TokenKind::Let),
            "mut" => Some(TokenKind::Mut),
            "const" => Some(TokenKind::Const),
            "struct" => Some(TokenKind::Struct),
            "enum" => Some(TokenKind::Enum),
            "trait" => Some(TokenKind::Trait),
            "impl" => Some(TokenKind::Impl),
            "type" => Some(TokenKind::Type),
            "import" => Some(TokenKind::Import),
            "self" => Some(TokenKind::SelfLower),
            "Self" => Some(TokenKind::SelfUpper),
            "own" => Some(TokenKind::Own),
            "ref" => Some(TokenKind::Ref),
            "as" => Some(TokenKind::As),
            "is" => Some(TokenKind::Is),
            "pub" => Some(TokenKind::Pub),
            "async" => Some(TokenKind::Async),
            "await" => Some(TokenKind::Await),
            "and" => Some(TokenKind::And),
            "or" => Some(TokenKind::Or),
            "not" => Some(TokenKind::Not),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            _ => None,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Integer(n) => write!(f, "{}", n),
            TokenKind::Float(n) => write!(f, "{}", n),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::Char(c) => write!(f, "'{}'", c),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::For => write!(f, "for"),
            TokenKind::In => write!(f, "in"),
            TokenKind::Loop => write!(f, "loop"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Match => write!(f, "match"),
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Mut => write!(f, "mut"),
            TokenKind::Const => write!(f, "const"),
            TokenKind::Struct => write!(f, "struct"),
            TokenKind::Enum => write!(f, "enum"),
            TokenKind::Trait => write!(f, "trait"),
            TokenKind::Impl => write!(f, "impl"),
            TokenKind::Type => write!(f, "type"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::SelfLower => write!(f, "self"),
            TokenKind::SelfUpper => write!(f, "Self"),
            TokenKind::Own => write!(f, "own"),
            TokenKind::Ref => write!(f, "ref"),
            TokenKind::As => write!(f, "as"),
            TokenKind::Is => write!(f, "is"),
            TokenKind::Pub => write!(f, "pub"),
            TokenKind::Async => write!(f, "async"),
            TokenKind::Await => write!(f, "await"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::Not => write!(f, "not"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::AmpAmp => write!(f, "&&"),
            TokenKind::PipePipe => write!(f, "||"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Amp => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::LessLess => write!(f, "<<"),
            TokenKind::GreaterGreater => write!(f, ">>"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::PlusEqual => write!(f, "+="),
            TokenKind::MinusEqual => write!(f, "-="),
            TokenKind::StarEqual => write!(f, "*="),
            TokenKind::SlashEqual => write!(f, "/="),
            TokenKind::Dot => write!(f, "."),
            TokenKind::DotDot => write!(f, ".."),
            TokenKind::DotDotEqual => write!(f, "..="),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::ColonColon => write!(f, "::"),
            TokenKind::Question => write!(f, "?"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::FatArrow => write!(f, "=>"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}
