// Semantic Analysis - Fruti Compiler
//
// Type checking, name resolution, and semantic validation

use crate::ast::*;
use crate::error::{Error, ErrorKind, Result};
use crate::span::Span;
use std::collections::HashMap;

/// Built-in primitive types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
    String,
    Unit, // ()
}

/// Resolved type after semantic analysis
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedType {
    Primitive(PrimitiveType),
    Reference(Box<ResolvedType>),
    Owned(Box<ResolvedType>),
    Tuple(Vec<ResolvedType>),
    Array(Box<ResolvedType>, Option<usize>),
    Function {
        params: Vec<ResolvedType>,
        return_type: Box<ResolvedType>,
    },
    UserDefined(String), // Struct, enum, trait
    Unknown,             // For type inference
}

/// Symbol kinds
#[derive(Debug, Clone)]
pub enum Symbol {
    Variable {
        ty: ResolvedType,
        mutable: bool,
        span: Span,
    },
    Function {
        params: Vec<ResolvedType>,
        return_type: ResolvedType,
        span: Span,
    },
    Type {
        kind: TypeKind,
        span: Span,
    },
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Struct,
    Enum,
    Trait,
}

/// Symbol table with scoping
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = SymbolTable {
            scopes: vec![HashMap::new()],
        };

        // Add built-in types
        table.define_builtin_types();

        table
    }

    fn define_builtin_types(&mut self) {
        let builtin_span = Span { start: 0, end: 0 };

        for type_name in &[
            "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "char",
            "str",
        ] {
            self.scopes[0].insert(
                type_name.to_string(),
                Symbol::Type {
                    kind: TypeKind::Struct, // Treat primitives as built-in "structs"
                    span: builtin_span,
                },
            );
        }

        // Add built-in functions
        self.scopes[0].insert(
            "print".to_string(),
            Symbol::Function {
                params: vec![ResolvedType::Primitive(PrimitiveType::String)],
                return_type: ResolvedType::Primitive(PrimitiveType::Unit),
                span: builtin_span,
            },
        );

        self.scopes[0].insert(
            "println".to_string(),
            Symbol::Function {
                params: vec![ResolvedType::Primitive(PrimitiveType::String)],
                return_type: ResolvedType::Primitive(PrimitiveType::Unit),
                span: builtin_span,
            },
        );
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, symbol: Symbol) -> std::result::Result<(), String> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                return Err(format!(
                    "Symbol '{}' already defined in current scope",
                    name
                ));
            }
            scope.insert(name, symbol);
            Ok(())
        } else {
            Err("No scope available".to_string())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }
}

