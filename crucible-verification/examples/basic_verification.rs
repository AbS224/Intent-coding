//! Basic Z3 Verification Examples
//!
//! This example demonstrates the Z3 SMT solver integration for constraint verification.

use crucible_core::{Constraint, ConstraintOperator, CompoundConstraint};
use crucible_verification::{Z3Verifier, VerificationError};

fn main() {
    println!("=== Z3 SMT Solver Verification Examples ===\n");

    // Example 1: Simple satisfiable constraint
    println!("1. Simple Satisfiable Constraint");
    let verifier = Z3Verifier::new();
    
    let constraint = Constraint {
        left_variable: "balance".to_string(),
        operator: ConstraintOperator::GreaterThanOrEqual,
        right_value: "0".to_string(),
    };
    
    match verifier.verify_constraints(&[constraint.clone()]) {
        Ok(result) => {
            println!("  Constraint: balance >= 0");
            println!("  Result: SAT (satisfiable)");
            println!("  Model: {:?}", result.model);
        }
        Err(e) => println!("  Error: {:?}", e),
    }
    println!();

    // Example 2: Compound AND constraints
    println!("2. Compound AND Constraints (Bank Account Balance)");
    let compound_and = CompoundConstraint::And(vec![
        CompoundConstraint::Simple(Constraint {
            left_variable: "balance".to_string(),
            operator: ConstraintOperator::GreaterThanOrEqual,
            right_value: "0".to_string(),
        }),
        CompoundConstraint::Simple(Constraint {
            left_variable: "balance".to_string(),
            operator: ConstraintOperator::LessThanOrEqual,
            right_value: "10000".to_string(),
        }),
    ]);
    
    match verifier.verify_compound_constraints(&compound_and) {
        Ok(result) => {
            println!("  Constraints: balance >= 0 AND balance <= 10000");
            println!("  Result: SAT (satisfiable)");
        }
        Err(e) => println!("  Error: {:?}", e),
    }
    println!();

    // Example 3: Unsatisfiable constraints
    println!("3. Unsatisfiable Constraint (x > x)");
    let unsat_constraint = Constraint {
        left_variable: "x".to_string(),
        operator: ConstraintOperator::GreaterThan,
        right_value: "x".to_string(),
    };
    
    match verifier.verify_constraints(&[unsat_constraint]) {
        Ok(result) => println!("  Result: SAT"),
        Err(VerificationError::Unsatisfiable(msg)) => {
            println!("  Constraint: x > x");
            println!("  Result: UNSAT (unsatisfiable)");
            println!("  Proof: {}", msg);
        }
        Err(e) => println!("  Error: {:?}", e),
    }
    println!();

    // Example 4: Withdraw pattern (balance >= amount AND amount > 0)
    println!("4. Withdraw Pattern Verification");
    let withdraw_pattern = CompoundConstraint::And(vec![
        CompoundConstraint::Simple(Constraint {
            left_variable: "balance".to_string(),
            operator: ConstraintOperator::GreaterThanOrEqual,
            right_value: "amount".to_string(),
        }),
        CompoundConstraint::Simple(Constraint {
            left_variable: "amount".to_string(),
            operator: ConstraintOperator::GreaterThan,
            right_value: "0".to_string(),
        }),
    ]);
    
    match verifier.verify_compound_constraints(&withdraw_pattern) {
        Ok(result) => {
            println!("  Constraints:");
            println!("    - balance >= amount");
            println!("    - amount > 0");
            println!("  Result: SAT (satisfiable)");
            println!("  This pattern ensures safe withdrawals!");
        }
        Err(e) => println!("  Error: {:?}", e),
    }
    println!();

    // Example 5: SMT-LIB output
    println!("5. SMT-LIB Format Output");
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
    println!("  Generated SMT-LIB:");
    for line in smt_lib.lines() {
        println!("    {}", line);
    }
    println!();

    // Example 6: OR constraint pattern
    println!("6. OR Constraint (Access Control)");
    let access_control = CompoundConstraint::Or(vec![
        CompoundConstraint::Simple(Constraint {
            left_variable: "user_role".to_string(),
            operator: ConstraintOperator::Equal,
            right_value: "admin".to_string(),
        }),
        CompoundConstraint::Simple(Constraint {
            left_variable: "user_role".to_string(),
            operator: ConstraintOperator::Equal,
            right_value: "moderator".to_string(),
        }),
    ]);
    
    match verifier.verify_compound_constraints(&access_control) {
        Ok(result) => {
            println!("  Constraints:");
            println!("    - user_role == admin OR");
            println!("    - user_role == moderator");
            println!("  Result: SAT (satisfiable)");
        }
        Err(e) => println!("  Error: {:?}", e),
    }
    println!();

    println!("=== Verification Complete ===");
}
