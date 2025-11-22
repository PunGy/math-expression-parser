pub mod ast;
pub mod error;
pub mod grammar;
pub mod lexer;
pub mod lr_table;
pub mod parser;
pub mod token;

pub use ast::{BinaryOp, Expr, UnaryOp};
pub use error::{ParseError, ParseResult};
pub use lexer::Lexer;
pub use parser::Parser;
pub use token::Token;

// Convenience function to parse and evaluate an expression
pub fn evaluate(input: &str) -> ParseResult<f64> {
    let mut parser = Parser::new();
    let expr = parser.parse(input)?;
    Ok(expr.evaluate())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_evaluation() {
        assert_eq!(evaluate("2 + 3").unwrap(), 5.0);
        assert_eq!(evaluate("2 * 3 + 4").unwrap(), 10.0);
        assert_eq!(evaluate("2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(evaluate("(2 + 3) * 4").unwrap(), 20.0);
    }
}