/// Type checker
pub struct TypeChecker {
    symbols: SymbolTable,
    current_function_return: Option<ResolvedType>,
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbols: SymbolTable::new(),
            current_function_return: None,
        }
    }

    /// Check a module
    pub fn check_module(&mut self, module: &Module) -> Result<()> {
        // First pass: collect all top-level definitions
        for item in &module.items {
            self.collect_item(item)?;
        }

        // Second pass: type check all items
        for item in &module.items {
            self.check_item(item)?;
        }

        Ok(())
    }

    /// Collect top-level definitions
    fn collect_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Function(func) => {
                let params = func
                    .params
                    .iter()
                    .map(|p| self.resolve_type(&p.ty))
                    .collect::<Result<Vec<_>>>()?;

                let return_type = func
                    .return_type
                    .as_ref()
                    .map(|t| self.resolve_type(t))
                    .transpose()?
                    .unwrap_or(ResolvedType::Primitive(PrimitiveType::Unit));

                self.symbols
                    .define(
                        func.name.value.clone(),
                        Symbol::Function {
                            params,
                            return_type,
                            span: func.name.span,
                        },
                    )
                    .map_err(|e| Error::new(ErrorKind::SemanticError, func.name.span, e))?;
            }
            Item::Struct(s) => {
                self.symbols
                    .define(
                        s.name.value.clone(),
                        Symbol::Type {
                            kind: TypeKind::Struct,
                            span: s.name.span,
                        },
                    )
                    .map_err(|e| Error::new(ErrorKind::SemanticError, s.name.span, e))?;
            }
            Item::Enum(e) => {
                self.symbols
                    .define(
                        e.name.value.clone(),
                        Symbol::Type {
                            kind: TypeKind::Enum,
                            span: e.name.span,
                        },
                    )
                    .map_err(|err| Error::new(ErrorKind::SemanticError, e.name.span, err))?;
            }
            Item::Trait(t) => {
                self.symbols
                    .define(
                        t.name.value.clone(),
                        Symbol::Type {
                            kind: TypeKind::Trait,
                            span: t.name.span,
                        },
                    )
                    .map_err(|err| Error::new(ErrorKind::SemanticError, t.name.span, err))?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Type check an item
    fn check_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Function(func) => self.check_function(func),
            _ => Ok(()), // TODO: Implement other items
        }
    }

    /// Type check a function
    fn check_function(&mut self, func: &Function) -> Result<()> {
        self.symbols.enter_scope();

        // Add parameters to scope
        for param in &func.params {
            let ty = self.resolve_type(&param.ty)?;
            self.symbols
                .define(
                    param.name.value.clone(),
                    Symbol::Variable {
                        ty,
                        mutable: false,
                        span: param.name.span,
                    },
                )
                .map_err(|e| Error::new(ErrorKind::SemanticError, param.name.span, e))?;
        }

        // Set current function return type
        let return_type = func
            .return_type
            .as_ref()
            .map(|t| self.resolve_type(t))
            .transpose()?
            .unwrap_or(ResolvedType::Primitive(PrimitiveType::Unit));
        self.current_function_return = Some(return_type.clone());

        // Check function body
        self.check_block(&func.body)?;

        self.current_function_return = None;
        self.symbols.exit_scope();

        Ok(())
    }

    /// Type check a block
    fn check_block(&mut self, block: &Block) -> Result<ResolvedType> {
        self.symbols.enter_scope();

        for stmt in &block.stmts {
            self.check_stmt(stmt)?;
        }

        let result_type = if let Some(expr) = &block.expr {
            self.check_expr(expr)?
        } else {
            ResolvedType::Primitive(PrimitiveType::Unit)
        };

        self.symbols.exit_scope();
        Ok(result_type)
    }

    /// Type check a statement
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Let { name, ty, value, mutable } => {
                let value_type = if let Some(v) = value {
                    self.check_expr(v)?
                } else {
                    return Err(Error::new(
                        ErrorKind::SemanticError,
                        name.span,
                        "Let binding must have an initializer or explicit type".to_string(),
                    ));
                };

                // If type annotation exists, check compatibility
                if let Some(annotated_ty) = ty {
                    let expected_ty = self.resolve_type(annotated_ty)?;
                    if !self.types_compatible(&value_type, &expected_ty) {
                        return Err(Error::new(
                            ErrorKind::TypeMismatch,
                            name.span,
                            format!(
                                "Type mismatch: expected {:?}, found {:?}",
                                expected_ty, value_type
                            ),
                        ));
                    }
                }

                self.symbols
                    .define(
                        name.value.clone(),
                        Symbol::Variable {
                            ty: value_type,
                            mutable: *mutable,
                            span: name.span,
                        },
                    )
                    .map_err(|e| Error::new(ErrorKind::SemanticError, name.span, e))?;
            }
            Stmt::Return(expr) => {
                let return_type = if let Some(e) = expr {
                    self.check_expr(e)?
                } else {
                    ResolvedType::Primitive(PrimitiveType::Unit)
                };

                if let Some(expected) = &self.current_function_return {
                    if !self.types_compatible(&return_type, expected) {
                        return Err(Error::new(
                            ErrorKind::TypeMismatch,
                            Span { start: 0, end: 0 }, // TODO: Better span
                            format!(
                                "Return type mismatch: expected {:?}, found {:?}",
                                expected, return_type
                            ),
                        ));
                    }
                }
            }
            Stmt::Expr(expr) => {
                self.check_expr(expr)?;
            }
            Stmt::While { condition, body } => {
                let cond_ty = self.check_expr(condition)?;
                if cond_ty != ResolvedType::Primitive(PrimitiveType::Bool) {
                    return Err(Error::new(
                        ErrorKind::TypeMismatch,
                        condition.span,
                        format!("While condition must be bool, found {:?}", cond_ty),
                    ));
                }
                self.check_block(body)?;
            }
            Stmt::For { var, iter, body } => {
                self.symbols.enter_scope();

                // For now, assume iterator yields i32 (simplified)
                self.symbols
                    .define(
                        var.value.clone(),
                        Symbol::Variable {
                            ty: ResolvedType::Primitive(PrimitiveType::I32),
                            mutable: false,
                            span: var.span,
                        },
                    )
                    .map_err(|e| Error::new(ErrorKind::SemanticError, var.span, e))?;

                self.check_expr(iter)?;
                self.check_block(body)?;

                self.symbols.exit_scope();
            }
            Stmt::Loop { body } => {
                self.check_block(body)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Type check an expression
    fn check_expr(&mut self, expr: &Expr) -> Result<ResolvedType> {
        match &expr.kind {
            ExprKind::Integer(_) => Ok(ResolvedType::Primitive(PrimitiveType::I32)),
            ExprKind::Float(_) => Ok(ResolvedType::Primitive(PrimitiveType::F64)),
            ExprKind::String(_) => Ok(ResolvedType::Primitive(PrimitiveType::String)),
            ExprKind::Char(_) => Ok(ResolvedType::Primitive(PrimitiveType::Char)),
            ExprKind::Bool(_) => Ok(ResolvedType::Primitive(PrimitiveType::Bool)),

            ExprKind::Ident(name) => {
                match self.symbols.lookup(name) {
                    Some(Symbol::Variable { ty, .. }) => Ok(ty.clone()),
                    Some(Symbol::Function {
                        params,
                        return_type,
                        ..
                    }) => {
                        // Allow functions to be used as values (for function pointers, closures, etc.)
                        Ok(ResolvedType::Function {
                            params: params.clone(),
                            return_type: Box::new(return_type.clone()),
                        })
                    }
                    _ => Err(Error::new(
                        ErrorKind::SemanticError,
                        expr.span,
                        format!("Undefined variable '{}'", name),
                    )),
                }
            }

            ExprKind::Binary { op, left, right } => {
                let left_ty = self.check_expr(left)?;
                let right_ty = self.check_expr(right)?;

                self.check_binary_op(*op, &left_ty, &right_ty, expr.span)
            }

            ExprKind::Unary { op, expr: inner } => {
                let inner_ty = self.check_expr(inner)?;
                self.check_unary_op(*op, &inner_ty, expr.span)
            }

            ExprKind::Call { func, args: _ } => {
                let func_ty = self.check_expr(func)?;

                // Extract return type from function type
                match func_ty {
                    ResolvedType::Function { return_type, .. } => Ok(*return_type),
                    _ => {
                        // For now, allow any type to be called (simplified)
                        Ok(ResolvedType::Unknown)
                    }
                }
            }

            ExprKind::If {
                condition,
                then_block,
                else_block,
            } => {
                let cond_ty = self.check_expr(condition)?;
                if cond_ty != ResolvedType::Primitive(PrimitiveType::Bool) {
                    return Err(Error::new(
                        ErrorKind::TypeMismatch,
                        condition.span,
                        format!("If condition must be bool, found {:?}", cond_ty),
                    ));
                }

                let then_ty = self.check_block(then_block)?;

                if let Some(else_blk) = else_block {
                    let else_ty = self.check_block(else_blk)?;
                    if self.types_compatible(&then_ty, &else_ty) {
                        Ok(then_ty)
                    } else {
                        Ok(ResolvedType::Primitive(PrimitiveType::Unit))
                    }
                } else {
                    Ok(ResolvedType::Primitive(PrimitiveType::Unit))
                }
            }

            ExprKind::Block(block) => self.check_block(block),

            ExprKind::Range { .. } => {
                // Ranges are their own type - for MVP just return Unknown
                Ok(ResolvedType::Unknown)
            }

            _ => Ok(ResolvedType::Unknown),
        }
    }

    /// Check binary operation type compatibility
    fn check_binary_op(
        &self,
        op: BinOp,
        left: &ResolvedType,
        right: &ResolvedType,
        span: Span,
    ) -> Result<ResolvedType> {
        use BinOp::*;
        use PrimitiveType::*;

        match op {
            Add | Sub | Mul | Div | Rem => {
                // Arithmetic operators require numeric types
                if self.is_numeric(left) && self.types_compatible(left, right) {
                    Ok(left.clone())
                } else {
                    Err(Error::new(
                        ErrorKind::TypeMismatch,
                        span,
                        format!(
                            "Arithmetic operation requires numeric types, found {:?} and {:?}",
                            left, right
                        ),
                    ))
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge => {
                // Comparison operators return bool
                if self.types_compatible(left, right) {
                    Ok(ResolvedType::Primitive(Bool))
                } else {
                    Err(Error::new(
                        ErrorKind::TypeMismatch,
                        span,
                        format!(
                            "Comparison requires compatible types, found {:?} and {:?}",
                            left, right
                        ),
                    ))
                }
            }
            And | Or => {
                // Logical operators require bool
                if *left == ResolvedType::Primitive(Bool) && *right == ResolvedType::Primitive(Bool)
                {
                    Ok(ResolvedType::Primitive(Bool))
                } else {
                    Err(Error::new(
                        ErrorKind::TypeMismatch,
                        span,
                        format!(
                            "Logical operation requires bool, found {:?} and {:?}",
                            left, right
                        ),
                    ))
                }
            }
            _ => Ok(ResolvedType::Unknown), // TODO: Implement other operators
        }
    }

    /// Check unary operation type compatibility
    fn check_unary_op(&self, op: UnOp, operand: &ResolvedType, span: Span) -> Result<ResolvedType> {
        use PrimitiveType::*;
        use UnOp::*;

        match op {
            Neg => {
                if self.is_numeric(operand) {
                    Ok(operand.clone())
                } else {
                    Err(Error::new(
                        ErrorKind::TypeMismatch,
                        span,
                        format!("Negation requires numeric type, found {:?}", operand),
                    ))
                }
            }
            Not => {
                if *operand == ResolvedType::Primitive(Bool) {
                    Ok(ResolvedType::Primitive(Bool))
                } else {
                    Err(Error::new(
                        ErrorKind::TypeMismatch,
                        span,
                        format!("Logical not requires bool, found {:?}", operand),
                    ))
                }
            }
            BitNot => {
                if self.is_integer(operand) {
                    Ok(operand.clone())
                } else {
                    Err(Error::new(
                        ErrorKind::TypeMismatch,
                        span,
                        format!("Bitwise not requires integer type, found {:?}", operand),
                    ))
                }
            }
        }
    }

    /// Resolve AST type to semantic type
    fn resolve_type(&self, ty: &Type) -> Result<ResolvedType> {
        resolve_type_helper(ty)
    }
}

/// Helper function to resolve AST type to semantic type
fn resolve_type_helper(ty: &Type) -> Result<ResolvedType> {
    match ty {
        Type::Simple(name) => match name.value.as_str() {
            "i8" => Ok(ResolvedType::Primitive(PrimitiveType::I8)),
            "i16" => Ok(ResolvedType::Primitive(PrimitiveType::I16)),
            "i32" => Ok(ResolvedType::Primitive(PrimitiveType::I32)),
            "i64" => Ok(ResolvedType::Primitive(PrimitiveType::I64)),
            "u8" => Ok(ResolvedType::Primitive(PrimitiveType::U8)),
            "u16" => Ok(ResolvedType::Primitive(PrimitiveType::U16)),
            "u32" => Ok(ResolvedType::Primitive(PrimitiveType::U32)),
            "u64" => Ok(ResolvedType::Primitive(PrimitiveType::U64)),
            "f32" => Ok(ResolvedType::Primitive(PrimitiveType::F32)),
            "f64" => Ok(ResolvedType::Primitive(PrimitiveType::F64)),
            "bool" => Ok(ResolvedType::Primitive(PrimitiveType::Bool)),
            "char" => Ok(ResolvedType::Primitive(PrimitiveType::Char)),
            "str" => Ok(ResolvedType::Primitive(PrimitiveType::String)),
            _ => Ok(ResolvedType::UserDefined(name.value.clone())),
        },
        Type::Ref(inner) => {
            let inner_ty = resolve_type_helper(inner)?;
            Ok(ResolvedType::Reference(Box::new(inner_ty)))
        }
        Type::Own(inner) => {
            let inner_ty = resolve_type_helper(inner)?;
            Ok(ResolvedType::Owned(Box::new(inner_ty)))
        }
        Type::Tuple(types) => {
            let resolved: Result<Vec<_>> = types.iter().map(resolve_type_helper).collect();
            Ok(ResolvedType::Tuple(resolved?))
        }
        Type::Array(elem_ty, size) => {
            let elem = resolve_type_helper(elem_ty)?;
            Ok(ResolvedType::Array(Box::new(elem), *size))
        }
        Type::Infer => Ok(ResolvedType::Unknown),
        _ => Ok(ResolvedType::Unknown),
    }
}

impl TypeChecker {
    /// Check if two types are compatible
    fn types_compatible(&self, a: &ResolvedType, b: &ResolvedType) -> bool {
        if a == b {
            return true;
        }

        // Allow Unknown to be compatible with anything (for type inference)
        if matches!(a, ResolvedType::Unknown) || matches!(b, ResolvedType::Unknown) {
            return true;
        }

        false
    }

    /// Check if type is numeric
    fn is_numeric(&self, ty: &ResolvedType) -> bool {
        matches!(
            ty,
            ResolvedType::Primitive(
                PrimitiveType::I8
                    | PrimitiveType::I16
                    | PrimitiveType::I32
                    | PrimitiveType::I64
                    | PrimitiveType::U8
                    | PrimitiveType::U16
                    | PrimitiveType::U32
                    | PrimitiveType::U64
                    | PrimitiveType::F32
                    | PrimitiveType::F64
            )
        )
    }

    /// Check if type is integer
    fn is_integer(&self, ty: &ResolvedType) -> bool {
        matches!(
            ty,
            ResolvedType::Primitive(
                PrimitiveType::I8
                    | PrimitiveType::I16
                    | PrimitiveType::I32
                    | PrimitiveType::I64
                    | PrimitiveType::U8
                    | PrimitiveType::U16
                    | PrimitiveType::U32
                    | PrimitiveType::U64
            )
        )
    }
}
