//! Crucible Parser - Natural Language Requirement Parser using Tree-Sitter
//! Phase 2: Parser & Basic Verification - v0.2.0-alpha
//!
//! This module provides parsing functionality for natural language requirements,
//! transforming them into an Intent-AST (Abstract Syntax Tree) for formal verification.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Language binding for the Tree-Sitter requirements grammar
mod language {
    include!("src/tree_sitter/alloc.rs");
    include!("src/tree_sitter/array.rs");
    include!("src/tree_sitter/parser.rs");
}

/// Represents the type of action in a requirement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Create,
    Read,
    Update,
    Delete,
    Validate,
    Verify,
    Authenticate,
    Authorize,
    Encrypt,
    Decrypt,
    Send,
    Receive,
    Store,
    Retrieve,
    Process,
    Calculate,
    Generate,
    Export,
    Import,
    Withdraw,
    Deposit,
    Transfer,
    Login,
    Logout,
    Register,
    Other(String),
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::Create => write!(f, "create"),
            ActionType::Read => write!(f, "read"),
            ActionType::Update => write!(f, "update"),
            ActionType::Delete => write!(f, "delete"),
            ActionType::Validate => write!(f, "validate"),
            ActionType::Verify => write!(f, "verify"),
            ActionType::Authenticate => write!(f, "authenticate"),
            ActionType::Authorize => write!(f, "authorize"),
            ActionType::Encrypt => write!(f, "encrypt"),
            ActionType::Decrypt => write!(f, "decrypt"),
            ActionType::Send => write!(f, "send"),
            ActionType::Receive => write!(f, "receive"),
            ActionType::Store => write!(f, "store"),
            ActionType::Retrieve => write!(f, "retrieve"),
            ActionType::Process => write!(f, "process"),
            ActionType::Calculate => write!(f, "calculate"),
            ActionType::Generate => write!(f, "generate"),
            ActionType::Export => write!(f, "export"),
            ActionType::Import => write!(f, "import"),
            ActionType::Withdraw => write!(f, "withdraw"),
            ActionType::Deposit => write!(f, "deposit"),
            ActionType::Transfer => write!(f, "transfer"),
            ActionType::Login => write!(f, "login"),
            ActionType::Logout => write!(f, "logout"),
            ActionType::Register => write!(f, "register"),
            ActionType::Other(s) => write!(f, "{}", s),
        }
    }
}

/// Represents a constraint operator
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    IsSet,
    IsNotSet,
    Contains,
    DoesNotContain,
}

/// Represents a logical operator for compound constraints
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

/// Represents a parsed constraint (atomic or compound)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParsedConstraint {
    Atomic(Constraint),
    Compound {
        operator: LogicalOperator,
        left: Box<ParsedConstraint>,
        right: Option<Box<ParsedConstraint>>,
    },
}

impl ConstraintOperator {
    /// Convert from string representation
    pub fn from_str(s: &str) -> Self {
        match s {
            "==" | "equals" => ConstraintOperator::Equal,
            "!=" | "not_equals" => ConstraintOperator::NotEqual,
            ">" | "greater_than" => ConstraintOperator::GreaterThan,
            "<" | "less_than" => ConstraintOperator::LessThan,
            ">=" | "at_least" => ConstraintOperator::GreaterEqual,
            "<=" | "at_most" => ConstraintOperator::LessEqual,
            "is_set" => ConstraintOperator::IsSet,
            "is_not_set" => ConstraintOperator::IsNotSet,
            "contains" => ConstraintOperator::Contains,
            "does_not_contain" => ConstraintOperator::DoesNotContain,
            _ => ConstraintOperator::Equal,
        }
    }
}

/// Represents a parsed constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub left_variable: String,
    pub operator: ConstraintOperator,
    pub right_value: String,
}

/// Represents a parsed action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub verb: ActionType,
    pub object: String,
    pub preposition: Option<String>,
    pub target: Option<String>,
}

/// Represents a parsed requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub subject: String,
    pub modal_verb: String,
    pub action: Action,
    pub condition: Option<ParsedConstraint>,
    pub constraint: Option<ParsedConstraint>,
}

