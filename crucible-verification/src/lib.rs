//! Crucible Verification Engine - Z3 SMT Solver Integration
//! "Correct by Design, Not by Debugging"
//!
//! Licensed under the Crucible Engine License v2.0
//! See LICENSE file for full terms
//!
//! Provisional Patent Application: 63/928,407
//!
//! This module provides formal verification capabilities using the Z3 SMT solver.
//! It translates constraint expressions into Z3 formulas and performs satisfiability checking.

use crucible_core::{Constraint, ConstraintOperator, CompoundConstraint};
use thiserror::Error;
use z3::{ast::Ast, Config, Context, Solver};
use std::collections::HashMap;

/// Result type for verification operations
pub type VerificationResult<T> = std::result::Result<T, VerificationError>;

/// Errors that can occur during verification
#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("Z3 solver error: {0}")]
    SolverError(String),
    
    #[error("Constraint translation error: {0}")]
    TranslationError(String),
    
    #[error("Unsatisfiable constraints: {0}")]
    Unsatisfiable(String),
    
    #[error("Unknown constraint type")]
    UnknownConstraintType,
}

/// Result of a verification check
#[derive(Debug, Clone)]
pub struct VerificationResultOutput {
    pub satisfiable: bool,
    pub model: Option<HashMap<String, String>>,
    pub proof: Option<String>,
    pub constraints_count: usize,
}

/// Z3-backed verification engine
pub struct Z3Verifier {
    ctx: Context,
}

impl Z3Verifier {
    /// Create a new Z3 verifier
    pub fn new() -> Self {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        Self { ctx }
    }

    /// Verify a list of constraints
    pub fn verify_constraints(
        &self,
        constraints: &[Constraint],
    ) -> VerificationResult<VerificationResultOutput> {
        let solver = Solver::new(&self.ctx);
        
        // Track variables created
        let mut var_map: HashMap<String, z3::ast::Int> = HashMap::new();
        let mut constraints_count = 0;
        
        for constraint in constraints {
            let z3_expr = self.translate_constraint(constraint, &mut var_map, &solver)?;
            solver.assert(&z3_expr);
            constraints_count += 1;
        }

        // Check satisfiability
        match solver.check() {
            z3::SatResult::Sat => {
                let model = solver.get_model();
                let model_map = model.as_ref().map(|m| {
                    let mut map = HashMap::new();
                    for decl in m.get_decls() {
                        let name = decl.name().to_string();
                        let value = m.eval(&decl).unwrap();
                        map.insert(name, value.to_string());
                    }
                    map
                });

                Ok(VerificationResultOutput {
                    satisfiable: true,
                    model: model_map,
                    proof: Some("Constraints are satisfiable".to_string()),
                    constraints_count,
                })
            }
            z3::SatResult::Unsat => {
                // Try to get an unsat core for proof
                let core = solver.get_unsat_core();
                let proof = format!(
                    "Constraints are unsatisfiable. Unsat core size: {}",
                    core.len()
                );
                
                Err(VerificationError::Unsatisfiable(proof))
            }
            z3::SatResult::Unknown => {
                Err(VerificationError::SolverError(
                    "Z3 solver returned unknown result".to_string(),
                ))
            }
        }
    }

    /// Verify compound constraints (AND/OR/NOT trees)
    pub fn verify_compound_constraints(
        &self,
        compound: &CompoundConstraint,
    ) -> VerificationResult<VerificationResultOutput> {
        let solver = Solver::new(&self.ctx);
        let mut var_map: HashMap<String, z3::ast::Int> = HashMap::new();
        
        let z3_expr = self.translate_compound(compound, &mut var_map, &solver)?;
        solver.assert(&z3_expr);
        
        match solver.check() {
            z3::SatResult::Sat => {
                let model = solver.get_model();
                let model_map = model.as_ref().map(|m| {
                    let mut map = HashMap::new();
                    for decl in m.get_decls() {
                        let name = decl.name().to_string();
                        let value = m.eval(&decl).unwrap();
                        map.insert(name, value.to_string());
                    }
                    map
                });

                Ok(VerificationResultOutput {
                    satisfiable: true,
                    model: model_map,
                    proof: Some("Compound constraints are satisfiable".to_string()),
                    constraints_count: compound.count_constraints(),
                })
            }
            z3::SatResult::Unsat => {
                Err(VerificationError::Unsatisfiable(
                    "Compound constraints are unsatisfiable".to_string(),
                ))
            }
            z3::SatResult::Unknown => {
                Err(VerificationError::SolverError(
                    "Z3 solver returned unknown result".to_string(),
                ))
            }
        }
    }

