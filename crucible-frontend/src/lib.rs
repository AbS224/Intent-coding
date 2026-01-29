//! Crucible Frontend - WebAssembly Components
//! "Correct by Design, Not by Debugging"
//!
//! Licensed under the Crucible Engine License v2.0
//! See LICENSE file for full terms
//!
//! Provisional Patent Application: 63/928,407
//!
//! WebAssembly frontend components for the Crucible Engine.

use wasm_bindgen::prelude::*;
use crucible_core::{Constraint, ConstraintOperator, CompoundConstraint};

/// WebAssembly wrapper for constraint validation
#[wasm_bindgen]
pub struct ConstraintValidator {
    // Internal state
}

#[wasm_bindgen]
impl ConstraintValidator {
    /// Create a new validator instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> ConstraintValidator {
        ConstraintValidator {}
    }

    /// Validate a simple constraint: left_var >= right_val
    #[wasm_bindgen]
    pub fn validate_greater_equal(&self, left_var: i64, right_val: i64) -> bool {
        left_var >= right_val
    }

    /// Validate a simple constraint: left_var <= right_val
    #[wasm_bindgen]
    pub fn validate_less_equal(&self, left_var: i64, right_val: i64) -> bool {
        left_var <= right_val
    }

    /// Validate a simple constraint: left_var > right_val
    #[wasm_bindgen]
    pub fn validate_greater(&self, left_var: i64, right_val: i64) -> bool {
        left_var > right_val
    }

    /// Validate a simple constraint: left_var < right_val
    #[wasm_bindgen]
    pub fn validate_less(&self, left_var: i64, right_val: i64) -> bool {
        left_var < right_val
    }

    /// Validate a simple constraint: left_var == right_val
    #[wasm_bindgen]
    pub fn validate_equal(&self, left_var: i64, right_val: i64) -> bool {
        left_var == right_val
    }

    /// Validate a simple constraint: left_var != right_val
    #[wasm_bindgen]
    pub fn validate_not_equal(&self, left_var: i64, right_val: i64) -> bool {
        left_var != right_val
    }
}

/// Initialize console logging for WebAssembly
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Get version information
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Convert constraint operator to string
#[wasm_bindgen]
pub fn operator_to_string(op: i32) -> String {
    match op {
        0 => ">=".to_string(),
        1 => "<=".to_string(),
        2 => ">".to_string(),
        3 => "<".to_string(),
        4 => "==".to_string(),
        5 => "!=".to_string(),
        _ => "unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greater_equal_validation() {
        let validator = ConstraintValidator::new();
        assert!(validator.validate_greater_equal(10, 5));
        assert!(validator.validate_greater_equal(5, 5));
        assert!(!validator.validate_greater_equal(5, 10));
    }

    #[test]
    fn test_version() {
        let version = get_version();
        assert!(!version.is_empty());
    }
}
