//! LR(1) parsing table construction
//!
//! This module implements the construction of LR(1) parsing tables using
//! the canonical collection of LR(1) items.

use crate::{
    grammar::{Grammar, NonTerminal, Symbol},
    token::TokenType,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

/// An LR(1) item: a production with a dot position and a lookahead token
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LrItem {
    pub production_id: usize,
    pub dot_position: usize,
    pub lookahead: TokenType,
}

/// A state in the LR(1) automaton (a set of LR(1) items)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LrState {
    pub id: usize,
    pub items: HashSet<LrItem>,
    pub kernel_items: HashSet<LrItem>,
}

/// Action in the parsing table
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Shift(usize),  // Shift and go to state
    Reduce(usize), // Reduce by production
    Accept,        // Accept the input
}

/// LR(1) parsing table
pub struct LrTable {
    pub action_table: HashMap<(usize, TokenType), Action>,
    pub goto_table: HashMap<(usize, NonTerminal), usize>,
    pub states: Vec<LrState>,
    pub grammar: Grammar,
}

impl LrTable {
    /// Construct an LR(1) parsing table for the given grammar
    pub fn new(grammar: Grammar) -> Self {
        let mut table = Self {
            action_table: HashMap::new(),
            goto_table: HashMap::new(),
            states: Vec::new(),
            grammar,
        };

        table.construct_states();
        table.construct_tables();

        table
    }

    /// Construct the canonical collection of LR(1) states
    fn construct_states(&mut self) {
        // Create initial state with augmented start production
        let initial_item = LrItem {
            production_id: 0, // S' -> E
            dot_position: 0,
            lookahead: TokenType::Eof,
        };

        let initial_state = self.closure(vec![initial_item.clone()].into_iter().collect());
        let mut state_id = 0;
        let mut states = vec![LrState {
            id: state_id,
            kernel_items: vec![initial_item].into_iter().collect(),
            items: initial_state.clone(),
        }];
        state_id += 1;

        // Queue of states to process
        let mut queue = VecDeque::new();
        queue.push_back(0);

        while let Some(current_state_id) = queue.pop_front() {
            let current_items = states[current_state_id].items.clone();

            // Group items by the symbol after the dot
            let mut transitions: HashMap<Symbol, HashSet<LrItem>> = HashMap::new();

            for item in &current_items {
                if let Some(symbol) = self.symbol_after_dot(&item) {
                    transitions.entry(symbol).or_insert_with(HashSet::new);

                    // Create new item with dot moved forward
                    let new_item = LrItem {
                        production_id: item.production_id,
                        dot_position: item.dot_position + 1,
                        lookahead: item.lookahead,
                    };

                    transitions.get_mut(&symbol).unwrap().insert(new_item);
                }
            }

            // Create new states for each transition
            for (symbol, kernel) in transitions {
                let new_state_items = self.closure(kernel.clone());

                // Check if this state already exists
                let existing_state = states.iter().position(|s| s.items == new_state_items);

                if let Some(existing_id) = existing_state {
                    // State already exists
                    self.add_transition(current_state_id, symbol, existing_id);
                } else {
                    // Create new state
                    let new_state = LrState {
                        id: state_id,
                        kernel_items: kernel,
                        items: new_state_items,
                    };

                    states.push(new_state);
                    queue.push_back(state_id);

                    self.add_transition(current_state_id, symbol, state_id);

                    state_id += 1;
                }
            }
        }

        self.states = states;
    }

