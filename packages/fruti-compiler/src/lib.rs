// Fruti Compiler
// Module declarations

pub mod lexer;
pub mod token;
pub mod span;
pub mod error;
pub mod ast;
pub mod parser;
pub mod semantic;
pub mod codegen;

// Re-exports for convenience
pub use lexer::Lexer;
pub use token::{Token, TokenKind};
pub use span::{Span, Spanned};
pub use error::{Error, Result};
pub use ast::*;
pub use parser::Parser;
pub use semantic::TypeChecker;
pub use codegen::CodeGen;
