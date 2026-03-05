# Monkey Interpreter in Rust

An implementation of the Monkey programming language interpreter in Rust, following Thorsten Ball's *Writing an Interpreter in Go* — with the original Go replaced entirely by Rust.

## Why I Built This

This interpreter isn't a detour — it's foundational to what I'm actually building: a [distributed SQL database engine](../distributed-db) in Rust.

Before writing a SQL query engine, I wanted to deeply understand what happens between raw text and meaningful computation. Building a lexer, parser, and evaluator from scratch for a simpler language first makes the database query layer much less intimidating — because the pipeline is identical: tokenize the input, parse it into an AST, evaluate it. Same architecture, different grammar.

I chose to implement it in Rust rather than Go for a second reason — the book's tree-walking evaluator exercises exactly the kinds of recursive, ownership-heavy patterns that I knew I'd need to get comfortable with for the database project.

## How This Connects to the Distributed SQL Database

The path from this interpreter to a SQL query engine is a change of grammar, not architecture:

- The **lexer** will tokenize SQL keywords (`SELECT`, `FROM`, `WHERE`) instead of Monkey keywords
- The **parser** will build a SQL AST — statements, expressions, predicates — using the same Pratt parsing technique I developed here
- The **evaluator** becomes the **execution engine** — walking the query plan and fetching data from the [LSM storage layer](../lsm-tree) instead of an in-memory environment

The closure environment and the SQL query planner are solving the same underlying problem: mapping names to values in a structured scope. Building it here first, in a simpler language, means I arrive at the database query layer with the pattern already internalized.

## The Monkey Language

Monkey is a C-like language with a clean, minimal feature set — enough to be interesting, not so much that it obscures the implementation. It supports:

- Integer and boolean literals
- Variable bindings with `let`
- Functions as first-class values
- Closures and higher-order functions
- Prefix and infix operators
- Conditionals
- Arrays and hashes (maps)
- A REPL for interactive use

```monkey
let double = fn(x) { x * 2 };
let numbers = [1, 2, 3, 4, 5];

let map = fn(arr, f) {
    let iter = fn(arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            iter(rest(arr), push(accumulated, f(first(arr))));
        }
    };
    iter(arr, []);
};

map(numbers, double);  // [2, 4, 6, 8, 10]
```

## Architecture

Source code passes through three transformations before it becomes output:

```
Source Code (string)
       │
       ▼
    Lexer
  (tokenization)
       │
       ▼
    Tokens
       │
       ▼
    Parser
  (Pratt parsing)
       │
       ▼
  AST (Abstract Syntax Tree)
       │
       ▼
   Evaluator
 (tree-walking)
       │
       ▼
    Output
```

### Lexer

Scans raw source code character by character and produces a flat stream of tokens — keywords (`let`, `fn`, `if`), identifiers, literals, operators, delimiters. The lexer has no understanding of structure or meaning, only of what the individual pieces are.

### Parser

Takes the token stream and builds an Abstract Syntax Tree — a tree structure that captures the grammatical relationships between tokens. I used a **Pratt parser** (top-down operator precedence parsing), which handles operator precedence elegantly without complex grammar rules. Each token type has an associated parsing function, and precedence is encoded as numeric values rather than grammar productions.

### Evaluator

Walks the AST recursively and evaluates each node to produce a value. This is a **tree-walking interpreter** — the simplest and most readable evaluation strategy. No bytecode, no virtual machine, no compilation step. The tradeoff is performance, but for understanding how evaluation works it's the clearest possible approach.

The evaluator maintains an **environment** — a map from identifiers to values — which is how variable bindings and closures work. When a function is defined, it captures its surrounding environment, which is what makes closures possible.

## What I Learned Implementing This in Rust Instead of Go

The book is written in Go. Rewriting it in Rust surfaced a set of interesting tensions that were genuinely valuable to work through:

**Recursive AST nodes** require `Box<T>` since Rust needs to know the size of types at compile time, and a recursive type has theoretically infinite size. Go handles this transparently with pointers; Rust makes you explicit about it — which I actually prefer.

**The environment for closures** needs shared ownership — a closure captures its enclosing environment, and multiple closures can reference the same environment. In Go this is handled automatically by the garbage collector. In Rust I used `Rc<RefCell<Environment>>`: `Rc` for shared ownership, `RefCell` for interior mutability. Understanding why this combination is needed was one of the most valuable things I got from this project.

**Pattern matching** on AST node types replaces Go's type switches. Rust's `match` on enums is a better fit — exhaustive matching means the compiler tells you if you've missed a case, which caught real bugs during development.

**Error handling** replaces Go's `(value, error)` return convention with Rust's `Result<T, E>`, which integrates cleanly with the `?` operator for propagation.

## Quick Start

```bash
cargo build --release

# Start the REPL
cargo run

# Run the test suite
cargo test
```

### REPL Example

```
Hello! This is the Monkey programming language, implemented in Rust.
>> let makeAdder = fn(x) { fn(y) { x + y } };
>> let addFive = makeAdder(5);
>> addFive(3);
8
```

## Project Structure

```
src/
├── lexer/
│   ├── mod.rs          # Lexer — source code to tokens
│   └── token.rs        # Token type definitions
├── parser/
│   ├── mod.rs          # Pratt parser — tokens to AST
│   └── ast.rs          # AST node definitions
├── evaluator/
│   ├── mod.rs          # Tree-walking evaluator
│   ├── environment.rs  # Variable bindings and closure scope
│   └── object.rs       # Value types (Integer, Boolean, Function, etc.)
├── repl/
│   └── mod.rs          # Read-Eval-Print Loop
└── main.rs             # Entry point
```

## References

- *Writing an Interpreter in Go* — Thorsten Ball
- *Writing a Compiler in Go* — Thorsten Ball (sequel, covering bytecode and a VM)
- [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) — matklad