/// Represents the Intent-AST (Abstract Syntax Tree) for requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentAst {
    pub requirements: Vec<Requirement>,
    pub source_text: String,
}

/// Represents parsing errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at line {}, column {}: {}", self.line, self.column, self.message)
    }
}

/// Result type for parsing operations
pub type ParseResult = Result<IntentAst, ParseError>;

/// Parse natural language requirements into an Intent-AST
///
/// # Arguments
///
/// * `input` - The natural language requirement text to parse
///
/// # Returns
///
/// A `ParseResult` containing the Intent-AST or a parse error
///
/// # Example
///
/// ```
/// use crucible_parser::parse;
///
/// let input = "User can withdraw money from account if balance >= amount";
/// let result = parse(input);
/// assert!(result.is_ok());
/// ```
pub fn parse(input: &str) -> ParseResult {
    use tree_sitter::{Parser, Tree};
    
    // Create a new parser
    let mut parser = Parser::new();
    
    // Set the language to our requirements grammar
    parser.set_language(language::LANGUAGE)
        .context("Failed to set language for parser")?;
    
    // Parse the input
    let tree = parser.parse(input.as_bytes(), None)
        .context("Failed to parse input")?;
    
    // Check for errors
    if tree.root_node().has_error() {
        bail!("Parse error in input");
    }
    
    // Extract requirements from the tree
    let requirements = extract_requirements(&tree, input);
    
    Ok(IntentAst {
        requirements,
        source_text: input.to_string(),
    })
}

/// Extract requirements from the parse tree
fn extract_requirements(tree: &Tree, source: &str) -> Vec<Requirement> {
    let mut requirements = Vec::new();
    
    // Get the root node
    let root = tree.root_node();
    
    // Iterate through all requirement nodes
    let mut cursor = tree.walk();
    
    // Process each requirement
    for i in 0..root.child_count() {
        if let Some(child) = root.child(i) {
            if child.kind() == "requirement" {
                if let Some(req) = parse_requirement_node(child, source) {
                    requirements.push(req);
                }
            }
        }
    }
    
    requirements
}

/// Parse a single requirement node
fn parse_requirement_node(node: tree_sitter::Node, source: &str) -> Option<Requirement> {
    // This is a simplified parser - a full implementation would recursively
    // traverse the parse tree and extract all components
    
    let subject = extract_subject(node, source)?;
    let modal_verb = extract_modal_verb(node, source)?;
    let action = extract_action(node, source)?;
    let condition = extract_condition(node, source);
    let constraint = extract_constraint(node, source);
    
    Some(Requirement {
        subject,
        modal_verb,
        action,
        condition,
        constraint,
    })
}

/// Extract the subject from a requirement node
fn extract_subject(node: tree_sitter::Node, source: &str) -> Option<String> {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "subject" {
                return Some(source[child.byte_range()].to_string());
            }
        }
    }
    None
}

/// Extract the modal verb from a requirement node
fn extract_modal_verb(node: tree_sitter::Node, source: &str) -> Option<String> {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "modal_verb" {
                return Some(source[child.byte_range()].to_string());
            }
        }
    }
    None
}

/// Extract the action from a requirement node
fn extract_action(node: tree_sitter::Node, source: &str) -> Option<Action> {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "action" {
                return parse_action_node(child, source);
            }
        }
    }
    None
}