    /// Translate a simple constraint to a Z3 expression
    fn translate_constraint<C: Into<Constraint>>(
        &self,
        constraint: &C,
        var_map: &mut HashMap<String, z3::ast::Int>,
        _solver: &Solver,
    ) -> VerificationResult<z3::ast::Bool> {
        let constraint = constraint.clone().into();
        
        // Get or create the left variable
        let left_var = var_map
            .entry(constraint.left_variable.clone())
            .or_insert_with(|| z3::ast::Int::new_const(&self.ctx, constraint.left_variable))
            .clone();

        // Parse the right value as an integer or variable
        let right_expr = self.parse_right_value(&constraint.right_value, var_map)?;

        // Map the operator to Z3 expression
        match constraint.operator {
            ConstraintOperator::GreaterThanOrEqual => Ok(left_var.ge(&right_expr)),
            ConstraintOperator::LessThanOrEqual => Ok(left_var.le(&right_expr)),
            ConstraintOperator::GreaterThan => Ok(left_var.gt(&right_expr)),
            ConstraintOperator::LessThan => Ok(left_var.lt(&right_expr)),
            ConstraintOperator::Equal => Ok(left_var._eq(&right_expr)),
            ConstraintOperator::NotEqual => Ok(left_var._eq(&right_expr).not()),
        }
    }

    /// Translate a compound constraint (AND/OR/NOT tree)
    fn translate_compound(
        &self,
        compound: &CompoundConstraint,
        var_map: &mut HashMap<String, z3::ast::Int>,
        solver: &Solver,
    ) -> VerificationResult<z3::ast::Bool> {
        match compound {
            CompoundConstraint::And(constraints) => {
                let z3_constraints: Vec<z3::ast::Bool> = constraints
                    .iter()
                    .map(|c| self.translate_compound(c, var_map, solver))
                    .collect::<Result<Vec<_>, _>>()?;
                
                let mut result = z3_constraints
                    .first()
                    .cloned()
                    .unwrap_or_else(|| z3::ast::Bool::from_bool(&self.ctx, true));
                
                for constraint in z3_constraints.into_iter().skip(1) {
                    result = result.and(&constraint);
                }
                
                Ok(result)
            }
            CompoundConstraint::Or(constraints) => {
                let z3_constraints: Vec<z3::ast::Bool> = constraints
                    .iter()
                    .map(|c| self.translate_compound(c, var_map, solver))
                    .collect::<Result<Vec<_>, _>>()?;
                
                let mut result = z3_constraints
                    .first()
                    .cloned()
                    .unwrap_or_else(|| z3::ast::Bool::from_bool(&self.ctx, false));
                
                for constraint in z3_constraints.into_iter().skip(1) {
                    result = result.or(&constraint);
                }
                
                Ok(result)
            }
            CompoundConstraint::Not(constraint) => {
                let inner = self.translate_compound(constraint, var_map, solver)?;
                Ok(inner.not())
            }
            CompoundConstraint::Simple(constraint) => {
                self.translate_constraint(constraint, var_map, solver)
            }
        }
    }

    /// Parse the right value (can be integer or variable reference)
    fn parse_right_value(
        &self,
        right_value: &str,
        var_map: &mut HashMap<String, z3::ast::Int>,
    ) -> VerificationResult<z3::ast::Int> {
        // Try to parse as integer
        if let Ok(int_val) = right_value.parse::<i64>() {
            return Ok(z3::ast::Int::from_i64(&self.ctx, int_val));
        }

        // Otherwise, treat as a variable
        let var = var_map
            .entry(right_value.to_string())
            .or_insert_with(|| z3::ast::Int::new_const(&self.ctx, right_value.to_string()))
            .clone();

        Ok(var)
    }

    /// Generate SMT-LIB format output for constraints
    pub fn generate_smt_lib(&self, constraints: &[Constraint]) -> String {
        let mut smt_lib = String::from("(set-logic QF_LIA)\n");
        smt_lib.push_str("(set-option :produce-models true)\n\n");
        
        // Track declared variables
        let mut declared_vars: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        for constraint in constraints {
            self.append_constraint_smt(constraint, &mut smt_lib, &mut declared_vars);
        }
        
        smt_lib.push_str("\n(check-sat)\n(get-model)\n");
        smt_lib
    }

    /// Append a constraint to SMT-LIB output
    fn append_constraint_smt(
        &self,
        constraint: &Constraint,
        output: &mut String,
        declared_vars: &mut std::collections::HashSet<String>,
    ) {
        // Declare left variable if not already declared
        if declared_vars.insert(constraint.left_variable.clone()) {
            output.push_str(&format!(
                "(declare-const {} Int)\n",
                constraint.left_variable
            ));
        }

        // Declare right variable if it's not a number
        if constraint.right_value.parse::<i64>().is_err() {
            if declared_vars.insert(constraint.right_value.clone()) {
                output.push_str(&format!(
                    "(declare-const {} Int)\n",
                    constraint.right_value
                ));
            }
        }

        // Add the constraint
        let op_str = match constraint.operator {
            ConstraintOperator::GreaterThanOrEqual => ">=",
            ConstraintOperator::LessThanOrEqual => "<=",
            ConstraintOperator::GreaterThan => ">",
            ConstraintOperator::LessThan => "<",
            ConstraintOperator::Equal => "=",
            ConstraintOperator::NotEqual => "distinct",
        };
        
        output.push_str(&format!(
            "(assert ({} {} {}))\n",
            op_str,
            constraint.left_variable,
            constraint.right_value
        ));
    }
}

