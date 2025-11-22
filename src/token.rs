//! Token definitions for the calculator lexer

use std::fmt;

/// Token types for the calculator language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Literals
    Number,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,

    // Delimiters
    LeftParen,
    RightParen,

    // Special
    Eof,
}

/// A token with its type, lexeme, and position information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub value: Option<f64>,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Create a new token
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        let value = if token_type == TokenType::Number {
            lexeme.parse::<f64>().ok()
        } else {
            None
        };

        Self {
            token_type,
            lexeme,
            value,
            line,
            column,
        }
    }

    /// Create a number token with a specific value
    pub fn number(value: f64, line: usize, column: usize) -> Self {
        Self {
            token_type: TokenType::Number,
            lexeme: value.to_string(),
            value: Some(value),
            line,
            column,
        }
    }

    /// Create an EOF token
    pub fn eof(line: usize, column: usize) -> Self {
        Self {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            value: None,
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.token_type)?;
        if !self.lexeme.is_empty() {
            write!(f, "({})", self.lexeme)?;
        }
        write!(f, " at {}:{}", self.line, self.column)
    }
}

impl TokenType {
    /// Get the precedence of an operator token
    pub fn precedence(&self) -> Option<u8> {
        match self {
            TokenType::Plus | TokenType::Minus => Some(1),
            TokenType::Star | TokenType::Slash => Some(2),
            _ => None,
        }
    }

    /// Check if this token type is a binary operator
    pub fn is_binary_op(&self) -> bool {
        matches!(
            self,
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash
        )
    }

    /// Check if this token type is a unary operator
    pub fn is_unary_op(&self) -> bool {
        matches!(self, TokenType::Minus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
        assert_eq!(token.token_type, TokenType::Plus);
        assert_eq!(token.lexeme, "+");
        assert_eq!(token.value, None);
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 5);
    }

    #[test]
    fn test_number_token() {
        let token = Token::number(42.5, 2, 10);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, Some(42.5));
        assert_eq!(token.lexeme, "42.5");
    }

    #[test]
    fn test_precedence() {
        assert_eq!(TokenType::Plus.precedence(), Some(1));
        assert_eq!(TokenType::Star.precedence(), Some(2));
        assert_eq!(TokenType::Number.precedence(), None);
    }
}