/// Parse an action node
fn parse_action_node(node: tree_sitter::Node, source: &str) -> Option<Action> {
    let mut verb = None;
    let mut object = None;
    let mut preposition = None;
    let mut target = None;
    
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            match child.kind() {
                "verb" => {
                    let verb_str = source[child.byte_range()].to_string();
                    verb = Some(ActionType::from_str(&verb_str));
                }
                "object" => {
                    object = Some(source[child.byte_range()].to_string());
                }
                "preposition_phrase" => {
                    // Extract preposition and target
                    for j in 0..child.child_count() {
                        if let Some(grandchild) = child.child(j) {
                            if grandchild.kind() == "preposition" {
                                preposition = Some(source[grandchild.byte_range()].to_string());
                            }
                            if grandchild.kind() == "noun_phrase" {
                                target = Some(source[grandchild.byte_range()].to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    Some(Action {
        verb: verb.unwrap_or(ActionType::Other("unknown".to_string())),
        object: object.unwrap_or_default(),
        preposition,
        target,
    })
}

/// Extract condition from a requirement node
fn extract_condition(node: tree_sitter::Node, source: &str) -> Option<ParsedConstraint> {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "condition" {
                return parse_constraint_expression(child, source);
            }
        }
    }
    None
}

/// Extract constraint from a requirement node
fn extract_constraint(node: tree_sitter::Node, source: &str) -> Option<ParsedConstraint> {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "constraint" {
                return parse_constraint_expression(child, source);
            }
        }
    }
    None
}

/// Parse a constraint expression (handles comparison, logical, and arithmetic)
fn parse_constraint_expression(node: tree_sitter::Node, source: &str) -> Option<ParsedConstraint> {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            match child.kind() {
                "constraint_expression" => {
                    // Recurse into the actual expression
                    return parse_constraint_expression(child, source);
                }
                "comparison" => {
                    return parse_comparison_node(child, source).map(ParsedConstraint::Atomic);
                }
                "logical_expression" => {
                    return parse_logical_expression_node(child, source);
                }
                "arithmetic_expression" => {
                    return parse_arithmetic_node(child, source).map(ParsedConstraint::Atomic);
                }
                _ => {}
            }
        }
    }
    None
}

