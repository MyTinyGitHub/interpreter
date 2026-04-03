# AGENTS.md - Monkey Interpreter in Rust

This file provides guidelines for agentic coding agents working on this codebase.

## Project Overview

A Rust implementation of the Monkey programming language interpreter, following the classic lexer→parser→evaluator pipeline with tree-walking evaluator and closures support.

## Build, Lint, and Test Commands

### Building
```bash
cargo build           # Debug build
cargo build --release # Release build
```

### Running
```bash
cargo run                    # Start the REPL
echo "let x = 5; x + 10;" | cargo run  # Run with input
```

### Testing
```bash
cargo test                   # Run all tests
cargo test test_token       # Run single test (use exact name)
cargo test lexer            # Run tests matching pattern
cargo test -- --nocapture   # Show output
cargo test --doc            # Run doc tests
```

### Linting and Formatting
```bash
cargo fmt                   # Format code
cargo fmt -- --check        # Check formatting
cargo clippy                # Run clippy lints
cargo clippy -- -D warnings # Warnings as errors
cargo audit                 # Security vulnerabilities
```

### Other Commands
```bash
cargo check   # Check without building
cargo doc     # Generate documentation
cargo clean   # Clean build artifacts
```

## Code Style Guidelines

### Module Organization
- Use directory-based modules (e.g., `lexer/mod.rs`) or single-file modules (e.g., `repl.rs`)
- Module declarations: `pub mod module_name;` with `#[cfg(test)] mod tests;` for inline tests

### Imports
- Internal: `crate::` prefix (e.g., `crate::lexer::Lexer`)
- External: full paths (e.g., `use std::collections::HashMap;`)
- Grouping: External imports first, then blank line, then internal

```rust
use std::collections::HashMap;

use crate::{
    ast::{Expression, Identifier},
    error::MonkeyError,
    lexer::Lexer,
};
```

### Naming Conventions
- **Types/Enums**: PascalCase (e.g., `Lexer`, `Token`, `MonkeyError`)
- **Functions/Variables**: snake_case (e.g., `next_token`, `parse_program`)
- **Struct fields**: snake_case (e.g., `position`, `read_position`)
- **Files**: kebab-case (e.g., `lexer/mod.rs`, `evaluator/object.rs`)

### Type Conventions
- i64 for integers, bool for booleans
- `Box<T>` for recursive types and heap allocation
- `Option<T>` for nullable, `Result<T, E>` for errors
- `Vec<T>` for sequences, `HashMap<K, V>` for key-value stores

### Error Handling
- Use `thiserror` with `#[derive(Error, Debug)]`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonkeyError {
    #[error("lexer: {0}")]
    Lexer(String),
    #[error("parser: {0}")]
    Parser(String),
    #[error("{0}")]
    Evaluator(String),
}
```

- Use `?` operator for error propagation
- Create errors: `MonkeyError::Lexer(format!("{}", e))`

### Struct and Enum Design
- **Visibility**: Default private; use `pub` only for public API
- **Derives**: Always `Debug`; derive `Clone` when needed; `PartialEq`/`Eq`/`Hash` for enum keys
- **Tuple structs**: For newtype wrappers (e.g., `pub struct Identifier { pub token: Token }`)
- **Data enums**: For tagged unions (e.g., `Expression`, `Statement`, `Object`)

### Method Conventions
- Constructor: `pub fn new(...) -> Self`
- Mutating: `&mut self`, Query: `&self`
- Builder pattern: Return `Self` for chaining

### Pattern Matching
- Exhaustive `match` for enums
- `if let Some(x) = value` for optionals
- Match guards: `match expr { Pattern if condition => ... }`

### Testing
- Include `#[cfg(test)] mod tests;` in each module
- Test functions: `#[test] fn test_token()`
- Assertion: `assert_eq!(expected, actual)`

```rust
#[test]
fn test_next_token() {
    let input = "let five = 5;";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(Token::Let, lexer.next_token());
}
```

### Code Formatting
- Lines under 100 characters
- 4 spaces indentation
- Trailing commas in multi-line expressions

### Common Patterns
- Token enum with `literal()` and `precedence()` methods
- AST nodes as enums with associated data
- Tree-walking evaluator with recursive pattern matching
- HashMap-based environment for closures

## Project Structure

```
src/
├── main.rs           # Entry point, calls repl_loop()
├── repl.rs           # Read-Eval-Print Loop
├── error/mod.rs      # MonkeyError enum
├── token/mod.rs      # Token enum and Precedence
├── ast/mod.rs        # AST node definitions
├── lexer/mod.rs      # Lexer + tests
├── parser/mod.rs    # Pratt parser + tests
└── evaluator/
    ├── mod.rs        # Evaluator + tests
    ├── object.rs     # Object value types
    └── environment.rs # Scope/closure handling
```

## Dependencies
- **thiserror**: 2.0.x for custom error types
- **Rust edition**: 2024 (requires recent stable)

## Key Design Decisions
1. **Pratt parser**: Top-down operator precedence parsing
2. **Tree-walking evaluator**: Simple recursive evaluation
3. **Environment chaining**: Closures capture defining environment
4. **Token-based AST**: Nodes carry Token for source position
