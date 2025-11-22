# LR(1) Parser for Calculator Expressions

This module implements a complete LR(1) parser for arithmetic expressions in Rust

## Overview

The parser implements a classic calculator grammar with proper operator precedence and associativity:

```
E → E + T | E - T | T
T → T * F | T / F | F  
F → ( E ) | number | - F
```

## Architecture

### Components

1. **Token Module** (`token.rs`)
   - Defines token types for the lexer
   - Implements traits for comparison and hashing

2. **Lexer Module** (`lexer.rs`)
   - Tokenizes input strings into a stream of tokens
   - Handles numbers (including decimals), operators, and parentheses
   - Tracks line and column positions for error reporting

3. **AST Module** (`ast.rs`)
   - Defines the Abstract Syntax Tree representation
   - Implements expression evaluation
   - Provides pretty-printing capabilities

4. **Grammar Module** (`grammar.rs`)
   - Defines the context-free grammar
   - Computes FIRST and FOLLOW sets
   - Manages productions and symbols

5. **LR Table Module** (`lr_table.rs`)
   - Constructs the canonical collection of LR(1) items
   - Builds ACTION and GOTO tables
   - Implements the LR automaton construction algorithm

6. **Parser Module** (`parser.rs`)
   - Implements the LR parsing algorithm
   - Converts parse trees to AST
   - Provides error recovery and reporting

7. **Error Module** (`error.rs`)
   - Defines error types for parsing failures
   - Implements detailed error messages with position information

## Features

### Supported Operations
- Addition (`+`)
- Subtraction (`-`)
- Multiplication (`*`)
- Division (`/`)
- Unary negation (`-`)
- Parentheses for grouping

### Key Characteristics
- **Proper Precedence**: Multiplication and division have higher precedence than addition and subtraction
- **Left Associativity**: All binary operators are left-associative
- **Error Recovery**: Detailed error messages with line and column information
- **Decimal Support**: Handles both integer and floating-point numbers

## Usage

```bash
# Run the demo
cargo run demo

# Start the REPL
cargo run repl

# Evaluate an expression directly
cargo run "2 + 3 * 4"
```

## Implementation Details

### LR(1) Parser Construction

The parser uses the canonical LR(1) construction algorithm:

1. **Item Sets**: Each parser state contains a set of LR(1) items (production + dot position + lookahead)
2. **Closure Operation**: Computes the closure of item sets by adding items for non-terminals after the dot
3. **State Transitions**: Builds transitions between states based on grammar symbols
4. **Table Generation**: Creates ACTION and GOTO tables from the automaton

### Parsing Algorithm

The parser uses a stack-based algorithm:

1. Push initial state onto stack
2. For each input token:
   - Look up action in ACTION table
   - Shift: Push symbol and new state
   - Reduce: Pop symbols, apply production, push result
   - Accept: Parsing complete
3. Convert parse tree to AST

### Error Handling

The parser provides detailed error information:
- Unexpected character errors during lexing
- Unexpected token errors during parsing
- Position tracking (line and column)
- Expected token suggestions

## Design Patterns

### Rust Best Practices Demonstrated

1. **Type Safety**: Strong typing with enums for tokens, AST nodes, and parser actions
2. **Error Handling**: Result types with custom error enums
3. **Zero-Copy Parsing**: Efficient string handling without unnecessary allocations
4. **Iterator Pattern**: Lexer implements Iterator trait
5. **Builder Pattern**: Grammar and table construction
6. **Visitor Pattern**: AST evaluation and pretty-printing

### Performance Considerations

- **Table-Driven Parsing**: O(n) parsing time for n tokens
- **Efficient Data Structures**: HashMaps for fast table lookups
- **Minimal Allocations**: Reuses buffers where possible

## Testing

Run the test suite:

```bash
cargo test
```

The tests cover:
- Basic arithmetic operations
- Operator precedence
- Parentheses handling
- Error cases
- Edge cases (negative numbers, decimals)

## References

- [Dragon Book](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools) - Comprehensive compiler construction reference
- [LR Parsing](https://en.wikipedia.org/wiki/LR_parser) - Wikipedia article on LR parsing
