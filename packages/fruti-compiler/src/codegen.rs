// LLVM IR Code Generation - Fruti Compiler
//
// Translates the typed AST into LLVM IR

use crate::ast::*;
use crate::error::Result;

/// Code generator for LLVM IR
pub struct CodeGen {
    // For now, we'll just generate a textual representation of LLVM IR
    // Once inkwell is enabled, this will use LLVM Context, Module, Builder
    module_name: String,
}

impl CodeGen {
    pub fn new(module_name: String) -> Self {
        CodeGen { module_name }
    }

    /// Generate LLVM IR for a module
    pub fn generate_module(&mut self, module: &Module) -> Result<String> {
        let mut ir = String::new();

        // Module header
        ir.push_str(&format!("; ModuleID = '{}'\n", self.module_name));
        ir.push_str("source_filename = \"");
        ir.push_str(&self.module_name);
        ir.push_str("\"\n\n");

        // Generate declarations for built-in functions
        ir.push_str("; Built-in functions\n");
        ir.push_str("declare i32 @printf(i8*, ...)\n");
        ir.push_str("declare i32 @puts(i8*)\n\n");

        // Generate code for each item
        for item in &module.items {
            match item {
                Item::Function(func) => {
                    let func_ir = self.generate_function(func)?;
                    ir.push_str(&func_ir);
                    ir.push('\n');
                }
                _ => {
                    // TODO: Implement other item types
                }
            }
        }

        Ok(ir)
    }

    /// Generate LLVM IR for a function
    fn generate_function(&mut self, func: &Function) -> Result<String> {
        let mut ir = String::new();

        // Function signature
        let return_ty = if func.return_type.is_some() {
            "i32" // Simplified: all functions return i32 for now
        } else {
            "void"
        };

        ir.push_str(&format!("define {} @{}(", return_ty, func.name.value));

        // Parameters
        for (i, param) in func.params.iter().enumerate() {
            if i > 0 {
                ir.push_str(", ");
            }
            ir.push_str("i32 %");
            ir.push_str(&param.name.value);
        }

        ir.push_str(") {\n");
        ir.push_str("entry:\n");

        // Function body
        // For MVP, we'll just generate a simple return
        if func.name.value == "main" {
            ir.push_str("  ; Main function body\n");
            ir.push_str("  ret i32 0\n");
        } else if return_ty == "void" {
            ir.push_str("  ret void\n");
        } else {
            ir.push_str("  ret i32 0\n");
        }

        ir.push_str("}\n");

        Ok(ir)
    }
}

// Placeholder for when we enable inkwell
/*
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, PointerValue, IntValue};
use inkwell::types::{BasicTypeEnum, IntType};
use inkwell::AddressSpace;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
        }
    }

    pub fn generate_module(&mut self, ast_module: &Module) -> Result<()> {
        // Declare built-in functions
        self.declare_builtins();

        // Generate code for all items
        for item in &ast_module.items {
            self.generate_item(item)?;
        }

        Ok(())
    }

    fn declare_builtins(&self) {
        // Declare printf
        let i8_type = self.context.i8_type();
        let i8_ptr_type = i8_type.ptr_type(AddressSpace::default());
        let i32_type = self.context.i32_type();

        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
        self.module.add_function("printf", printf_type, None);

        // Declare puts
        let puts_type = i32_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("puts", puts_type, None);
    }

    fn generate_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Function(func) => {
                self.generate_function(func)?;
            }
            _ => {
                // TODO: Other items
            }
        }
        Ok(())
    }

    fn generate_function(&mut self, func: &Function) -> Result<FunctionValue<'ctx>> {
        // Build parameter types
        let param_types: Vec<BasicTypeEnum> = func.params
            .iter()
            .map(|_| self.context.i32_type().into())
            .collect();

        // Build function type
        let return_type = if func.return_type.is_some() {
            self.context.i32_type().into()
        } else {
            self.context.void_type().into()
        };

        let fn_type = match return_type {
            BasicTypeEnum::IntType(int_ty) => {
                int_ty.fn_type(&param_types, false)
            }
            _ => {
                self.context.void_type().fn_type(&param_types, false)
            }
        };

        // Create function
        let function = self.module.add_function(&func.name.value, fn_type, None);
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // Allocate space for parameters
        self.variables.clear();
        for (i, param) in func.params.iter().enumerate() {
            let alloca = self.builder.build_alloca(self.context.i32_type(), &param.name.value);
            self.builder.build_store(alloca, function.get_nth_param(i as u32).unwrap());
            self.variables.insert(param.name.value.clone(), alloca);
        }

        // Generate function body
        self.generate_block(&func.body)?;

        // Add return if not already present
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            if func.return_type.is_none() {
                self.builder.build_return(None);
            } else {
                self.builder.build_return(Some(&self.context.i32_type().const_int(0, false)));
            }
        }

        Ok(function)
    }

    fn generate_block(&mut self, block: &Block) -> Result<Option<IntValue<'ctx>>> {
        // Generate statements
        for stmt in &block.stmts {
            self.generate_stmt(stmt)?;
        }

        // Generate trailing expression if present
        if let Some(expr) = &block.expr {
            return self.generate_expr(expr);
        }

        Ok(None)
    }

    fn generate_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                if let Some(val_expr) = value {
                    let value = self.generate_expr(val_expr)?;
                    if let Some(val) = value {
                        let alloca = self.builder.build_alloca(val.get_type(), &name.value);
                        self.builder.build_store(alloca, val);
                        self.variables.insert(name.value.clone(), alloca);
                    }
                }
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    let value = self.generate_expr(e)?;
                    if let Some(val) = value {
                        self.builder.build_return(Some(&val));
                    } else {
                        self.builder.build_return(None);
                    }
                } else {
                    self.builder.build_return(None);
                }
            }
            _ => {
                // TODO: Other statements
            }
        }
        Ok(())
    }

    fn generate_expr(&mut self, expr: &Expr) -> Result<Option<IntValue<'ctx>>> {
        match &expr.kind {
            ExprKind::Integer(n) => {
                let val = self.context.i32_type().const_int(*n as u64, false);
                Ok(Some(val))
            }
            ExprKind::Ident(name) => {
                if let Some(ptr) = self.variables.get(name) {
                    let val = self.builder.build_load(*ptr, name);
                    Ok(Some(val.into_int_value()))
                } else {
                    Err(Error::new(
                        ErrorKind::SemanticError,
                        expr.span,
                        format!("Undefined variable: {}", name)
                    ))
                }
            }
            ExprKind::Binary { op, left, right } => {
                let l = self.generate_expr(left)?.unwrap();
                let r = self.generate_expr(right)?.unwrap();

                let result = match op {
                    BinOp::Add => self.builder.build_int_add(l, r, "add"),
                    BinOp::Sub => self.builder.build_int_sub(l, r, "sub"),
                    BinOp::Mul => self.builder.build_int_mul(l, r, "mul"),
                    BinOp::Div => self.builder.build_int_signed_div(l, r, "div"),
                    _ => {
                        return Err(Error::new(
                            ErrorKind::SemanticError,
                            expr.span,
                            format!("Unsupported binary operator: {:?}", op)
                        ));
                    }
                };

                Ok(Some(result))
            }
            _ => {
                // TODO: Other expressions
                Ok(None)
            }
        }
    }

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }

    pub fn write_to_file(&self, path: &str) -> std::result::Result<(), String> {
        self.module.print_to_file(path).map_err(|e| e.to_string())
    }
}
*/