impl Default for Z3Verifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to verify a single constraint
pub fn verify_single_constraint(constraint: &Constraint) -> VerificationResult<VerificationResultOutput> {
    let verifier = Z3Verifier::new();
    verifier.verify_constraints(&[constraint.clone()])
}

/// Check if two constraints are equivalent
pub fn check_equivalence(
    constraint1: &Constraint,
    constraint2: &Constraint,
) -> VerificationResult<bool> {
    let verifier = Z3Verifier::new();
    
    // Create solver with both constraints
    let solver = Solver::new(&verifier.ctx);
    let mut var_map: HashMap<String, z3::ast::Int> = HashMap::new();
    
    let z3_c1 = verifier.translate_constraint(constraint1, &mut var_map, &solver)?;
    let z3_c2 = verifier.translate_constraint(constraint2, &mut var_map, &solver)?;
    
    // Check if c1 AND NOT c2 is unsatisfiable (c1 implies c2)
    solver.assert(&z3_c1);
    solver.assert(&z3_c2.not());
    let c1_implies_c2 = solver.check() == z3::SatResult::Unsat;
    
    // Reset and check c2 AND NOT c1 (c2 implies c1)
    solver.reset();
    solver.assert(&z3_c2);
    solver.assert(&z3_c1.not());
    let c2_implies_c1 = solver.check() == z3::SatResult::Unsat;
    
    Ok(c1_implies_c2 && c2_implies_c1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crucible_core::{Constraint, ConstraintOperator};

    #[test]
    fn test_simple_satisfiable_constraint() {
        let verifier = Z3Verifier::new();
        
        let constraint = Constraint {
            left_variable: "x".to_string(),
            operator: ConstraintOperator::GreaterThanOrEqual,
            right_value: "0".to_string(),
        };
        
        let result = verifier.verify_constraints(&[constraint]);
        assert!(result.is_ok());
        assert!(result.unwrap().satisfiable);
    }

    #[test]
    fn test_simple_unsatisfiable_constraint() {
        let verifier = Z3Verifier::new();
        
        let constraint = Constraint {
            left_variable: "x".to_string(),
            operator: ConstraintOperator::GreaterThan,
            right_value: "x".to_string(),
        };
        
        let result = verifier.verify_constraints(&[constraint]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VerificationError::Unsatisfiable(_)));
    }

    #[test]
    fn test_compound_and_constraints() {
        let verifier = Z3Verifier::new();
        
        let compound = CompoundConstraint::And(vec![
            CompoundConstraint::Simple(Constraint {
                left_variable: "x".to_string(),
                operator: ConstraintOperator::GreaterThanOrEqual,
                right_value: "0".to_string(),
            }),
            CompoundConstraint::Simple(Constraint {
                left_variable: "x".to_string(),
                operator: ConstraintOperator::LessThanOrEqual,
                right_value: "10".to_string(),
            }),
        ]);
        
        let result = verifier.verify_compound_constraints(&compound);
        assert!(result.is_ok());
        assert!(result.unwrap().satisfiable);
    }

    #[test]
    fn test_compound_or_constraints() {
        let verifier = Z3Verifier::new();
        
        let compound = CompoundConstraint::Or(vec![
            CompoundConstraint::Simple(Constraint {
                left_variable: "x".to_string(),
                operator: ConstraintOperator::LessThan,
                right_value: "0".to_string(),
            }),
            CompoundConstraint::Simple(Constraint {
                left_variable: "x".to_string(),
                operator: ConstraintOperator::GreaterThan,
                right_value: "10".to_string(),
            }),
        ]);
        
        let result = verifier.verify_compound_constraints(&compound);
        assert!(result.is_ok());
        assert!(result.unwrap().satisfiable);
    }

    #[test]
    fn test_smt_lib_output() {
        let verifier = Z3Verifier::new();
        
        let constraints = vec![
            Constraint {
                left_variable: "balance".to_string(),
                operator: ConstraintOperator::GreaterThanOrEqual,
                right_value: "amount".to_string(),
            },
            Constraint {
                left_variable: "amount".to_string(),
                operator: ConstraintOperator::GreaterThan,
                right_value: "0".to_string(),
            },
        ];
        
        let smt_lib = verifier.generate_smt_lib(&constraints);
        assert!(smt_lib.contains("(declare-const balance Int)"));
        assert!(smt_lib.contains("(declare-const amount Int)"));
        assert!(smt_lib.contains("(assert (>= balance amount))"));
        assert!(smt_lib.contains("(assert (> amount 0))"));
    }
}
