// Fruti Compiler
// Module declarations

pub mod ast;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod span;
pub mod token;

// Re-exports for convenience
pub use ast::*;
pub use codegen::CodeGen;
pub use error::{Error, Result};
pub use lexer::Lexer;
pub use parser::Parser;
pub use semantic::TypeChecker;
pub use span::{Span, Spanned};
pub use token::{Token, TokenKind};
