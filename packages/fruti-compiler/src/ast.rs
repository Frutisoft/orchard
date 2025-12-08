// Abstract Syntax Tree (AST) - Fruti Compiler
//
// Represents the structured parse tree of Fruti source code.
// This is the output of the parser and input to semantic analysis.

use crate::span::{Span, Spanned};

/// A complete Fruti source file
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub items: Vec<Item>,
}

/// Top-level items in a module
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Trait(Trait),
    Impl(Impl),
    TypeAlias(TypeAlias),
    Const(Const),
    Mod(Mod),
    Use(Use),
}

/// Function definition
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Spanned<String>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub is_async: bool,
    pub is_pub: bool,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: Spanned<String>,
    pub ty: Type,
}

/// Struct definition
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: Spanned<String>,
    pub fields: Vec<Field>,
    pub is_pub: bool,
}

/// Struct field
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: Spanned<String>,
    pub ty: Type,
    pub is_pub: bool,
}

/// Enum definition
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: Spanned<String>,
    pub variants: Vec<Variant>,
    pub is_pub: bool,
}

/// Enum variant
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub name: Spanned<String>,
    pub data: VariantData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariantData {
    Unit,                     // None
    Tuple(Vec<Type>),         // Some(i32, String)
    Struct(Vec<Field>),       // Point { x: i32, y: i32 }
}

/// Trait definition
#[derive(Debug, Clone, PartialEq)]
pub struct Trait {
    pub name: Spanned<String>,
    pub methods: Vec<TraitMethod>,
    pub is_pub: bool,
}

/// Trait method signature
#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethod {
    pub name: Spanned<String>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
}

/// Impl block
#[derive(Debug, Clone, PartialEq)]
pub struct Impl {
    pub trait_name: Option<Spanned<String>>,
    pub type_name: Spanned<String>,
    pub methods: Vec<Function>,
}

/// Type alias
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub name: Spanned<String>,
    pub ty: Type,
    pub is_pub: bool,
}

/// Constant definition
#[derive(Debug, Clone, PartialEq)]
pub struct Const {
    pub name: Spanned<String>,
    pub ty: Type,
    pub value: Expr,
    pub is_pub: bool,
}

/// Module definition
#[derive(Debug, Clone, PartialEq)]
pub struct Mod {
    pub name: Spanned<String>,
    pub items: Vec<Item>,
    pub is_pub: bool,
}

/// Use statement
#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    pub path: Vec<Spanned<String>>,
}

/// Type representation
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Simple type: i32, String, etc.
    Simple(Spanned<String>),
    
    /// Reference type: &T
    Ref(Box<Type>),
    
    /// Owned type: own T (explicit ownership)
    Own(Box<Type>),
    
    /// Tuple type: (i32, String)
    Tuple(Vec<Type>),
    
    /// Array type: [i32; 10]
    Array(Box<Type>, Option<usize>),
    
    /// Function type: fn(i32) -> String
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    
    /// Inferred type (placeholder for type inference)
    Infer,
}

/// Block of statements
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub expr: Option<Box<Expr>>, // Optional trailing expression
    pub span: Span,
}

/// Statement
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// Let binding: let x = 5;
    Let {
        name: Spanned<String>,
        ty: Option<Type>,
        value: Option<Expr>,
    },
    
    /// Var binding: var x = 5;
    Var {
        name: Spanned<String>,
        ty: Option<Type>,
        value: Option<Expr>,
    },
    
    /// Expression statement: foo();
    Expr(Expr),
    
    /// Return statement: return 5;
    Return(Option<Expr>),
    
    /// Break statement: break;
    Break,
    
    /// Continue statement: continue;
    Continue,
    
    /// While loop: while x < 10 { ... }
    While {
        condition: Expr,
        body: Block,
    },
    
    /// For loop: for i in 0..10 { ... }
    For {
        var: Spanned<String>,
        iter: Expr,
        body: Block,
    },
    
    /// Infinite loop: loop { ... }
    Loop {
        body: Block,
    },
}

