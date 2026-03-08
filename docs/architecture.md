# Monkey Interpreter Architecture

This document provides a deep dive into the Monkey interpreter's implementation, explaining how each component works and how they interact.

## Pipeline Overview

```
Source Code → Lexer → Tokens → Parser → AST → Evaluator → Output
```

Each stage transforms data into a form the next stage can consume. Errors at any stage halt execution.

---

## 1. Lexer (`src/lexer/mod.rs`)

The lexer is the first stage. It scans source code character-by-character and produces a stream of tokens.

### Key Types

- **`Lexer`** (`lexer/mod.rs:9-14`): The main lexer struct
  - `input`: Source code as bytes (enables easy character peeking)
  - `position`: Current reading position
  - `read_position`: Position of next character (for lookahead)
  - `ch`: Current character (None when at end of input)

### How It Works

1. **`next_token()`** (`lexer/mod.rs:41-100`): The main entry point. Returns the next token and advances the lexer.

2. **Two-pointer technique**: The lexer uses `position` and `read_position` to implement one-character lookahead. This is essential for distinguishing:
   - `=` (Assign) from `==` (Equal)
   - `!` (Bang) from `!=` (Not equal)

3. **Keyword lookup** (`lexer/mod.rs:16-26`): A static `HashMap` maps keywords like `let`, `fn`, `if` to their token variants. Identifiers not in this map become `Token::Ident`.

4. **Token types** (`token/mod.rs:15-54`): The `Token` enum covers all syntactic elements:
   - Keywords: `Let`, `Function`, `If`, `Else`, `Return`, `True`, `False`
   - Operators: `Plus`, `Minus`, `Bang`, `Asterisk`, `Slash`
   - Delimiters: `Lparen`, `Rparen`, `Lbrace`, `Rbrace`, etc.
   - Literals: `Ident(String)`, `Int(String)`

### Design Decision: Bytes Instead of Chars

The lexer uses `Vec<u8>` instead of `&str` to iterate over characters. This is a pragmatic choice—Monkey only supports ASCII, so each byte corresponds to one character, making indexing O(1) instead of O(n) for UTF-8 chars.

---

## 2. Token and Precedence (`src/token/mod.rs`)

### Precedence Levels (`token/mod.rs:3-13`)

```rust
pub enum Precedence {
    Lowest = 1,
    Equals = 2,      // == !=
    Lessgreater = 3, // < >
    Sum = 4,         // + -
    Product = 5,    // * /
    Prefix = 6,     // - !
    Call = 7,       // ()
}
```

Precedence is encoded as numeric values where higher numbers bind more tightly. This is critical for Pratt parsing.

### Why Precedence Matters

In `5 + 3 * 2`, multiplication should bind tighter than addition. The parser uses precedence to determine how to group:
- `5 + (3 * 2)` — not `(5 + 3) * 2`

---

## 3. Parser (`src/parser/mod.rs`)

The parser transforms tokens into an Abstract Syntax Tree (AST). This implementation uses **Pratt parsing** (top-down operator precedence parsing).

### Key Types

- **`Parser`** (`parser/mod.rs:20-26`): Holds state for parsing
  - `lexer`: Source of tokens
  - `current_token`, `peek_token`: Two-token lookahead (standard for Pratt)
  - `prefix_fns`: Map of prefix parsers (unary operators, literals)
  - `infix_fns`: Map of infix parsers (binary operators, function calls)

### Pratt Parsing Explained

Traditional parsers use grammars (BNF/EBNF). Pratt parsing uses a function table instead:

1. **Prefix parsers**: Handle tokens that can start an expression
   - `5` → `parse_integer_literal`
   - `foo` → `parse_identifier`
   - `-5` → `parse_prefix_expression` (operator + recursive parse)

2. **Infix parsers**: Handle tokens that appear in the middle of expressions
   - `5 + 3` → after parsing `5`, `+` triggers `parse_infix`
   - `foo(arg)` → after parsing `foo`, `(` triggers `parse_call`

### The Parsing Loop (`parser/mod.rs:356-376`)

```rust
fn parse_expresion(&mut self, precedence: Precedence) -> Result<Expression, MonkeyError> {
    // 1. Parse the left side using a prefix parser
    let prefix = self.prefix_fn(&self.current_token)?;
    let mut left_expr = prefix(self)?;

    // 2. Continue while next token binds tighter
    while self.peek_token != Token::Semicolon && precedence < self.peek_token.precedence() {
        let infix = self.infix_fns[&peek_token];
        self.next_token();
        left_expr = infix(self, left_expr)?;
    }

    Ok(left_expr)
}
```

This elegantly handles precedence: when parsing `5 + 3 * 2`:
1. Parse `5` (precedence Lowest)
2. See `+`, precedence Sum > Lowest, so continue
3. Parse `3` as right side of `+`
4. See `*`, precedence Product > Sum, so continue
5. Parse `2` as right side of `*`
6. Result: `5 + (3 * 2)`

### Registered Parsers (`parser/mod.rs:39-94`)

| Token Type | Parser Function | Purpose |
|------------|-----------------|---------|
| `Ident` | `parse_identifier` | Variable references |
| `Int` | `parse_integer_literal` | Integer literals |
| `Bang/Minus` | `parse_prefix_expression` | Unary operators |
| `True/False` | `parse_boolean` | Boolean literals |
| `Lparen` | `parse_grouped` | Parenthesized expressions |
| `If` | `parse_if` | Conditionals |
| `Function` | `parse_function` | Function literals |
| `+/etc` | `parse_infix` | Binary operators |
| `Lparen` (after expr) | `parse_call` | Function calls |

---

