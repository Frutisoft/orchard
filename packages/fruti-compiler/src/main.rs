// Fruti Compiler - Main Entry Point
//
// This is the bootstrapped version written in Rust.
// The self-hosting compiler (written in Fruti) is a future goal.

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use fruti_compiler::{Lexer, Parser as FrutiParser, TypeChecker, CodeGen};

#[derive(Parser)]
#[command(name = "fruti")]
#[command(about = "The Fruti programming language compiler", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a Fruti project
    Build {
        /// Input source file
        #[arg(value_name = "FILE")]
        input: PathBuf,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Optimization level (0-3)
        #[arg(short = 'O', long, default_value = "0")]
        opt_level: u8,
        
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
        
        /// Target platform
        #[arg(long, default_value = "native")]
        target: String,
    },
    
    /// Run a Fruti program
    Run {
        /// Input source file
        #[arg(value_name = "FILE")]
        input: PathBuf,
        
        /// Arguments to pass to the program
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// Check a Fruti program without building
    Check {
        /// Input source file
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },
    
    /// Format Fruti source code
    Fmt {
        /// Input source file or directory
        #[arg(value_name = "PATH")]
        path: PathBuf,
        
        /// Check formatting without modifying files
        #[arg(long)]
        check: bool,
    },
    
    /// Lint Fruti source code
    Lint {
        /// Input source file or directory
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
    
    /// Run tests
    Test {
        /// Input source file or directory
        #[arg(value_name = "PATH")]
        path: Option<PathBuf>,
    },
    
    /// Create a new Fruti project
    New {
        /// Project name
        #[arg(value_name = "NAME")]
        name: String,
        
        /// Project type (binary, library)
        #[arg(long, default_value = "binary")]
        project_type: String,
    },
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Build { input, output, opt_level, release, target } => {
            compile_file(&input, output.as_deref(), opt_level, release, &target)?;
        }
        
        Commands::Run { input, args } => {
            println!("Running {:?}...", input);
            if !args.is_empty() {
                println!("  Args: {:?}", args);
            }
            
            // TODO: Compile and execute
            println!("\n[TODO] Run command in progress (Phase 1: building lexer first)");
        }
        
        Commands::Check { input } => {
            check_file(&input)?;
        }
        
        Commands::Fmt { path, check } => {
            println!("Formatting {:?}...", path);
            if check {
                println!("  Mode: Check only");
            }
            
            // TODO: Implement formatter
            println!("\n[TODO] Formatter planned for Phase 2");
        }
        
        Commands::Lint { path } => {
            println!("Linting {:?}...", path);
            
            // TODO: Implement linter
            println!("\n[TODO] Linter planned for Phase 2");
        }
        
        Commands::Test { path } => {
            println!("Running tests...");
            if let Some(p) = path {
                println!("  Path: {:?}", p);
            }
            
            // TODO: Implement test runner
            println!("\n[TODO] Test runner planned for Phase 2");
        }
        
        Commands::New { name, project_type } => {
            println!("Creating new {} project: {}", project_type, name);
            
            // TODO: Implement project scaffolding
            println!("\n[TODO] Project creation planned for Phase 2");
        }
    }

    Ok(())
}

