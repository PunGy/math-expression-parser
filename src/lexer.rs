//! Lexer for tokenizing calculator expressions

use crate::{
    token::{Token, TokenType},
    error::{ParseError, ParseResult},
};

/// Lexer for tokenizing input strings
pub struct Lexer {
    input: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer for the given input
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> ParseResult<Vec<Token>> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }
            
            let token = self.next_token()?;
            tokens.push(token);
        }
        
        tokens.push(Token::eof(self.line, self.column));
        Ok(tokens)
    }
    
    /// Get the next token
    pub fn next_token(&mut self) -> ParseResult<Token> {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Ok(Token::eof(self.line, self.column));
        }
        
        let start_column = self.column;
        let ch = self.advance();
        
        let token_type = match ch {
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '0'..='9' => return self.number(start_column),
            _ => return Err(ParseError::unexpected_char(ch, self.line, start_column)),
        };
        
        Ok(Token::new(
            token_type,
            ch.to_string(),
            self.line,
            start_column,
        ))
    }
    
    /// Parse a number token
    fn number(&mut self, start_column: usize) -> ParseResult<Token> {
        let start = self.current - 1;
        
        // Consume integer part
        while self.peek().map_or(false, |ch| ch.is_ascii_digit()) {
            self.advance();
        }
        
        // Check for decimal part
        if self.peek() == Some('.') && self.peek_next().map_or(false, |ch| ch.is_ascii_digit()) {
            self.advance(); // Consume '.'
            
            // Consume fractional part
            while self.peek().map_or(false, |ch| ch.is_ascii_digit()) {
                self.advance();
            }
        }
        
        let lexeme: String = self.input[start..self.current].iter().collect();
        
        match lexeme.parse::<f64>() {
            Ok(value) => Ok(Token {
                token_type: TokenType::Number,
                lexeme,
                value: Some(value),
                line: self.line,
                column: start_column,
            }),
            Err(_) => Err(ParseError::invalid_number(lexeme, self.line, start_column)),
        }
    }
    
    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                _ => break,
            }
        }
    }
    
    /// Check if we've reached the end of input
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
    
    /// Peek at the current character without consuming it
    fn peek(&self) -> Option<char> {
        self.input.get(self.current).copied()
    }
    
    /// Peek at the next character without consuming it
    fn peek_next(&self) -> Option<char> {
        self.input.get(self.current + 1).copied()
    }
    
    /// Advance to the next character and return the current one
    fn advance(&mut self) -> char {
        let ch = self.input[self.current];
        self.current += 1;
        self.column += 1;
        ch
    }
}

/// Iterator implementation for the lexer
impl Iterator for Lexer {
    type Item = ParseResult<Token>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            None
        } else {
            Some(self.next_token())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_simple() {
        let mut lexer = Lexer::new("2 + 3");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 4); // 2, +, 3, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, Some(2.0));
        assert_eq!(tokens[1].token_type, TokenType::Plus);
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].value, Some(3.0));
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }
    
    #[test]
    fn test_tokenize_complex() {
        let mut lexer = Lexer::new("(2.5 + 3) * -4");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 9); // (, 2.5, +, 3, ), *, -, 4, EOF
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, Some(2.5));
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[4].token_type, TokenType::RightParen);
        assert_eq!(tokens[5].token_type, TokenType::Star);
        assert_eq!(tokens[6].token_type, TokenType::Minus);
        assert_eq!(tokens[7].token_type, TokenType::Number);
        assert_eq!(tokens[8].token_type, TokenType::Eof);
    }
    
    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("2 +\n  3");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 3);
        assert_eq!(tokens[2].line, 2);
        assert_eq!(tokens[2].column, 3);
    }
    
    #[test]
    fn test_invalid_character() {
        let mut lexer = Lexer::new("2 @ 3");
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        if let Err(ParseError::UnexpectedChar { char, line, column }) = result {
            assert_eq!(char, '@');
            assert_eq!(line, 1);
            assert_eq!(column, 3);
        } else {
            panic!("Expected UnexpectedChar error");
        }
    }
}
