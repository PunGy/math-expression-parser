//! Grammar definition for the calculator language
//!
//! This module defines the context-free grammar used by the LR parser.
//! The grammar is designed to handle operator precedence and associativity correctly.

use crate::token::TokenType;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Non-terminal symbols in the grammar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonTerminal {
    Start,  // S' -> E
    Expr,   // E -> E + T | E - T | T
    Term,   // T -> T * F | T / F | F
    Factor, // F -> ( E ) | number | - F
}

/// Symbol in the grammar (either terminal or non-terminal)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    Terminal(TokenType),
    NonTerminal(NonTerminal),
}

/// A production rule in the grammar
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Production {
    pub id: usize,
    pub lhs: NonTerminal,
    pub rhs: Vec<Symbol>,
}

/// The complete grammar for the calculator
pub struct Grammar {
    pub productions: Vec<Production>,
    pub start_symbol: NonTerminal,
    terminals: HashSet<TokenType>,
    non_terminals: HashSet<NonTerminal>,
    first_sets: HashMap<Symbol, HashSet<TokenType>>,
    follow_sets: HashMap<NonTerminal, HashSet<TokenType>>,
}

impl Grammar {
    /// Create the calculator grammar
    pub fn new() -> Self {
        let productions = vec![
            // 0: S' -> E
            Production {
                id: 0,
                lhs: NonTerminal::Start,
                rhs: vec![Symbol::NonTerminal(NonTerminal::Expr)],
            },
            // 1: E -> E + T
            Production {
                id: 1,
                lhs: NonTerminal::Expr,
                rhs: vec![
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(TokenType::Plus),
                    Symbol::NonTerminal(NonTerminal::Term),
                ],
            },
            // 2: E -> E - T
            Production {
                id: 2,
                lhs: NonTerminal::Expr,
                rhs: vec![
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(TokenType::Minus),
                    Symbol::NonTerminal(NonTerminal::Term),
                ],
            },
            // 3: E -> T
            Production {
                id: 3,
                lhs: NonTerminal::Expr,
                rhs: vec![Symbol::NonTerminal(NonTerminal::Term)],
            },
            // 4: T -> T * F
            Production {
                id: 4,
                lhs: NonTerminal::Term,
                rhs: vec![
                    Symbol::NonTerminal(NonTerminal::Term),
                    Symbol::Terminal(TokenType::Star),
                    Symbol::NonTerminal(NonTerminal::Factor),
                ],
            },
            // 5: T -> T / F
            Production {
                id: 5,
                lhs: NonTerminal::Term,
                rhs: vec![
                    Symbol::NonTerminal(NonTerminal::Term),
                    Symbol::Terminal(TokenType::Slash),
                    Symbol::NonTerminal(NonTerminal::Factor),
                ],
            },
            // 6: T -> F
            Production {
                id: 6,
                lhs: NonTerminal::Term,
                rhs: vec![Symbol::NonTerminal(NonTerminal::Factor)],
            },
            // 7: F -> ( E )
            Production {
                id: 7,
                lhs: NonTerminal::Factor,
                rhs: vec![
                    Symbol::Terminal(TokenType::LeftParen),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(TokenType::RightParen),
                ],
            },
            // 8: F -> number
            Production {
                id: 8,
                lhs: NonTerminal::Factor,
                rhs: vec![Symbol::Terminal(TokenType::Number)],
            },
            // 9: F -> - F
            Production {
                id: 9,
                lhs: NonTerminal::Factor,
                rhs: vec![
                    Symbol::Terminal(TokenType::Minus),
                    Symbol::NonTerminal(NonTerminal::Factor),
                ],
            },
        ];

        let terminals = vec![
            TokenType::Number,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Star,
            TokenType::Slash,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Eof,
        ]
        .into_iter()
        .collect();

        let non_terminals = vec![
            NonTerminal::Start,
            NonTerminal::Expr,
            NonTerminal::Term,
            NonTerminal::Factor,
        ]
        .into_iter()
        .collect();

        let mut grammar = Self {
            productions,
            start_symbol: NonTerminal::Start,
            terminals,
            non_terminals,
            first_sets: HashMap::new(),
            follow_sets: HashMap::new(),
        };

        grammar.compute_first_sets();
        grammar.compute_follow_sets();

        grammar
    }

    /// Get all productions for a given non-terminal
    pub fn productions_for(&self, non_terminal: NonTerminal) -> Vec<&Production> {
        self.productions
            .iter()
            .filter(|p| p.lhs == non_terminal)
            .collect()
    }

    /// Compute FIRST sets for all symbols
    fn compute_first_sets(&mut self) {
        // Initialize FIRST sets for terminals
        for &terminal in &self.terminals {
            self.first_sets.insert(
                Symbol::Terminal(terminal),
                vec![terminal].into_iter().collect(),
            );
        }

        // Initialize empty FIRST sets for non-terminals
        for &non_terminal in &self.non_terminals {
            self.first_sets
                .insert(Symbol::NonTerminal(non_terminal), HashSet::new());
        }

        // Iteratively compute FIRST sets
        let mut changed = true;
        while changed {
            changed = false;

            for production in &self.productions.clone() {
                let lhs_symbol = Symbol::NonTerminal(production.lhs);
                let mut first_set = self.first_sets[&lhs_symbol].clone();
                let old_size = first_set.len();

                // Add FIRST(rhs) to FIRST(lhs)
                for symbol in &production.rhs {
                    let symbol_first = self.first_sets[symbol].clone();
                    first_set.extend(symbol_first);

                    // If symbol can't derive epsilon, stop
                    if !self.can_derive_epsilon(symbol) {
                        break;
                    }
                }

                if first_set.len() > old_size {
                    changed = true;
                    self.first_sets.insert(lhs_symbol, first_set);
                }
            }
        }
    }

