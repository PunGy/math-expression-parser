use calculator::{evaluate, Parser};
use std::env;

pub fn run_example() {
    println!("=== LR Parser Calculator Example ===\n");

    let expressions = vec![
        "42",
        "2 + 3",
        "2 + 3 * 4",
        "(2 + 3) * 4",
        "10 / 2 - 3",
        "-5 + 3",
        "-(2 + 3) * 4",
        "((1 + 2) * (3 + 4)) / 5",
        "2.5 * 4 + 1.5",
    ];

    for expr_str in expressions {
        println!("Expression: {}", expr_str);

        match evaluate(expr_str) {
            Ok(result) => {
                println!("  Result: {}", result);

                // Also show the AST
                let mut parser = Parser::new();
                if let Ok(ast) = parser.parse(expr_str) {
                    println!("  AST: {}", ast.pretty_print());
                    println!("  Tree depth: {}", ast.depth());
                }
            }
            Err(e) => {
                println!("  Error: {}", e);
            }
        }

        println!();
    }

    println!("=== Error Handling Examples ===\n");

    let error_expressions = vec![
        ("2 +", "Missing operand"),
        ("2 + + 3", "Unexpected operator"),
        ("(2 + 3", "Missing closing parenthesis"),
        ("2 @ 3", "Invalid character"),
        ("2 + (3 * )", "Missing operand in parentheses"),
    ];

    for (expr_str, description) in error_expressions {
        println!("Expression: {} ({})", expr_str, description);

        match evaluate(expr_str) {
            Ok(result) => {
                println!("  Unexpected success: {}", result);
            }
            Err(e) => {
                println!("  Expected error: {}", e);
            }
        }

        println!();
    }

    // Show parsing table information
    println!("=== Parser Information ===\n");

    let parser = Parser::new();
    println!("Parser created with LR(1) parsing table");
    println!("Use parser.print_table() to see the full parsing table");

    println!("\n=== Step-by-Step Parsing Example ===\n");
    demonstrate_parsing_steps();
}

fn demonstrate_parsing_steps() {
    use calculator::{Lexer, Token};

    let input = "2 + 3 * 4";
    println!("Parsing: {}", input);

    // Step 1: Lexical analysis
    println!("\n1. Lexical Analysis:");
    let mut lexer = Lexer::new(input);
    match lexer.tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("   {}", token);
            }
        }
        Err(e) => {
            println!("   Lexer error: {}", e);
            return;
        }
    }

    // Step 2: Parsing
    println!("\n2. Parsing:");
    let mut parser = Parser::new();
    match parser.parse(input) {
        Ok(ast) => {
            println!("   Success! AST: {}", ast.pretty_print());

            // Step 3: Evaluation
            println!("\n3. Evaluation:");
            println!("   {} = {}", ast.pretty_print(), ast.evaluate());
        }
        Err(e) => {
            println!("   Parser error: {}", e);
        }
    }
}

/// Interactive calculator REPL
pub fn run_repl() {
    use std::io::{self, Write};

    println!("=== LR Parser Calculator REPL ===");
    println!("Enter expressions to evaluate, or 'quit' to exit.");
    println!("Type 'help' for available commands.\n");

    let mut parser = Parser::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let input = input.trim();

        match input {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "help" => {
                print_help();
            }
            "table" => {
                parser.print_table();
            }
            "" => {
                // Empty input, just continue
            }
            _ => match parser.parse(input) {
                Ok(ast) => {
                    println!("AST: {}", ast.pretty_print());
                    println!("Result: {}", ast.evaluate());
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            },
        }
    }
}

fn print_help() {
    println!("\nAvailable commands:");
    println!("  <expression>  - Evaluate a mathematical expression");
    println!("  help         - Show this help message");
    println!("  table        - Show the LR parsing table");
    println!("  quit/exit    - Exit the REPL");
    println!("\nSupported operators:");
    println!("  +  Addition");
    println!("  -  Subtraction (binary and unary)");
    println!("  *  Multiplication");
    println!("  /  Division");
    println!("  () Parentheses for grouping");
    println!("\nExamples:");
    println!("  2 + 3");
    println!("  2 + 3 * 4");
    println!("  (2 + 3) * 4");
    println!("  -5 + 3");
    println!();
}

/// Main entry point
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "repl" => run_repl(),
            "demo" => run_example(),
            expr => {
                // Evaluate the expression directly
                match evaluate(expr) {
                    Ok(result) => println!("{} = {}", expr, result),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    } else {
        println!("LR Parser Calculator");
        println!("Usage:");
        println!("  {} <expression>     - Evaluate an expression", args[0]);
        println!("  {} repl            - Start interactive REPL", args[0]);
        println!("  {} demo            - Run demonstration", args[0]);
        println!("\nExample: {} \"2 + 3 * 4\"", args[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use calculator::evaluate;

    #[test]
    fn test_example_expressions() {
        // Test that all example expressions parse correctly
        let expressions = vec![
            ("42", 42.0),
            ("2 + 3", 5.0),
            ("2 + 3 * 4", 14.0),
            ("(2 + 3) * 4", 20.0),
            ("10 / 2 - 3", 2.0),
            ("-5 + 3", -2.0),
            ("-(2 + 3) * 4", -20.0),
            ("((1 + 2) * (3 + 4)) / 5", 4.2),
            ("2.5 * 4 + 1.5", 11.5),
        ];

        for (expr, expected) in expressions {
            let result = evaluate(expr).unwrap();
            assert!(
                (result - expected).abs() < 0.0001,
                "Expression '{}' evaluated to {} but expected {}",
                expr,
                result,
                expected
            );
        }
    }
}