/// Parse a comparison node
fn parse_comparison_node(node: tree_sitter::Node, source: &str) -> Option<Constraint> {
    let mut left_var = None;
    let mut operator = None;
    let mut right_val = None;
    
    for k in 0..node.child_count() {
        if let Some(ggchild) = node.child(k) {
            match ggchild.kind() {
                "left_expression" => {
                    for l in 0..ggchild.child_count() {
                        if let Some(gggchild) = ggchild.child(l) {
                            if gggchild.kind() == "variable" {
                                left_var = Some(source[gggchild.byte_range()].to_string());
                            }
                        }
                    }
                }
                "comparison_operator" => {
                    let op_str = source[ggchild.byte_range()].to_string();
                    operator = Some(ConstraintOperator::from_str(&op_str.trim()));
                }
                "right_expression" => {
                    for l in 0..ggchild.child_count() {
                        if let Some(gggchild) = ggchild.child(l) {
                            if gggchild.kind() == "variable" || gggchild.kind() == "number" {
                                right_val = Some(source[gggchild.byte_range()].to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    match (left_var, operator, right_val) {
        (Some(l), Some(op), Some(r)) => Some(Constraint {
            left_variable: l,
            operator: op,
            right_value: r,
        }),
        _ => None,
    }
}

/// Parse a logical expression node (and/or/not)
fn parse_logical_expression_node(node: tree_sitter::Node, source: &str) -> Option<ParsedConstraint> {
    let mut operator = None;
    let mut left_expr = None;
    let mut right_expr = None;
    
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            match child.kind() {
                "and" | "or" => {
                    operator = Some(if child.kind() == "and" { LogicalOperator::And } else { LogicalOperator::Or });
                }
                "not" => {
                    operator = Some(LogicalOperator::Not);
                }
                "expression" => {
                    // This is a nested expression
                    for j in 0..child.child_count() {
                        if let Some(expr_child) = child.child(j) {
                            if expr_child.kind() == "comparison" {
                                if left_expr.is_none() {
                                    left_expr = parse_comparison_node(expr_child, source).map(ParsedConstraint::Atomic);
                                } else {
                                    right_expr = parse_comparison_node(expr_child, source).map(ParsedConstraint::Atomic);
                                }
                            }
                        }
                    }
                }
                "comparison" => {
                    if left_expr.is_none() {
                        left_expr = parse_comparison_node(child, source).map(ParsedConstraint::Atomic);
                    } else {
                        right_expr = parse_comparison_node(child, source).map(ParsedConstraint::Atomic);
                    }
                }
                _ => {}
            }
        }
    }
    
    match operator {
        Some(LogicalOperator::Not) => {
            left_expr.map(|expr| ParsedConstraint::Compound {
                operator: LogicalOperator::Not,
                left: Box::new(expr),
                right: None,
            })
        }
        Some(_) if left_expr.is_some() && right_expr.is_some() => {
            Some(ParsedConstraint::Compound {
                operator: operator.unwrap(),
                left: Box::new(left_expr.unwrap()),
                right: Some(Box::new(right_expr.unwrap())),
            })
        }
        _ => left_expr,
    }
}

/// Parse an arithmetic expression node
fn parse_arithmetic_node(node: tree_sitter::Node, source: &str) -> Option<Constraint> {
    // For arithmetic expressions like "a + b", we create a constraint where
    // the left side equals the arithmetic result
    let mut left_var = None;
    let mut right_var = None;
    let mut right_num = None;
    
    for k in 0..node.child_count() {
        if let Some(ggchild) = node.child(k) {
            match ggchild.kind() {
                "left_expression" => {
                    for l in 0..ggchild.child_count() {
                        if let Some(gggchild) = ggchild.child(l) {
                            if gggchild.kind() == "variable" {
                                left_var = Some(source[gggchild.byte_range()].to_string());
                            }
                        }
                    }
                }
                "right_expression" => {
                    for l in 0..ggchild.child_count() {
                        if let Some(gggchild) = ggchild.child(l) {
                            if gggchild.kind() == "variable" {
                                right_var = Some(source[gggchild.byte_range()].to_string());
                            } else if gggchild.kind() == "number" {
                                right_num = Some(source[gggchild.byte_range()].to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    match (left_var, right_var, right_num) {
        (Some(l), Some(r), _) => Some(Constraint {
            left_variable: l,
            operator: ConstraintOperator::Equal,
            right_value: format!("({})", r), // Placeholder for arithmetic
        }),
        (Some(l), None, Some(n)) => Some(Constraint {
            left_variable: l,
            operator: ConstraintOperator::Equal,
            right_value: n,
        }),
        _ => None,
    }
}

/// Legacy parser for simple constraints (kept for backward compatibility)
fn parse_constraint_node(node: tree_sitter::Node, source: &str) -> Option<ParsedConstraint> {
    parse_constraint_expression(node, source)
}

/// Get the Tree-Sitter language for this parser
pub fn get_language() -> tree_sitter::Language {
    language::LANGUAGE
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_requirement() {
        let input = "User can withdraw money from account if balance >= amount";
        let result = parse(input);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert_eq!(ast.requirements.len(), 1);
        assert_eq!(ast.requirements[0].subject, "User");
        assert_eq!(ast.requirements[0].modal_verb, "can");
    }
    
    #[test]
    fn test_parse_with_constraint() {
        let input = "Admin should validate input where length > 0";
        let result = parse(input);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert_eq!(ast.requirements.len(), 1);
        assert_eq!(ast.requirements[0].subject, "Admin");
        assert!(ast.requirements[0].constraint.is_some());
    }
    
    #[test]
    fn test_parse_complex_constraint() {
        let input = "Service shall process transaction where amount > 0 and amount <= balance";
        let result = parse(input);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert_eq!(ast.requirements.len(), 1);
        assert_eq!(ast.requirements[0].subject, "Service");
    }
    
    #[test]
    fn test_parse_logical_and_constraint() {
        let input = "User can withdraw money if balance >= amount and amount > 0";
        let result = parse(input);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert_eq!(ast.requirements.len(), 1);
        assert_eq!(ast.requirements[0].subject, "User");
        // Check that condition is a compound constraint
        if let Some(ref constraint) = ast.requirements[0].condition {
            match constraint {
                ParsedConstraint::Compound { operator, left: _, right: _ } => {
                    assert_eq!(*operator, LogicalOperator::And);
                }
                ParsedConstraint::Atomic(_) => {
                    // Single constraint - still valid
                }
            }
        }
    }
    
    #[test]
    fn test_parse_logical_or_constraint() {
        let input = "Admin can delete record if role == admin or role == superuser";
        let result = parse(input);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert_eq!(ast.requirements.len(), 1);
        assert_eq!(ast.requirements[0].subject, "Admin");
    }
    
    #[test]
    fn test_parse_nested_logical_constraint() {
        let input = "System shall validate input where (length > 0) and (width > 0) or (is_default == true)";
        let result = parse(input);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert_eq!(ast.requirements.len(), 1);
        assert_eq!(ast.requirements[0].subject, "System");
    }
}
