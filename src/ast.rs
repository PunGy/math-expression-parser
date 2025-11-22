//! Abstract Syntax Tree definitions for calculator expressions

use std::fmt;

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Negate,
}

/// Expression nodes in the AST
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Numeric literal
    Number(f64),
    
    /// Binary operation
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    
    /// Unary operation
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
}

impl Expr {
    /// Create a number expression
    pub fn number(value: f64) -> Self {
        Expr::Number(value)
    }
    
    /// Create a binary expression
    pub fn binary(left: Expr, op: BinaryOp, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
    
    /// Create a unary expression
    pub fn unary(op: UnaryOp, operand: Expr) -> Self {
        Expr::Unary {
            op,
            operand: Box::new(operand),
        }
    }
    
    /// Evaluate the expression to a numeric value
    pub fn evaluate(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            
            Expr::Binary { left, op, right } => {
                let left_val = left.evaluate();
                let right_val = right.evaluate();
                
                match op {
                    BinaryOp::Add => left_val + right_val,
                    BinaryOp::Subtract => left_val - right_val,
                    BinaryOp::Multiply => left_val * right_val,
                    BinaryOp::Divide => left_val / right_val,
                }
            }
            
            Expr::Unary { op, operand } => {
                let val = operand.evaluate();
                
                match op {
                    UnaryOp::Negate => -val,
                }
            }
        }
    }
    
    /// Pretty-print the expression
    pub fn pretty_print(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            
            Expr::Binary { left, op, right } => {
                format!(
                    "({} {} {})",
                    left.pretty_print(),
                    op.symbol(),
                    right.pretty_print()
                )
            }
            
            Expr::Unary { op, operand } => {
                format!("({}{})", op.symbol(), operand.pretty_print())
            }
        }
    }
    
    /// Get the depth of the expression tree
    pub fn depth(&self) -> usize {
        match self {
            Expr::Number(_) => 1,
            
            Expr::Binary { left, right, .. } => {
                1 + left.depth().max(right.depth())
            }
            
            Expr::Unary { operand, .. } => {
                1 + operand.depth()
            }
        }
    }
}

impl BinaryOp {
    /// Get the symbol representation of the operator
    pub fn symbol(&self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
        }
    }
    
    /// Get the precedence of the operator (higher number = higher precedence)
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Add | BinaryOp::Subtract => 1,
            BinaryOp::Multiply | BinaryOp::Divide => 2,
        }
    }
    
    /// Check if the operator is left-associative
    pub fn is_left_associative(&self) -> bool {
        true // All our operators are left-associative
    }
}

impl UnaryOp {
    /// Get the symbol representation of the operator
    pub fn symbol(&self) -> &'static str {
        match self {
            UnaryOp::Negate => "-",
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_expr_creation() {
        let expr = Expr::binary(
            Expr::number(2.0),
            BinaryOp::Add,
            Expr::number(3.0),
        );
        
        assert_eq!(expr.evaluate(), 5.0);
        assert_eq!(expr.pretty_print(), "(2 + 3)");
    }
    
    #[test]
    fn test_complex_expression() {
        // (2 + 3) * 4
        let expr = Expr::binary(
            Expr::binary(
                Expr::number(2.0),
                BinaryOp::Add,
                Expr::number(3.0),
            ),
            BinaryOp::Multiply,
            Expr::number(4.0),
        );
        
        assert_eq!(expr.evaluate(), 20.0);
        assert_eq!(expr.pretty_print(), "((2 + 3) * 4)");
        assert_eq!(expr.depth(), 3);
    }
    
    #[test]
    fn test_unary_expression() {
        let expr = Expr::unary(
            UnaryOp::Negate,
            Expr::number(5.0),
        );
        
        assert_eq!(expr.evaluate(), -5.0);
        assert_eq!(expr.pretty_print(), "(-5)");
    }
    
    #[test]
    fn test_operator_precedence() {
        assert!(BinaryOp::Multiply.precedence() > BinaryOp::Add.precedence());
        assert_eq!(BinaryOp::Add.precedence(), BinaryOp::Subtract.precedence());
    }
}
