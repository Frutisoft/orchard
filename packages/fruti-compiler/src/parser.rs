// Parser - Fruti Compiler
//
// Recursive descent parser: Tokens to AST
// Implements operator precedence parsing for expressions

use crate::ast::*;
use crate::error::{Error, ErrorKind, Result};
use crate::span::{Span, Spanned};
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Parse a complete module
    pub fn parse_module(&mut self) -> Result<Module> {
        let mut items = Vec::new();

        while !self.is_at_end() {
            items.push(self.parse_item()?);
        }

        Ok(Module { items })
    }

    /// Parse a top-level item
    fn parse_item(&mut self) -> Result<Item> {
        let is_pub = self.eat(&TokenKind::Pub);

        match self.peek().value {
            TokenKind::Fn => {
                let func = self.parse_function(is_pub)?;
                Ok(Item::Function(func))
            }
            TokenKind::Struct => {
                let struc = self.parse_struct(is_pub)?;
                Ok(Item::Struct(struc))
            }
            TokenKind::Enum => {
                let enm = self.parse_enum(is_pub)?;
                Ok(Item::Enum(enm))
            }
            TokenKind::Trait => {
                let trt = self.parse_trait(is_pub)?;
                Ok(Item::Trait(trt))
            }
            TokenKind::Impl => {
                let imp = self.parse_impl()?;
                Ok(Item::Impl(imp))
            }
            TokenKind::Type => {
                let alias = self.parse_type_alias(is_pub)?;
                Ok(Item::TypeAlias(alias))
            }
            TokenKind::Const => {
                let cnst = self.parse_const(is_pub)?;
                Ok(Item::Const(cnst))
            }
            TokenKind::Import => {
                let imp = self.parse_import()?;
                Ok(Item::Import(imp))
            }
            _ => {
                let tok = self.peek();
                Err(Error::new(
                    ErrorKind::UnexpectedToken,
                    tok.span,
                    format!("Expected item, found {:?}", tok.value),
                ))
            }
        }
    }

    /// Parse function definition
    fn parse_function(&mut self, is_pub: bool) -> Result<Function> {
        let is_async = self.eat(&TokenKind::Async);
        self.expect(&TokenKind::Fn)?;

        let name = self.expect_ident()?;
        self.expect(&TokenKind::LeftParen)?;

        let params = self.parse_param_list()?;
        self.expect(&TokenKind::RightParen)?;

        let return_type = if self.eat(&TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(Function {
            name,
            params,
            return_type,
            body,
            is_async,
            is_pub,
        })
    }

    /// Parse parameter list
    fn parse_param_list(&mut self) -> Result<Vec<Param>> {
        let mut params = Vec::new();

        if matches!(self.peek().value, TokenKind::RightParen) {
            return Ok(params);
        }

        loop {
            let name = self.expect_ident()?;
            self.expect(&TokenKind::Colon)?;
            let ty = self.parse_type()?;

            params.push(Param { name, ty });

            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }

        Ok(params)
    }

    /// Parse type annotation
    fn parse_type(&mut self) -> Result<Type> {
        match self.peek().value {
            TokenKind::Amp => {
                self.advance();
                let inner = Box::new(self.parse_type()?);
                Ok(Type::Ref(inner))
            }
            TokenKind::Own => {
                self.advance();
                let inner = Box::new(self.parse_type()?);
                Ok(Type::Own(inner))
            }
            TokenKind::LeftParen => {
                self.advance();
                let mut types = Vec::new();

                if !matches!(self.peek().value, TokenKind::RightParen) {
                    loop {
                        types.push(self.parse_type()?);
                        if !self.eat(&TokenKind::Comma) {
                            break;
                        }
                    }
                }

                self.expect(&TokenKind::RightParen)?;
                Ok(Type::Tuple(types))
            }
            TokenKind::LeftBracket => {
                self.advance();
                let elem_type = Box::new(self.parse_type()?);
                let size = if self.eat(&TokenKind::Semicolon) {
                    if let TokenKind::Integer(n) = self.peek().value {
                        self.advance();
                        Some(n as usize)
                    } else {
                        None
                    }
                } else {
                    None
                };
                self.expect(&TokenKind::RightBracket)?;
                Ok(Type::Array(elem_type, size))
            }
            TokenKind::Ident(_) => {
                let name = self.expect_ident()?;
                Ok(Type::Simple(name))
            }
            _ => {
                let tok = self.peek();
                Err(Error::new(
                    ErrorKind::UnexpectedToken,
                    tok.span,
                    format!("Expected type, found {:?}", tok.value),
                ))
            }
        }
    }

    /// Parse struct definition
    fn parse_struct(&mut self, is_pub: bool) -> Result<Struct> {
        self.expect(&TokenKind::Struct)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::LeftBrace)?;

        let mut fields = Vec::new();
        while !matches!(self.peek().value, TokenKind::RightBrace) {
            let field_is_pub = self.eat(&TokenKind::Pub);
            let field_name = self.expect_ident()?;
            self.expect(&TokenKind::Colon)?;
            let field_ty = self.parse_type()?;

            fields.push(Field {
                name: field_name,
                ty: field_ty,
                is_pub: field_is_pub,
            });

            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }

        self.expect(&TokenKind::RightBrace)?;

        Ok(Struct {
            name,
            fields,
            is_pub,
        })
    }

    /// Parse enum definition
    fn parse_enum(&mut self, is_pub: bool) -> Result<Enum> {
        self.expect(&TokenKind::Enum)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::LeftBrace)?;

        let mut variants = Vec::new();
        while !matches!(self.peek().value, TokenKind::RightBrace) {
            let variant_name = self.expect_ident()?;

            let data = match self.peek().value {
                TokenKind::LeftParen => {
                    self.advance();
                    let mut types = Vec::new();
                    if !matches!(self.peek().value, TokenKind::RightParen) {
                        loop {
                            types.push(self.parse_type()?);
                            if !self.eat(&TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect(&TokenKind::RightParen)?;
                    VariantData::Tuple(types)
                }
                TokenKind::LeftBrace => {
                    self.advance();
                    let mut fields = Vec::new();
                    while !matches!(self.peek().value, TokenKind::RightBrace) {
                        let field_name = self.expect_ident()?;
                        self.expect(&TokenKind::Colon)?;
                        let field_ty = self.parse_type()?;
                        fields.push(Field {
                            name: field_name,
                            ty: field_ty,
                            is_pub: false,
                        });
                        if !self.eat(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(&TokenKind::RightBrace)?;
                    VariantData::Struct(fields)
                }
                _ => VariantData::Unit,
            };

            variants.push(Variant {
                name: variant_name,
                data,
            });

            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }

        self.expect(&TokenKind::RightBrace)?;

        Ok(Enum {
            name,
            variants,
            is_pub,
        })
    }

    /// Parse trait definition (simplified)
    fn parse_trait(&mut self, is_pub: bool) -> Result<Trait> {
        self.expect(&TokenKind::Trait)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::LeftBrace)?;

        let mut methods = Vec::new();
        while !matches!(self.peek().value, TokenKind::RightBrace) {
            self.expect(&TokenKind::Fn)?;
            let method_name = self.expect_ident()?;
            self.expect(&TokenKind::LeftParen)?;
            let params = self.parse_param_list()?;
            self.expect(&TokenKind::RightParen)?;

            let return_type = if self.eat(&TokenKind::Arrow) {
                Some(self.parse_type()?)
            } else {
                None
            };

            self.expect(&TokenKind::Semicolon)?;

            methods.push(TraitMethod {
                name: method_name,
                params,
                return_type,
            });
        }

        self.expect(&TokenKind::RightBrace)?;

        Ok(Trait {
            name,
            methods,
            is_pub,
        })
    }

    /// Parse impl block
    fn parse_impl(&mut self) -> Result<Impl> {
        self.expect(&TokenKind::Impl)?;

        // Try to parse "impl TraitName for TypeName" or "impl TypeName"
        let first_name = self.expect_ident()?;

        let (trait_name, type_name) = if self.eat(&TokenKind::For) {
            let type_name = self.expect_ident()?;
            (Some(first_name), type_name)
        } else {
            (None, first_name)
        };

        self.expect(&TokenKind::LeftBrace)?;

        let mut methods = Vec::new();
        while !matches!(self.peek().value, TokenKind::RightBrace) {
            let method = self.parse_function(false)?;
            methods.push(method);
        }

        self.expect(&TokenKind::RightBrace)?;

        Ok(Impl {
            trait_name,
            type_name,
            methods,
        })
    }

    /// Parse type alias
    fn parse_type_alias(&mut self, is_pub: bool) -> Result<TypeAlias> {
        self.expect(&TokenKind::Type)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::Equal)?;
        let ty = self.parse_type()?;
        self.expect(&TokenKind::Semicolon)?;

        Ok(TypeAlias { name, ty, is_pub })
    }

    /// Parse constant
    fn parse_const(&mut self, is_pub: bool) -> Result<Const> {
        self.expect(&TokenKind::Const)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_type()?;
        self.expect(&TokenKind::Equal)?;
        let value = self.parse_expr()?;
        self.expect(&TokenKind::Semicolon)?;

        Ok(Const {
            name,
            ty,
            value,
            is_pub,
        })
    }

    /// Parse import statement
    fn parse_import(&mut self) -> Result<Import> {
        self.expect(&TokenKind::Import)?;

        let mut path = Vec::new();
        loop {
            path.push(self.expect_ident()?);
            if !self.eat(&TokenKind::ColonColon) {
                break;
            }
        }

        self.expect(&TokenKind::Semicolon)?;

        Ok(Import { path })
    }

    /// Parse a block
    fn parse_block(&mut self) -> Result<Block> {
        let start = self.expect(&TokenKind::LeftBrace)?.span;
        let mut stmts = Vec::new();
        let mut expr = None;

        while !matches!(self.peek().value, TokenKind::RightBrace) {
            // Save position in case we need to backtrack
            let saved_pos = self.pos;

            // Try to parse statement
            match self.try_parse_stmt()? {
                Some(stmt) => {
                    stmts.push(stmt);
                }
                None => {
                    // Not a statement - restore position and parse as trailing expression
                    self.pos = saved_pos;
                    expr = Some(Box::new(self.parse_expr()?));
                    break;
                }
            }
        }

        let end = self.expect(&TokenKind::RightBrace)?.span;
        let span = Span {
            start: start.start,
            end: end.end,
        };

        Ok(Block { stmts, expr, span })
    }

    /// Try to parse a statement (returns None if it's an expression)
    fn try_parse_stmt(&mut self) -> Result<Option<Stmt>> {
        match self.peek().value {
            TokenKind::Let => {
                self.advance();
                let mutable = self.eat(&TokenKind::Mut);
                let name = self.expect_ident()?;
                let ty = if self.eat(&TokenKind::Colon) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                let value = if self.eat(&TokenKind::Equal) {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(&TokenKind::Semicolon)?;
                Ok(Some(Stmt::Let {
                    name,
                    ty,
                    value,
                    mutable,
                }))
            }
            TokenKind::Return => {
                self.advance();
                let value = if !matches!(self.peek().value, TokenKind::Semicolon) {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(&TokenKind::Semicolon)?;
                Ok(Some(Stmt::Return(value)))
            }
            TokenKind::Break => {
                self.advance();
                self.expect(&TokenKind::Semicolon)?;
                Ok(Some(Stmt::Break))
            }
            TokenKind::Continue => {
                self.advance();
                self.expect(&TokenKind::Semicolon)?;
                Ok(Some(Stmt::Continue))
            }
            TokenKind::While => {
                self.advance();
                let condition = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Some(Stmt::While { condition, body }))
            }
            TokenKind::For => {
                self.advance();
                let var = self.expect_ident()?;
                self.expect(&TokenKind::In)?;
                let iter = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Some(Stmt::For { var, iter, body }))
            }
            TokenKind::Loop => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Some(Stmt::Loop { body }))
            }
            _ => {
                // Check if it's an if/match/block expression used as a statement
                let starts_with_control = matches!(
                    self.peek().value,
                    TokenKind::If | TokenKind::Match | TokenKind::LeftBrace
                );

                // Try parsing as expression statement
                let expr = self.parse_expr()?;

                // Control flow expressions don't need semicolons when used as statements
                if starts_with_control || self.eat(&TokenKind::Semicolon) {
                    Ok(Some(Stmt::Expr(expr)))
                } else {
                    // It's a trailing expression, not a statement
                    Ok(None)
                }
            }
        }
    }

    /// Parse an expression
    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_binary_expr(0)
    }

    /// Parse binary expression with precedence climbing
    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr> {
        let mut left = self.parse_unary_expr()?;

        loop {
            // Check for range operators first (they're not binary ops in our AST)
            if matches!(
                self.peek().value,
                TokenKind::DotDot | TokenKind::DotDotEqual
            ) {
                let inclusive = matches!(self.peek().value, TokenKind::DotDotEqual);
                self.advance();

                // Range can have optional end
                let end = if matches!(
                    self.peek().value,
                    TokenKind::RightBrace
                        | TokenKind::Semicolon
                        | TokenKind::RightParen
                        | TokenKind::Comma
                ) {
                    None
                } else {
                    Some(Box::new(self.parse_unary_expr()?))
                };

                let span = Span {
                    start: left.span.start,
                    end: end.as_ref().map(|e| e.span.end).unwrap_or(left.span.end),
                };

                left = Expr {
                    kind: ExprKind::Range {
                        start: Some(Box::new(left)),
                        end,
                        inclusive,
                    },
                    span,
                };
                continue;
            }

            let op = match self.peek_binop() {
                Some(op) if op.precedence() >= min_prec => op,
                _ => break,
            };

            self.advance();

            let next_min_prec = if op.is_left_associative() {
                op.precedence() + 1
            } else {
                op.precedence()
            };

            let right = self.parse_binary_expr(next_min_prec)?;

            let span = Span {
                start: left.span.start,
                end: right.span.end,
            };

            left = Expr {
                kind: ExprKind::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            };
        }

        Ok(left)
    }

    /// Try to get binary operator from current token
    fn peek_binop(&self) -> Option<BinOp> {
        match self.peek().value {
            TokenKind::Plus => Some(BinOp::Add),
            TokenKind::Minus => Some(BinOp::Sub),
            TokenKind::Star => Some(BinOp::Mul),
            TokenKind::Slash => Some(BinOp::Div),
            TokenKind::Percent => Some(BinOp::Rem),
            TokenKind::EqualEqual => Some(BinOp::Eq),
            TokenKind::NotEqual => Some(BinOp::Ne),
            TokenKind::Less => Some(BinOp::Lt),
            TokenKind::LessEqual => Some(BinOp::Le),
            TokenKind::Greater => Some(BinOp::Gt),
            TokenKind::GreaterEqual => Some(BinOp::Ge),
            TokenKind::And | TokenKind::AmpAmp => Some(BinOp::And),
            TokenKind::Or | TokenKind::PipePipe => Some(BinOp::Or),
            TokenKind::Amp => Some(BinOp::BitAnd),
            TokenKind::Pipe => Some(BinOp::BitOr),
            TokenKind::Caret => Some(BinOp::BitXor),
            TokenKind::LessLess => Some(BinOp::Shl),
            TokenKind::GreaterGreater => Some(BinOp::Shr),
            TokenKind::Equal => Some(BinOp::Assign),
            TokenKind::PlusEqual => Some(BinOp::AddAssign),
            TokenKind::MinusEqual => Some(BinOp::SubAssign),
            TokenKind::StarEqual => Some(BinOp::MulAssign),
            TokenKind::SlashEqual => Some(BinOp::DivAssign),
            _ => None,
        }
    }

    /// Parse unary expression
    fn parse_unary_expr(&mut self) -> Result<Expr> {
        match self.peek().value {
            TokenKind::Minus => {
                let start = self.advance().span;
                let expr = Box::new(self.parse_unary_expr()?);
                let span = Span {
                    start: start.start,
                    end: expr.span.end,
                };
                Ok(Expr {
                    kind: ExprKind::Unary {
                        op: UnOp::Neg,
                        expr,
                    },
                    span,
                })
            }
            TokenKind::Not | TokenKind::Bang => {
                let start = self.advance().span;
                let expr = Box::new(self.parse_unary_expr()?);
                let span = Span {
                    start: start.start,
                    end: expr.span.end,
                };
                Ok(Expr {
                    kind: ExprKind::Unary {
                        op: UnOp::Not,
                        expr,
                    },
                    span,
                })
            }
            TokenKind::Tilde => {
                let start = self.advance().span;
                let expr = Box::new(self.parse_unary_expr()?);
                let span = Span {
                    start: start.start,
                    end: expr.span.end,
                };
                Ok(Expr {
                    kind: ExprKind::Unary {
                        op: UnOp::BitNot,
                        expr,
                    },
                    span,
                })
            }
            _ => self.parse_postfix_expr(),
        }
    }

    /// Parse postfix expressions (call, index, field access, etc.)
    fn parse_postfix_expr(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary_expr()?;

        loop {
            match self.peek().value {
                TokenKind::LeftParen => {
                    self.advance();
                    let args = self.parse_expr_list()?;
                    let end = self.expect(&TokenKind::RightParen)?.span;

                    expr = Expr {
                        span: Span {
                            start: expr.span.start,
                            end: end.end,
                        },
                        kind: ExprKind::Call {
                            func: Box::new(expr),
                            args,
                        },
                    };
                }
                TokenKind::LeftBracket => {
                    self.advance();
                    let index = Box::new(self.parse_expr()?);
                    let end = self.expect(&TokenKind::RightBracket)?.span;

                    expr = Expr {
                        span: Span {
                            start: expr.span.start,
                            end: end.end,
                        },
                        kind: ExprKind::Index {
                            expr: Box::new(expr),
                            index,
                        },
                    };
                }
                TokenKind::Dot => {
                    self.advance();
                    let field = self.expect_ident()?;

                    // Check if it's a method call
                    if matches!(self.peek().value, TokenKind::LeftParen) {
                        self.advance();
                        let args = self.parse_expr_list()?;
                        let end = self.expect(&TokenKind::RightParen)?.span;

                        expr = Expr {
                            span: Span {
                                start: expr.span.start,
                                end: end.end,
                            },
                            kind: ExprKind::MethodCall {
                                receiver: Box::new(expr),
                                method: field,
                                args,
                            },
                        };
                    } else {
                        expr = Expr {
                            span: Span {
                                start: expr.span.start,
                                end: field.span.end,
                            },
                            kind: ExprKind::Field {
                                expr: Box::new(expr),
                                field,
                            },
                        };
                    }
                }
                TokenKind::Question => {
                    let end = self.advance().span;
                    expr = Expr {
                        span: Span {
                            start: expr.span.start,
                            end: end.end,
                        },
                        kind: ExprKind::Try(Box::new(expr)),
                    };
                }
                TokenKind::As => {
                    self.advance();
                    let ty = self.parse_type()?;
                    expr = Expr {
                        span: expr.span, // Approximation
                        kind: ExprKind::Cast {
                            expr: Box::new(expr),
                            ty,
                        },
                    };
                }
                TokenKind::Is => {
                    self.advance();
                    let ty = self.parse_type()?;
                    expr = Expr {
                        span: expr.span, // Approximation
                        kind: ExprKind::Is {
                            expr: Box::new(expr),
                            ty,
                        },
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parse primary expression (literals, identifiers, parenthesized, etc.)
    fn parse_primary_expr(&mut self) -> Result<Expr> {
        let tok = self.peek();

        match &tok.value {
            TokenKind::Integer(n) => {
                let n = *n;
                let span = self.advance().span;
                Ok(Expr::integer(n, span))
            }
            TokenKind::Float(f) => {
                let f = *f;
                let span = self.advance().span;
                Ok(Expr::float(f, span))
            }
            TokenKind::String(s) => {
                let s = s.clone();
                let span = self.advance().span;
                Ok(Expr::string(s, span))
            }
            TokenKind::Char(c) => {
                let c = *c;
                let span = self.advance().span;
                Ok(Expr::new(ExprKind::Char(c), span))
            }
            TokenKind::True => {
                let span = self.advance().span;
                Ok(Expr::bool(true, span))
            }
            TokenKind::False => {
                let span = self.advance().span;
                Ok(Expr::bool(false, span))
            }
            TokenKind::Ident(_) => {
                let ident = self.expect_ident()?;

                // Check for struct literal
                if matches!(self.peek().value, TokenKind::LeftBrace) {
                    self.advance();
                    let fields = self.parse_struct_lit_fields()?;
                    let end = self.expect(&TokenKind::RightBrace)?.span;

                    Ok(Expr {
                        span: Span {
                            start: ident.span.start,
                            end: end.end,
                        },
                        kind: ExprKind::StructLit {
                            name: ident,
                            fields,
                        },
                    })
                } else {
                    let span = ident.span;
                    Ok(Expr::ident(ident.value, span))
                }
            }
            TokenKind::LeftParen => {
                let start = self.advance().span;

                // Empty tuple or single expression
                if matches!(self.peek().value, TokenKind::RightParen) {
                    let end = self.advance().span;
                    return Ok(Expr {
                        span: Span {
                            start: start.start,
                            end: end.end,
                        },
                        kind: ExprKind::Tuple(vec![]),
                    });
                }

                let first_expr = self.parse_expr()?;

                // Check for tuple
                if self.eat(&TokenKind::Comma) {
                    let mut exprs = vec![first_expr];

                    if !matches!(self.peek().value, TokenKind::RightParen) {
                        loop {
                            exprs.push(self.parse_expr()?);
                            if !self.eat(&TokenKind::Comma) {
                                break;
                            }
                        }
                    }

                    let end = self.expect(&TokenKind::RightParen)?.span;
                    Ok(Expr {
                        span: Span {
                            start: start.start,
                            end: end.end,
                        },
                        kind: ExprKind::Tuple(exprs),
                    })
                } else {
                    self.expect(&TokenKind::RightParen)?;
                    Ok(first_expr)
                }
            }
            TokenKind::LeftBracket => {
                let start = self.advance().span;
                let exprs = self.parse_expr_list()?;
                let end = self.expect(&TokenKind::RightBracket)?.span;

                Ok(Expr {
                    span: Span {
                        start: start.start,
                        end: end.end,
                    },
                    kind: ExprKind::Array(exprs),
                })
            }
            TokenKind::LeftBrace => {
                let block = self.parse_block()?;
                let span = block.span;
                Ok(Expr {
                    span,
                    kind: ExprKind::Block(block),
                })
            }
            TokenKind::If => {
                self.advance();
                let condition = Box::new(self.parse_expr()?);
                let then_block = self.parse_block()?;
                let else_block = if self.eat(&TokenKind::Else) {
                    Some(self.parse_block()?)
                } else {
                    None
                };

                Ok(Expr {
                    span: then_block.span, // Approximation
                    kind: ExprKind::If {
                        condition,
                        then_block,
                        else_block,
                    },
                })
            }
            TokenKind::Match => {
                self.advance();
                let expr = Box::new(self.parse_expr()?);
                self.expect(&TokenKind::LeftBrace)?;

                let mut arms = Vec::new();
                while !matches!(self.peek().value, TokenKind::RightBrace) {
                    let pattern = self.parse_pattern()?;
                    let guard = if self.eat(&TokenKind::If) {
                        Some(self.parse_expr()?)
                    } else {
                        None
                    };
                    self.expect(&TokenKind::FatArrow)?;
                    let body = self.parse_expr()?;

                    arms.push(MatchArm {
                        pattern,
                        guard,
                        body,
                    });

                    if !self.eat(&TokenKind::Comma) {
                        break;
                    }
                }

                let end = self.expect(&TokenKind::RightBrace)?.span;

                Ok(Expr {
                    span: end, // Approximation
                    kind: ExprKind::Match { expr, arms },
                })
            }
            TokenKind::Await => {
                let start = self.advance().span;
                let expr = Box::new(self.parse_postfix_expr()?);
                Ok(Expr {
                    span: Span {
                        start: start.start,
                        end: expr.span.end,
                    },
                    kind: ExprKind::Await(expr),
                })
            }
            TokenKind::Pipe => {
                // Lambda expression
                let start = self.advance().span;
                let mut params = Vec::new();

                if !matches!(self.peek().value, TokenKind::Pipe) {
                    loop {
                        let name = self.expect_ident()?;
                        let ty = if self.eat(&TokenKind::Colon) {
                            self.parse_type()?
                        } else {
                            Type::Infer
                        };
                        params.push(Param { name, ty });

                        if !self.eat(&TokenKind::Comma) {
                            break;
                        }
                    }
                }

                self.expect(&TokenKind::Pipe)?;

                let body = if matches!(self.peek().value, TokenKind::LeftBrace) {
                    let block = self.parse_block()?;
                    Box::new(Expr {
                        span: block.span,
                        kind: ExprKind::Block(block),
                    })
                } else {
                    Box::new(self.parse_expr()?)
                };

                Ok(Expr {
                    span: Span {
                        start: start.start,
                        end: body.span.end,
                    },
                    kind: ExprKind::Lambda { params, body },
                })
            }
            _ => Err(Error::new(
                ErrorKind::UnexpectedToken,
                tok.span,
                format!("Expected expression, found {:?}", tok.value),
            )),
        }
    }

    /// Parse pattern (simplified for MVP)
    fn parse_pattern(&mut self) -> Result<Pattern> {
        match self.peek().value {
            TokenKind::Ident(ref s) if s == "_" => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            TokenKind::Ident(_) => {
                let ident = self.expect_ident()?;

                // Check for variant pattern
                if matches!(self.peek().value, TokenKind::LeftParen) {
                    self.advance();
                    let mut patterns = Vec::new();

                    if !matches!(self.peek().value, TokenKind::RightParen) {
                        loop {
                            patterns.push(self.parse_pattern()?);
                            if !self.eat(&TokenKind::Comma) {
                                break;
                            }
                        }
                    }

                    self.expect(&TokenKind::RightParen)?;

                    Ok(Pattern::Variant {
                        name: ident.value,
                        patterns,
                    })
                } else {
                    Ok(Pattern::Ident(ident.value))
                }
            }
            TokenKind::Integer(n) => {
                self.advance();
                Ok(Pattern::Literal(Literal::Integer(n)))
            }
            TokenKind::String(ref s) => {
                let s = s.clone();
                self.advance();
                Ok(Pattern::Literal(Literal::String(s)))
            }
            TokenKind::True => {
                self.advance();
                Ok(Pattern::Literal(Literal::Bool(true)))
            }
            TokenKind::False => {
                self.advance();
                Ok(Pattern::Literal(Literal::Bool(false)))
            }
            _ => {
                let tok = self.peek();
                Err(Error::new(
                    ErrorKind::UnexpectedToken,
                    tok.span,
                    format!("Expected pattern, found {:?}", tok.value),
                ))
            }
        }
    }

    /// Parse expression list (comma-separated)
    fn parse_expr_list(&mut self) -> Result<Vec<Expr>> {
        let mut exprs = Vec::new();

        if matches!(
            self.peek().value,
            TokenKind::RightParen | TokenKind::RightBracket
        ) {
            return Ok(exprs);
        }

        loop {
            exprs.push(self.parse_expr()?);
            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }

        Ok(exprs)
    }

    /// Parse struct literal fields
    fn parse_struct_lit_fields(&mut self) -> Result<Vec<(Spanned<String>, Expr)>> {
        let mut fields = Vec::new();

        if matches!(self.peek().value, TokenKind::RightBrace) {
            return Ok(fields);
        }

        loop {
            let name = self.expect_ident()?;
            self.expect(&TokenKind::Colon)?;
            let value = self.parse_expr()?;

            fields.push((name, value));

            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }

        Ok(fields)
    }

    // === Helper functions ===

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        if !matches!(tok.value, TokenKind::Eof) {
            self.pos += 1;
        }
        tok
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().value, TokenKind::Eof)
    }

    fn eat(&mut self, kind: &TokenKind) -> bool {
        if std::mem::discriminant(&self.peek().value) == std::mem::discriminant(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<&Token> {
        let tok = self.peek();
        if std::mem::discriminant(&tok.value) == std::mem::discriminant(kind) {
            Ok(self.advance())
        } else {
            Err(Error::new(
                ErrorKind::UnexpectedToken,
                tok.span,
                format!("Expected {:?}, found {:?}", kind, tok.value),
            ))
        }
    }

    fn expect_ident(&mut self) -> Result<Spanned<String>> {
        let tok = self.peek();
        if let TokenKind::Ident(name) = &tok.value {
            let name = name.clone();
            let span = self.advance().span;
            Ok(Spanned { value: name, span })
        } else {
            Err(Error::new(
                ErrorKind::UnexpectedToken,
                tok.span,
                format!("Expected identifier, found {:?}", tok.value),
            ))
        }
    }
}