fn compile_file(
    input: &PathBuf,
    output: Option<&Path>,
    opt_level: u8,
    release: bool,
    target: &str,
) -> Result<()> {
    println!("[BUILD] Building {:?}...", input);
    println!("  Target: {}", target);
    println!("  Optimization: {}", if release { 3 } else { opt_level });
    
    // Read source file
    let source = fs::read_to_string(input)
        .with_context(|| format!("Failed to read file: {:?}", input))?;
    
    println!("\n[Phase 1] Lexical Analysis");
    
    // Tokenize
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()
        .with_context(|| format!("Failed to tokenize file: {:?}", input))?;
    
    println!("  [OK] Tokenized {} tokens", tokens.len());
    
    // Display tokens if verbose
    if std::env::var("FRUTI_VERBOSE").is_ok() {
        println!("\n  Tokens:");
        for (i, token) in tokens.iter().enumerate() {
            println!("    {:3}: {:?}", i, token.value);
        }
    }
    
    println!("\n[Phase 2] Parsing");
    
    // Parse
    let mut parser = FrutiParser::new(tokens);
    let ast = parser.parse_module()
        .with_context(|| format!("Failed to parse file: {:?}", input))?;
    
    println!("  [OK] Parsed {} items", ast.items.len());
    
    // Display AST if verbose
    if std::env::var("FRUTI_VERBOSE").is_ok() {
        println!("\n  AST:");
        println!("{:#?}", ast);
    }
    
    println!("\n[Phase 3] Semantic Analysis");
    
    // Type checking
    let mut type_checker = TypeChecker::new();
    match type_checker.check_module(&ast) {
        Ok(()) => {
            println!("  [OK] Type checking passed");
        }
        Err(e) => {
            println!("  [ERROR] Type checking failed: {}", e);
            return Err(e.into());
        }
    }
    
    println!("\n[Phase 4] LLVM IR Code Generation");
    
    // Generate LLVM IR
    let mut codegen = CodeGen::new(
        input.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("module")
            .to_string()
    );
    
    let ir = codegen.generate_module(&ast)
        .with_context(|| format!("Failed to generate IR for file: {:?}", input))?;
    
    println!("  [OK] Generated LLVM IR ({} bytes)", ir.len());
    
    // Write IR to file if output specified
    if let Some(out) = output {
        let ir_path = out.with_extension("ll");
        fs::write(&ir_path, &ir)
            .with_context(|| format!("Failed to write IR file: {:?}", ir_path))?;
        println!("  [OK] Written IR to {:?}", ir_path);
    }
    
    // Display IR if verbose
    if std::env::var("FRUTI_VERBOSE").is_ok() {
        println!("\n  Generated LLVM IR:");
        println!("{}", ir);
    }
    
    println!("\n[TODO] Object file generation and linking");
    println!("  Current status: Lexer [OK] | Parser [OK] | Semantic [OK] | Codegen [OK] | Linking [TODO]");
    
    if let Some(out) = output {
        println!("\n  Output would be written to: {:?}", out);
    }
    
    Ok(())
}

fn check_file(input: &PathBuf) -> Result<()> {
    println!("[CHECK] Checking {:?}...", input);
    
    // Read source file
    let source = fs::read_to_string(input)
        .with_context(|| format!("Failed to read file: {:?}", input))?;
    
    // Tokenize
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => {
            println!("  [OK] Lexical analysis passed ({} tokens)", tokens.len());
            
            // Show tokens in verbose mode
            if std::env::var("FRUTI_VERBOSE").is_ok() {
                println!("\n  Tokens:");
                for (i, token) in tokens.iter().enumerate() {
                    println!("    {}: {:?} at {}..{}", i, token.value, token.span.start, token.span.end);
                }
            }
            
            tokens
        }
        Err(e) => {
            eprintln!("  [ERROR] Lexical error: {}", e);
            return Err(e.into());
        }
    };
    
    // Parse
    let mut parser = FrutiParser::new(tokens);
    let ast = match parser.parse_module() {
        Ok(ast) => {
            println!("  [OK] Parsing passed ({} items)", ast.items.len());
            
            // Show AST in verbose mode
            if std::env::var("FRUTI_VERBOSE").is_ok() {
                println!("\n  AST:");
                println!("{:#?}", ast);
            }
            
            ast
        }
        Err(e) => {
            eprintln!("  [ERROR] Parse error: {}", e);
            return Err(e.into());
        }
    };
    
    // Type check
    let mut type_checker = TypeChecker::new();
    match type_checker.check_module(&ast) {
        Ok(()) => {
            println!("  [OK] Type checking passed");
        }
        Err(e) => {
            eprintln!("  [ERROR] Type checking failed: {}", e);
            return Err(e.into());
        }
    }
    
    // Generate IR (but don't write to file)
    let mut codegen = CodeGen::new(
        input.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("module")
            .to_string()
    );
    
    match codegen.generate_module(&ast) {
        Ok(ir) => {
            println!("  [OK] IR generation passed ({} bytes)", ir.len());
            
            // Display IR if verbose
            if std::env::var("FRUTI_VERBOSE").is_ok() {
                println!("\n  Generated LLVM IR:");
                println!("{}", ir);
            }
        }
        Err(e) => {
            eprintln!("  [ERROR] IR generation failed: {}", e);
            return Err(e.into());
        }
    }
    
    println!("\n[OK] All checks passed");
    
    Ok(())
}