    /// Compute the closure of a set of LR(1) items
    fn closure(&self, kernel: HashSet<LrItem>) -> HashSet<LrItem> {
        let mut closure = kernel.clone();
        let mut changed = true;

        while changed {
            changed = false;
            let current_closure = closure.clone();

            for item in &current_closure {
                if let Some(Symbol::NonTerminal(non_terminal)) = self.symbol_after_dot(&item) {
                    // Compute lookaheads for new items
                    let beta = self.symbols_after_dot(&item, 1);
                    let lookaheads = if beta.is_empty() {
                        vec![item.lookahead].into_iter().collect()
                    } else {
                        let mut first_beta = self.grammar.first_of_sequence(&beta);
                        if beta.iter().all(|s| self.can_derive_epsilon(s)) {
                            first_beta.insert(item.lookahead);
                        }
                        first_beta
                    };

                    // Add items for all productions of the non-terminal
                    for production in self.grammar.productions_for(non_terminal) {
                        for &lookahead in &lookaheads {
                            let new_item = LrItem {
                                production_id: production.id,
                                dot_position: 0,
                                lookahead,
                            };

                            if closure.insert(new_item) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        closure
    }

    /// Get the symbol after the dot in an LR item
    fn symbol_after_dot(&self, item: &LrItem) -> Option<Symbol> {
        let production = &self.grammar.productions[item.production_id];
        production.rhs.get(item.dot_position).copied()
    }

    /// Get symbols after the dot (skipping the first n symbols)
    fn symbols_after_dot(&self, item: &LrItem, skip: usize) -> Vec<Symbol> {
        let production = &self.grammar.productions[item.production_id];
        production.rhs[item.dot_position + skip..].to_vec()
    }

    /// Check if a symbol can derive epsilon
    fn can_derive_epsilon(&self, _symbol: &Symbol) -> bool {
        // Our grammar has no epsilon productions
        false
    }

    /// Add a transition to the parsing tables
    fn add_transition(&mut self, from_state: usize, symbol: Symbol, to_state: usize) {
        match symbol {
            Symbol::Terminal(terminal) => {
                self.action_table
                    .insert((from_state, terminal), Action::Shift(to_state));
            }
            Symbol::NonTerminal(non_terminal) => {
                self.goto_table.insert((from_state, non_terminal), to_state);
            }
        }
    }

    /// Construct the action and goto tables from the states
    fn construct_tables(&mut self) {
        for state in &self.states.clone() {
            for item in &state.items {
                let production = &self.grammar.productions[item.production_id];

                if item.dot_position == production.rhs.len() {
                    // Item is complete (dot at end)
                    if production.id == 0 {
                        // Accept item: S' -> E •
                        self.action_table
                            .insert((state.id, TokenType::Eof), Action::Accept);
                    } else {
                        // Reduce item
                        self.action_table
                            .insert((state.id, item.lookahead), Action::Reduce(production.id));
                    }
                }
            }
        }
    }

    /// Get the action for a state and terminal
    pub fn action(&self, state: usize, terminal: TokenType) -> Option<&Action> {
        self.action_table.get(&(state, terminal))
    }

    /// Get the goto state for a state and non-terminal
    pub fn goto(&self, state: usize, non_terminal: NonTerminal) -> Option<usize> {
        self.goto_table.get(&(state, non_terminal)).copied()
    }

    /// Print the parsing table in a human-readable format
    pub fn print_table(&self) {
        println!("LR(1) Parsing Table:");
        println!("===================");

        // Print states
        for state in &self.states {
            println!("\nState {}:", state.id);
            for item in &state.items {
                println!("  {}", self.format_item(item));
            }

            // Print actions for this state
            let mut actions: Vec<_> = self
                .action_table
                .iter()
                .filter(|((s, _), _)| *s == state.id)
                .collect();
            actions.sort_by_key(|((_, t), _)| format!("{:?}", t));

            if !actions.is_empty() {
                println!("  Actions:");
                for ((_, terminal), action) in actions {
                    println!("    {:?} -> {:?}", terminal, action);
                }
            }

            // Print gotos for this state
            let mut gotos: Vec<_> = self
                .goto_table
                .iter()
                .filter(|((s, _), _)| *s == state.id)
                .collect();
            gotos.sort_by_key(|((_, nt), _)| format!("{}", nt));

            if !gotos.is_empty() {
                println!("  Gotos:");
                for ((_, non_terminal), target) in gotos {
                    println!("    {} -> {}", non_terminal, target);
                }
            }
        }
    }

    /// Format an LR item for display
    fn format_item(&self, item: &LrItem) -> String {
        let production = &self.grammar.productions[item.production_id];
        let mut result = format!("{} ->", production.lhs);

        for (i, symbol) in production.rhs.iter().enumerate() {
            if i == item.dot_position {
                result.push_str(" •");
            }
            result.push(' ');
            result.push_str(&format!("{}", symbol));
        }

        if item.dot_position == production.rhs.len() {
            result.push_str(" •");
        }

        result.push_str(&format!(", {:?}", item.lookahead));
        result
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Shift(state) => write!(f, "s{}", state),
            Action::Reduce(prod) => write!(f, "r{}", prod),
            Action::Accept => write!(f, "acc"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lr_table_construction() {
        let grammar = Grammar::new();
        let table = LrTable::new(grammar);

        // Check that initial state exists
        assert!(!table.states.is_empty());
        assert_eq!(table.states[0].id, 0);

        // Check that we have some actions
        assert!(!table.action_table.is_empty());

        // Check accept action exists
        let accept_actions: Vec<_> = table
            .action_table
            .values()
            .filter(|a| matches!(a, Action::Accept))
            .collect();
        assert_eq!(accept_actions.len(), 1);
    }

    #[test]
    fn test_closure() {
        let grammar = Grammar::new();
        let table = LrTable::new(grammar);

        // Test closure of initial item
        let initial_item = LrItem {
            production_id: 0,
            dot_position: 0,
            lookahead: TokenType::Eof,
        };

        let closure = table.closure(vec![initial_item.clone()].into_iter().collect());

        // Closure should contain more than just the kernel item
        assert!(closure.len() > 1);
        assert!(closure.contains(&initial_item));
    }
}