/// Expression
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// Literal values
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
    
    /// Identifier: x, foo
    Ident(String),
    
    /// Binary operation: a + b, x == y
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    
    /// Unary operation: -x, not flag
    Unary {
        op: UnOp,
        expr: Box<Expr>,
    },
    
    /// Function call: foo(x, y)
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    
    /// Method call: obj.method(x)
    MethodCall {
        receiver: Box<Expr>,
        method: Spanned<String>,
        args: Vec<Expr>,
    },
    
    /// Field access: obj.field
    Field {
        expr: Box<Expr>,
        field: Spanned<String>,
    },
    
    /// Index access: arr[0]
    Index {
        expr: Box<Expr>,
        index: Box<Expr>,
    },
    
    /// Range: 0..10, 0..=10
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        inclusive: bool,
    },
    
    /// If expression: if x { a } else { b }
    If {
        condition: Box<Expr>,
        then_block: Block,
        else_block: Option<Block>,
    },
    
    /// Match expression: match x { ... }
    Match {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    
    /// Block expression: { ... }
    Block(Block),
    
    /// Tuple: (1, "hello", 3.14)
    Tuple(Vec<Expr>),
    
    /// Array literal: [1, 2, 3]
    Array(Vec<Expr>),
    
    /// Struct literal: Point { x: 1, y: 2 }
    StructLit {
        name: Spanned<String>,
        fields: Vec<(Spanned<String>, Expr)>,
    },
    
    /// Lambda: |x| x + 1
    Lambda {
        params: Vec<Param>,
        body: Box<Expr>,
    },
    
    /// Await: await some_future()
    Await(Box<Expr>),
    
    /// Try operator: might_fail()?
    Try(Box<Expr>),
    
    /// Type cast: x as i32
    Cast {
        expr: Box<Expr>,
        ty: Type,
    },
    
    /// Type check: x is SomeType
    Is {
        expr: Box<Expr>,
        ty: Type,
    },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    // Arithmetic
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Rem,      // %
    
    // Comparison
    Eq,       // ==
    Ne,       // !=
    Lt,       // <
    Le,       // <=
    Gt,       // >
    Ge,       // >=
    
    // Logical
    And,      // and or &&
    Or,       // or or ||
    
    // Bitwise
    BitAnd,   // &
    BitOr,    // |
    BitXor,   // ^
    Shl,      // <<
    Shr,      // >>
    
    // Assignment
    Assign,   // =
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
    RemAssign, // %=
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,      // -x
    Not,      // not x or !x
    BitNot,   // ~x
}

/// Match arm
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
}

/// Pattern (simplified for MVP)
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard: _
    Wildcard,
    
    /// Identifier: x
    Ident(String),
    
    /// Literal: 42, "hello"
    Literal(Literal),
    
    /// Tuple: (x, y)
    Tuple(Vec<Pattern>),
    
    /// Enum variant: Some(x)
    Variant {
        name: String,
        patterns: Vec<Pattern>,
    },
}

/// Literal values
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

// Helper implementations

impl Type {
    /// Create a simple type from a string
    pub fn simple(name: impl Into<String>, span: Span) -> Self {
        Type::Simple(Spanned {
            value: name.into(),
            span,
        })
    }
}

impl Expr {
    /// Create an expression with kind and span
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Expr { kind, span }
    }
    
    /// Create an integer literal
    pub fn integer(value: i64, span: Span) -> Self {
        Expr::new(ExprKind::Integer(value), span)
    }
    
    /// Create a float literal
    pub fn float(value: f64, span: Span) -> Self {
        Expr::new(ExprKind::Float(value), span)
    }
    
    /// Create a string literal
    pub fn string(value: impl Into<String>, span: Span) -> Self {
        Expr::new(ExprKind::String(value.into()), span)
    }
    
    /// Create a boolean literal
    pub fn bool(value: bool, span: Span) -> Self {
        Expr::new(ExprKind::Bool(value), span)
    }
    
    /// Create an identifier expression
    pub fn ident(name: impl Into<String>, span: Span) -> Self {
        Expr::new(ExprKind::Ident(name.into()), span)
    }
}

impl BinOp {
    /// Get precedence for operator (higher = tighter binding)
    pub fn precedence(&self) -> u8 {
        match self {
            // Assignment (lowest precedence)
            BinOp::Assign | BinOp::AddAssign | BinOp::SubAssign | 
            BinOp::MulAssign | BinOp::DivAssign | BinOp::RemAssign => 1,
            
            // Logical OR
            BinOp::Or => 2,
            
            // Logical AND
            BinOp::And => 3,
            
            // Comparison
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | 
            BinOp::Gt | BinOp::Ge => 4,
            
            // Bitwise OR
            BinOp::BitOr => 5,
            
            // Bitwise XOR
            BinOp::BitXor => 6,
            
            // Bitwise AND
            BinOp::BitAnd => 7,
            
            // Shifts
            BinOp::Shl | BinOp::Shr => 8,
            
            // Addition/Subtraction
            BinOp::Add | BinOp::Sub => 9,
            
            // Multiplication/Division/Remainder (highest precedence)
            BinOp::Mul | BinOp::Div | BinOp::Rem => 10,
        }
    }
    
    /// Check if operator is left-associative
    pub fn is_left_associative(&self) -> bool {
        !matches!(self, BinOp::Assign | BinOp::AddAssign | BinOp::SubAssign | 
                       BinOp::MulAssign | BinOp::DivAssign | BinOp::RemAssign)
    }
}
