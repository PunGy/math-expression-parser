//! Error types for the calculator parser

use crate::token::{Token, TokenType};
use std::error::Error;
use std::fmt;

/// Result type for parser operations
pub type ParseResult<T> = Result<T, ParseError>;

/// Errors that can occur during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Unexpected character during lexing
    UnexpectedChar {
        char: char,
        line: usize,
        column: usize,
    },

    /// Unexpected token during parsing
    UnexpectedToken {
        expected: Vec<TokenType>,
        found: Token,
    },

    /// Unexpected end of input
    UnexpectedEof { expected: Vec<TokenType> },

    /// Invalid number format
    InvalidNumber {
        lexeme: String,
        line: usize,
        column: usize,
    },

    /// Division by zero
    DivisionByZero { line: usize, column: usize },

    /// Generic syntax error
    SyntaxError {
        message: String,
        line: usize,
        column: usize,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedChar { char, line, column } => {
                write!(f, "Unexpected character '{}' at {}:{}", char, line, column)
            }

            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "Expected ")?;
                if expected.len() == 1 {
                    write!(f, "{:?}", expected[0])?;
                } else {
                    write!(f, "one of ")?;
                    for (i, token_type) in expected.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{:?}", token_type)?;
                    }
                }
                write!(f, ", found {}", found)
            }

            ParseError::UnexpectedEof { expected } => {
                write!(f, "Unexpected end of input, expected ")?;
                if expected.len() == 1 {
                    write!(f, "{:?}", expected[0])
                } else {
                    write!(f, "one of ")?;
                    for (i, token_type) in expected.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{:?}", token_type)?;
                    }
                    Ok(())
                }
            }

            ParseError::InvalidNumber {
                lexeme,
                line,
                column,
            } => {
                write!(f, "Invalid number '{}' at {}:{}", lexeme, line, column)
            }

            ParseError::DivisionByZero { line, column } => {
                write!(f, "Division by zero at {}:{}", line, column)
            }

            ParseError::SyntaxError {
                message,
                line,
                column,
            } => {
                write!(f, "Syntax error at {}:{}: {}", line, column, message)
            }
        }
    }
}

impl Error for ParseError {}

impl ParseError {
    /// Create an unexpected character error
    pub fn unexpected_char(char: char, line: usize, column: usize) -> Self {
        ParseError::UnexpectedChar { char, line, column }
    }

    /// Create an unexpected token error
    pub fn unexpected_token(expected: Vec<TokenType>, found: Token) -> Self {
        ParseError::UnexpectedToken { expected, found }
    }

    /// Create an unexpected EOF error
    pub fn unexpected_eof(expected: Vec<TokenType>) -> Self {
        ParseError::UnexpectedEof { expected }
    }

    /// Create an invalid number error
    pub fn invalid_number(lexeme: String, line: usize, column: usize) -> Self {
        ParseError::InvalidNumber {
            lexeme,
            line,
            column,
        }
    }

    /// Create a division by zero error
    pub fn division_by_zero(line: usize, column: usize) -> Self {
        ParseError::DivisionByZero { line, column }
    }

    /// Create a generic syntax error
    pub fn syntax_error(message: String, line: usize, column: usize) -> Self {
        ParseError::SyntaxError {
            message,
            line,
            column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ParseError::unexpected_char('$', 1, 5);
        assert_eq!(err.to_string(), "Unexpected character '$' at 1:5");

        let token = Token::new(TokenType::Plus, "+".to_string(), 2, 10);
        let err = ParseError::unexpected_token(vec![TokenType::Number], token);
        assert!(err.to_string().contains("Expected Number"));
        assert!(err.to_string().contains("found Plus"));
    }
}

