//! Crucible Engine Core - "Correct by Design, Not by Debugging"
//!
//! Licensed under the Crucible Engine License v2.0
//! See LICENSE file for full terms
//!
//! Provisional Patent Application: 63/928,407

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Operators for constraint expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintOperator {
    GreaterThanOrEqual,
    LessThanOrEqual,
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}

/// A simple constraint expression: `left_variable operator right_value`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constraint {
    pub left_variable: String,
    pub operator: ConstraintOperator,
    pub right_value: String,
}

impl From<&str> for Constraint {
    fn from(s: &str) -> Self {
        Self {
            left_variable: s.to_string(),
            operator: ConstraintOperator::GreaterThanOrEqual,
            right_value: "0".to_string(),
        }
    }
}

impl From<String> for Constraint {
    fn from(s: String) -> Self {
        Self {
            left_variable: s,
            operator: ConstraintOperator::GreaterThanOrEqual,
            right_value: "0".to_string(),
        }
    }
}

/// A constraint that can be simple or compound (AND/OR/NOT tree)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompoundConstraint {
    And(Vec<CompoundConstraint>),
    Or(Vec<CompoundConstraint>),
    Not(Box<CompoundConstraint>),
    Simple(Constraint),
}

impl CompoundConstraint {
    /// Count the number of simple constraints in the tree
    pub fn count_constraints(&self) -> usize {
        match self {
            CompoundConstraint::And(constraints) | CompoundConstraint::Or(constraints) => {
                constraints.iter().map(|c| c.count_constraints()).sum()
            }
            CompoundConstraint::Not(constraint) => constraint.count_constraints(),
            CompoundConstraint::Simple(_) => 1,
        }
    }
}

impl From<Constraint> for CompoundConstraint {
    fn from(c: Constraint) -> Self {
        CompoundConstraint::Simple(c)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub id: Uuid,
    pub content: String,
    pub verified: bool,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentAst {
    pub id: Uuid,
    pub requirements: Vec<Requirement>,
    pub correctness_score: f64,
}

impl IntentAst {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            requirements: Vec::new(),
            correctness_score: 0.0,
        }
    }

    pub fn add_requirement(&mut self, content: String) {
        let req = Requirement {
            id: Uuid::new_v4(),
            content,
            verified: false,
            constraints: Vec::new(),
        };
        self.requirements.push(req);
        self.update_score();
    }

    fn update_score(&mut self) {
        if self.requirements.is_empty() {
            self.correctness_score = 0.0;
            return;
        }
        
        let verified = self.requirements.iter().filter(|r| r.verified).count();
        self.correctness_score = (verified as f64 / self.requirements.len() as f64) * 100.0;
    }
}

impl Default for IntentAst {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Type-Aware Schema Registry (v0.1.5-alpha)
// =============================================================================

/// Data types for type-aware code generation with overflow protection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    /// Unsigned 64-bit integer (common for balances)
    Uint64,
    /// Unsigned 32-bit integer
    Uint32,
    /// Signed 64-bit integer
    Int64,
    /// Signed 32-bit integer
    Int32,
    /// String type
    String,
    /// Boolean type
    Bool,
    /// Fixed-point decimal (for financial precision)
    Decimal,
    /// Custom type with range constraints
    Custom {
        name: String,
        range_min: Option<i128>,
        range_max: Option<i128>,
    },
}

/// Maps a variable name to its data type for overflow-safe code generation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    /// Variable name -> Data type mapping
    pub fields: std::collections::HashMap<String, DataType>,
    /// Optional documentation for each field
    pub documentation: std::collections::HashMap<String, String>,
    /// Traceability ID linking to Z3 SMT solver run
    pub traceability_id: String,
}

impl Schema {
    /// Create a new empty schema
    pub fn new(traceability_id: String) -> Self {
        Self {
            fields: std::collections::HashMap::new(),
            documentation: std::collections::HashMap::new(),
            traceability_id,
        }
    }

    /// Add a field to the schema
    pub fn add_field(&mut self, name: String, data_type: DataType, docs: Option<String>) {
        self.fields.insert(name.clone(), data_type);
        if let Some(doc) = docs {
            self.documentation.insert(name, doc);
        }
    }

    /// Get the data type for a variable, defaulting to Int32
    pub fn get_type(&self, name: &str) -> DataType {
        self.fields.get(name).cloned().unwrap_or(DataType::Int32)
    }

    /// Check if a field requires overflow-safe arithmetic
    pub fn requires_overflow_protection(&self, name: &str) -> bool {
        matches!(
            self.get_type(name),
            DataType::Uint64 | DataType::Uint32 | DataType::Int64 | DataType::Int32
        )
    }
}

/// Arithmetic operators for overflow-safe operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl ArithmeticOperator {
    /// Get the Rust operator symbol
    pub fn rust_symbol(&self) -> &'static str {
        match self {
            ArithmeticOperator::Add => "+",
            ArithmeticOperator::Subtract => "-",
            ArithmeticOperator::Multiply => "*",
            ArithmeticOperator::Divide => "/",
        }
    }

    /// Get the symbol for display
    pub fn symbol(&self) -> &'static str {
        self.rust_symbol()
    }
}