    /// Compute FOLLOW sets for all non-terminals
    fn compute_follow_sets(&mut self) {
        // Initialize empty FOLLOW sets
        for &non_terminal in &self.non_terminals {
            self.follow_sets.insert(non_terminal, HashSet::new());
        }

        // Add EOF to FOLLOW(start_symbol)
        self.follow_sets
            .get_mut(&self.start_symbol)
            .unwrap()
            .insert(TokenType::Eof);

        // Iteratively compute FOLLOW sets
        let mut changed = true;
        while changed {
            changed = false;

            for production in &self.productions.clone() {
                for (i, symbol) in production.rhs.iter().enumerate() {
                    if let Symbol::NonTerminal(non_terminal) = symbol {
                        let mut follow_set = self.follow_sets[non_terminal].clone();
                        let old_size = follow_set.len();

                        // Add FIRST(β) to FOLLOW(A) for production X -> αAβ
                        let beta = &production.rhs[i + 1..];
                        if !beta.is_empty() {
                            let first_beta = self.first_of_sequence(beta);
                            follow_set.extend(first_beta);
                        }

                        // If β can derive epsilon or β is empty, add FOLLOW(X) to FOLLOW(A)
                        if beta.is_empty() || self.sequence_can_derive_epsilon(beta) {
                            let follow_lhs = self.follow_sets[&production.lhs].clone();
                            follow_set.extend(follow_lhs);
                        }

                        if follow_set.len() > old_size {
                            changed = true;
                            self.follow_sets.insert(*non_terminal, follow_set);
                        }
                    }
                }
            }
        }
    }

    /// Get the FIRST set for a symbol
    pub fn first(&self, symbol: &Symbol) -> &HashSet<TokenType> {
        &self.first_sets[symbol]
    }

    /// Get the FOLLOW set for a non-terminal
    pub fn follow(&self, non_terminal: NonTerminal) -> &HashSet<TokenType> {
        &self.follow_sets[&non_terminal]
    }

    /// Compute FIRST set for a sequence of symbols
    pub fn first_of_sequence(&self, symbols: &[Symbol]) -> HashSet<TokenType> {
        let mut result = HashSet::new();

        for symbol in symbols {
            result.extend(self.first_sets[symbol].iter());

            if !self.can_derive_epsilon(symbol) {
                break;
            }
        }

        result
    }

    /// Check if a symbol can derive epsilon (empty string)
    fn can_derive_epsilon(&self, _symbol: &Symbol) -> bool {
        // Our grammar doesn't have epsilon productions
        false
    }

    /// Check if a sequence of symbols can derive epsilon
    fn sequence_can_derive_epsilon(&self, symbols: &[Symbol]) -> bool {
        symbols.iter().all(|s| self.can_derive_epsilon(s))
    }
}

impl Default for Grammar {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NonTerminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NonTerminal::Start => write!(f, "S'"),
            NonTerminal::Expr => write!(f, "E"),
            NonTerminal::Term => write!(f, "T"),
            NonTerminal::Factor => write!(f, "F"),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Terminal(t) => write!(f, "{:?}", t),
            Symbol::NonTerminal(nt) => write!(f, "{}", nt),
        }
    }
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ->", self.lhs)?;
        for symbol in &self.rhs {
            write!(f, " {}", symbol)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_creation() {
        let grammar = Grammar::new();
        assert_eq!(grammar.productions.len(), 10);
        assert_eq!(grammar.start_symbol, NonTerminal::Start);
    }

    #[test]
    fn test_first_sets() {
        let grammar = Grammar::new();

        // FIRST(Factor) should contain Number, LeftParen, and Minus
        let first_factor = grammar.first(&Symbol::NonTerminal(NonTerminal::Factor));
        assert!(first_factor.contains(&TokenType::Number));
        assert!(first_factor.contains(&TokenType::LeftParen));
        assert!(first_factor.contains(&TokenType::Minus));

        // FIRST(Term) should be the same as FIRST(Factor)
        let first_term = grammar.first(&Symbol::NonTerminal(NonTerminal::Term));
        assert_eq!(first_term, first_factor);
    }

    #[test]
    fn test_follow_sets() {
        let grammar = Grammar::new();

        // FOLLOW(Start) should contain EOF
        let follow_start = grammar.follow(NonTerminal::Start);
        assert!(follow_start.contains(&TokenType::Eof));

        // FOLLOW(Expr) should contain EOF, Plus, Minus, and RightParen
        let follow_expr = grammar.follow(NonTerminal::Expr);
        assert!(follow_expr.contains(&TokenType::Eof));
        assert!(follow_expr.contains(&TokenType::RightParen));
        assert!(follow_expr.contains(&TokenType::Plus));
        assert!(follow_expr.contains(&TokenType::Minus));
    }
}

