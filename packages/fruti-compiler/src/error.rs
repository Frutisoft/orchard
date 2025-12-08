// Error handling for Fruti compiler

use crate::span::Span;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // Lexer errors
    UnterminatedString,
    UnterminatedChar,
    InvalidNumber,
    InvalidChar,
    UnexpectedCharacter,

    // Parser errors (for future)
    UnexpectedToken,
    ExpectedToken,

    // Semantic errors
    UndefinedVariable,
    TypeMismatch,
    SemanticError,
}

impl Error {
    pub fn new(kind: ErrorKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    pub fn lexer_error(span: Span, message: impl Into<String>) -> Self {
        Self::new(ErrorKind::UnexpectedCharacter, span, message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error at {}: {}", self.span, self.message)
    }
}

impl std::error::Error for Error {}