## 4. AST (`src/ast/mod.rs`)

The AST represents parsed code as a tree of nodes.

### Core Types

- **`Program`** (`ast/mod.rs:32-35`): Root node containing all statements
- **`Statement`** (`ast/mod.rs:12-18`): Executable statements
  - `Let`: Variable binding (`let x = 5;`)
  - `Return`: Return statement (`return 5;`)
  - `Block`: Grouped statements (`{ ... }`)
  - `Expression`: Expression statement (just evaluates, no binding)

- **`Expression`** (`ast/mod.rs:20-30`): Values that produce results
  - `Identifier`: Variable reference (`x`)
  - `IntegerLiteral`: Integer (`42`)
  - `BooleanLiteral`: Boolean (`true`/`false`)
  - `Prefix`: Unary operation (`-5`, `!true`)
  - `Infix`: Binary operation (`5 + 3`)
  - `If`: Conditional (`if (x > 0) { ... } else { ... }`)
  - `FunctionLiteral`: Function definition (`fn(x) { ... }`)
  - `Call`: Function call (`add(5, 3)`)

### String Representation

Every AST node implements `string()` to enable debugging and REPL feedback. This recursively formats the entire program as a string.

---

## 5. Evaluator (`src/evaluator/mod.rs`)

The evaluator walks the AST and computes values. This is a **tree-walking interpreter**—no bytecode, no compilation.

### Evaluation Entry Point

**`eval()`** (`evaluator/mod.rs:16-66`): Main dispatch function. Pattern matches on the node type and routes to specialized evaluation functions.

### Key Evaluation Functions

| Function | Purpose |
|----------|---------|
| `eval_program` | Evaluate all statements in a program |
| `eval_block_statements` | Evaluate statements in a block, handle early returns |
| `eval_identifier` | Look up variable in environment |
| `eval_prefix` | Handle unary operators (`-`, `!`) |
| `eval_infix` | Handle binary operators (`+`, `-`, `*`, `/`, `==`, etc.) |
| `eval_if` | Conditionals—evaluate condition, pick branch |
| `apply_function` | Execute function calls with proper scope |

### Truthiness (`evaluator/mod.rs:129-134`)

```rust
pub fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Boolean(b) => b,
        _ => true,  // Everything else is truthy!
    }
}
```

Monkey's truthiness: only `false` is falsy. `0`, `null`, and empty strings are all truthy.

---

## 6. Object System (`src/evaluator/object.rs`)

Runtime values in the interpreter.

### Object Types (`object/mod.rs:15-22`)

```rust
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Return(Box<Object>),  // Wraps return values
    Function(Function),   // Functions are first-class
    Null,
}
```

### Function Type (`object/mod.rs:8-13`)

```rust
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,  // Captured closure environment
}
```

The `env` field is what enables closures—functions carry their defining scope with them.

---

## 7. Environment (`src/evaluator/environment.rs`)

The environment maps variable names to values and implements lexical scoping.

### Key Fields (`environment.rs:5-9`)

```rust
pub struct Environment {
    store: HashMap<String, Object>,  // Local bindings
    outer: Option<Box<Environment>>, // Parent scope (for closures)
}
```

### Variable Lookup (`environment.rs:19-23`)

```rust
pub fn get(&self, value: &str) -> Option<&Object> {
    self.store
        .get(value)
        .or_else(|| self.outer.as_ref()?.get(value))  // Chain to outer scope
}
```

This implements **lexical scoping**: if a variable isn't found locally, the evaluator checks the outer environment (and its outer, recursively).

### Closure Implementation (`environment.rs:29-37`)

```rust
pub fn extend_func_env(&self, func: &Function, args: Vec<Object>) -> Environment {
    let mut extended = Environment::new_enclosed(self);

    for (parameter, arg) in func.parameters.iter().zip(args) {
        extended.set(parameter.token.literal(), arg);
    }

    extended
}
```

When a function is called:
1. Create a new environment enclosed by the function's captured environment
2. Bind parameters to arguments
3. Evaluate the function body in this extended environment

This is how closures work: the function sees variables from its definition site, not its call site.

---

## 8. REPL (`src/repl.rs`)

The Read-Eval-Print Loop ties everything together:

```rust
pub fn repl_loop() {
    let mut env = Environment::default();  // Shared environment across REPL sessions

    loop {
        // Read
        io::stdin().read_line(&mut input)?;

        // Parse
        let token_processor = Lexer::new(&input);
        let mut parser = Parser::new(token_processor);
        let program = parser.parse_program()?;

        // Evaluate
        let obj = eval(&Node::Program(program), &mut env)?;

        // Print
        println!("{}", obj.inspect());
    }
}
```

Each input line creates a fresh lexer/parser but shares the environment, so variables persist across REPL commands.

---

## Error Handling

All errors flow through `MonkeyError` (`error/mod.rs:3-10`):

```rust
pub enum MonkeyError {
    Lexer(String),   // Tokenization errors
    Parser(String), // Syntactic errors
    Evaluator(String), // Runtime errors (unknown operators, etc.)
}
```

The REPL catches errors at each stage and prints them, allowing the user to continue.

---

## Design Decisions Summary

| Decision | Rationale |
|----------|-----------|
| **Pratt parsing** | Elegant handling of operator precedence without complex grammar |
| **Tree-walking eval** | Simple, readable, easy to understand (chosen for learning) |
| **Byte-based lexer** | O(1) indexing for ASCII-only language |
| **Closure via environment capture** | Standard approach for first-class functions |
| **Two-token lookahead** | Standard Pratt parsing pattern |
| **Static keyword map** | O(1) keyword vs identifier distinction |
