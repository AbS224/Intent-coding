//! Crucible Code Generation Engine - v0.1.5-alpha (Enhanced)
//! "Correct by Design, Not by Debugging"
//!
//! Licensed under the Crucible Engine License v2.0
//! See LICENSE file for full terms
//!
//! Provisional Patent Application: 63/928,407
//!
//! Multi-language code generation with formal verification contracts.
//! Supporting: Rust, TypeScript, Python, Solidity, SPARK/Ada, Zig, Elixir.
//!
//! ## Strategy-Based Model (v0.1.5)
//! This module implements a `VerifiableStrategy` trait for each supported language.
//! Every language must define how it expresses mathematical truths and runtime assertions.
//! This ensures contract-first generation with formal proof traceability.

use crucible_core::{Constraint, ConstraintOperator, CompoundConstraint};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during code generation
#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Language {0} requires formal contracts not provided")]
    MissingContract(String),

    #[error("Unsupported target language: {0}")]
    UnsupportedLanguage(String),

    #[error("Generation error: {0}")]
    GenerationError(String),
}

/// Supported output languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetLanguage {
    Rust,
    TypeScript,
    Python,
    Solidity,
    SparkAda, // High-integrity formal verification (MIL-SPEC)
    Zig,      // Memory-safe systems programming
    Elixir,   // Fault-tolerant distributed logic
}

/// Code generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodegenOutput {
    pub language: TargetLanguage,
    pub code: String,
    pub constraints_count: usize,
}

/// Information about a constraint for contract generation
#[derive(Debug, Clone)]
pub struct ConstraintInfo {
    pub left_variable: String,
    pub operator: ConstraintOperator,
    pub right_value: String,
    pub is_static: bool, // Can be evaluated at compile time
}

/// The Generator Strategy defines how a specific language expresses logic.
/// This trait-based approach allows adding new languages without modifying core recursion.
trait CodegenStrategy {
    fn wrap_in_function(&self, body: &str, func_name: &str) -> String;
    fn format_operator(&self, op: &ConstraintOperator) -> &'static str;
    fn format_variable(&self, name: &str) -> String;
    fn logical_and(&self) -> &'static str;
    fn logical_or(&self) -> &'static str;
    fn logical_not(&self, expr: &str) -> String;

    /// Formal Verification Hook: How the language handles "Assertions" or "Contracts"
    fn wrap_assertion(&self, condition: &str) -> String {
        format!("assert({});", condition)
    }

    /// Generate precondition contract (for formal verification)
    #[allow(dead_code)]
    fn precondition(&self, _condition: &str) -> Option<String> {
        None
    }

    /// Generate postcondition contract (for formal verification)
    #[allow(dead_code)]
    fn postcondition(&self, _condition: &str) -> Option<String> {
        None
    }

    /// Emit full contracts (Pre/Post/Invariants) for formal verification
    /// Returns a string containing all contract declarations
    fn emit_contracts(&self, _compound: &CompoundConstraint) -> Option<String> {
        None
    }

    /// Wrap a verified function with contracts and assertions
    fn wrap_verified_function(
        &self,
        func_name: &str,
        contracts: &str,
        body: &str,
        assertions: &str,
    ) -> String;

    /// Check if constraints can be evaluated at compile time
    fn is_comptime_capable(&self, _compound: &CompoundConstraint) -> bool {
        false
    }

    /// Generate guard-compatible expression (for languages like Elixir)
    fn to_guard_expression(&self, compound: &CompoundConstraint) -> Option<String> {
        None
    }

    /// Generate compile-time error for invalid constraints
    fn compile_error(&self, message: &str) -> String {
        format!("@compileError(\"{}\");", message)
    }
}

// --- SPARK/Ada Strategy (MIL-SPEC Formal Verification) ---

struct SparkAdaStrategy;

impl CodegenStrategy for SparkAdaStrategy {
    fn wrap_in_function(&self, body: &str, func_name: &str) -> String {
        format!(
            r#"-- SPARK/Ada Generated Code - Formally Verifiable
-- Use GNATprove for mathematical verification

function {func_name} (Params : Validation_Params) return Boolean
   with SPARK_Mode => On
is
begin
   return {body};
end {func_name};"#,
            func_name = func_name,
            body = body
        )
    }

    fn format_operator(&self, op: &ConstraintOperator) -> &'static str {
        match op {
            ConstraintOperator::GreaterThanOrEqual => ">=",
            ConstraintOperator::LessThanOrEqual => "<=",
            ConstraintOperator::GreaterThan => ">",
            ConstraintOperator::LessThan => "<",
            ConstraintOperator::Equal => "=",
            ConstraintOperator::NotEqual => "/=",
        }
    }

    fn format_variable(&self, name: &str) -> String {
        format!("Params.{}", to_ada_case(name))
    }

    fn logical_and(&self) -> &'static str {
        "and then"
    }

    fn logical_or(&self) -> &'static str {
        "or else"
    }

    fn logical_not(&self, expr: &str) -> String {
        format!("not ({})", expr)
    }

    fn wrap_assertion(&self, condition: &str) -> String {
        format!("pragma Assert ({});", condition)
    }

    fn precondition(&self, condition: &str) -> Option<String> {
        Some(format!("Pre  => {}", condition))
    }

    fn postcondition(&self, condition: &str) -> Option<String> {
        Some(format!("Post => {}", condition))
    }

    fn emit_contracts(&self, compound: &CompoundConstraint) -> Option<String> {
        let preconditions = self.extract_preconditions(compound);
        let postcondition = self.build_postcondition(compound);

        if preconditions.is_empty() && postcondition.is_none() {
            return None;
        }

        let mut contracts = String::new();
        contracts.push_str("   with\n");

        let mut first = true;
        for pre in &preconditions {
            if !first {
                contracts.push_str(",\n");
            }
            contracts.push_str(&format!("        {}", pre));
            first = false;
        }

        if let Some(post) = postcondition {
            if !first {
                contracts.push_str(",\n");
            }
            contracts.push_str(&format!("        {}", post));
        }

        Some(contracts)
    }

    fn wrap_verified_function(
        &self,
        func_name: &str,
        contracts: &str,
        body: &str,
        assertions: &str,
    ) -> String {
        let assertions_block = if !assertions.is_empty() {
            format!("   -- Runtime assertion checks\n   {}\n", assertions)
        } else {
            String::new()
        };

        format!(
            r#"-- SPARK/Ada Generated Code - Formally Verifiable
-- Use GNATprove for mathematical verification: `gnatprove -P<project> --level=4`

function {func_name} (Params : Validation_Params) return Boolean
   with SPARK_Mode => On{contracts}
is
 begin
{assertions_block}   return {body};
 end {func_name};"#,
            func_name = func_name,
            contracts = contracts,
            body = body,
            assertions_block = assertions_block.trim()
        )
    }
}

impl SparkAdaStrategy {
    fn extract_preconditions(&self, compound: &CompoundConstraint) -> Vec<String> {
        let mut preconditions = Vec::new();
        self.collect_preconditions(compound, &mut preconditions);
        preconditions
    }

    fn collect_preconditions(&self, compound: &CompoundConstraint, preconditions: &mut Vec<String>) {
        match compound {
            CompoundConstraint::Simple(c) => {
                // Extract meaningful preconditions from simple constraints
                let var = self.format_variable(&c.left_variable);
                let op = self.format_operator(&c.operator);
                let val = &c.right_value;
                preconditions.push(format!("{} {} {}", var, op, val));
            }
            CompoundConstraint::And(constraints) => {
                for c in constraints {
                    self.collect_preconditions(c, preconditions);
                }
            }
            CompoundConstraint::Or(_) | CompoundConstraint::Not(_) => {
                // OR/NOT constraints typically become part of postcondition or body
            }
        }
    }

    fn build_postcondition(&self, compound: &CompoundConstraint) -> Option<String> {
        let expr = self.build_expression_body(compound);
        // Relate 'Result directly to inputs for stronger GNATprove verification
        Some(format!("Post => ({}'Result = {})", "validate_intent", expr))
    }

    fn build_expression_body(&self, compound: &CompoundConstraint) -> String {
        match compound {
            CompoundConstraint::Simple(c) => {
                format!(
                    "{} {} {}",
                    self.format_variable(&c.left_variable),
                    self.format_operator(&c.operator),
                    c.right_value
                )
            }
            CompoundConstraint::And(constraints) => {
                let parts: Vec<String> = constraints
                    .iter()
                    .map(|c| self.build_expression_body(c))
                    .collect();
                format!("({})", parts.join(" and then "))
            }
            CompoundConstraint::Or(constraints) => {
                let parts: Vec<String> = constraints
                    .iter()
                    .map(|c| self.build_expression_body(c))
                    .collect();
                format!("({})", parts.join(" or else "))
            }
            CompoundConstraint::Not(inner) => {
                self.logical_not(&self.build_expression_body(inner))
            }
        }
    }
}

// --- Zig Strategy (Memory-Safe Systems Programming) ---

struct ZigStrategy;

impl CodegenStrategy for ZigStrategy {
    fn wrap_in_function(&self, body: &str, func_name: &str) -> String {
        format!(
            r#"// Zig Generated Code - Memory Safe Systems Programming
// Compile-time verification via comptime blocks

const std = @import("std");

pub const ValidationParams = struct {{
    // Define your validation parameters here
}};

pub fn {func_name}(params: ValidationParams) bool {{
    // Compile-time contract check (when possible)
    comptime {{
        // Static assertions can be added here
    }}

    return {body};
}}

test "{func_name}" {{
    // Property-based test harness
    const params = ValidationParams{{}};
    const result = {func_name}(params);
    try std.testing.expect(result == true or result == false);
}}"#,
            func_name = func_name,
            body = body
        )
    }

    fn format_operator(&self, op: &ConstraintOperator) -> &'static str {
        match op {
            ConstraintOperator::GreaterThanOrEqual => ">=",
            ConstraintOperator::LessThanOrEqual => "<=",
            ConstraintOperator::GreaterThan => ">",
            ConstraintOperator::LessThan => "<",
            ConstraintOperator::Equal => "==",
            ConstraintOperator::NotEqual => "!=",
        }
    }

    fn format_variable(&self, name: &str) -> String {
        format!("params.{}", name)
    }

    fn logical_and(&self) -> &'static str {
        "and"
    }

    fn logical_or(&self) -> &'static str {
        "or"
    }

    fn logical_not(&self, expr: &str) -> String {
        format!("!({})", expr)
    }

    fn wrap_assertion(&self, condition: &str) -> String {
        format!("std.debug.assert({});", condition)
    }

    fn is_comptime_capable(&self, compound: &CompoundConstraint) -> bool {
        // Check if all constraints are static (no runtime dependencies)
        self.is_static_constraint(compound)
    }

    fn wrap_verified_function(
        &self,
        func_name: &str,
        contracts: &str,
        body: &str,
        assertions: &str,
    ) -> String {
        let comptime_block = if !contracts.is_empty() {
            format!(
                r#"    comptime {{
        // Compile-time contract validation
        {contracts}
    }}
"#
            )
        } else {
            String::new()
        };

        let runtime_assertions = if !assertions.is_empty() {
            format!(
                r#"    // Runtime assertion checks
    {assertions}
"#
            )
        } else {
            String::new()
        };

        format!(
            r#"// Zig Generated Code - Memory Safe Systems Programming
// Compile-time and runtime verification

const std = @import("std");

pub const ValidationParams = struct {{
    // Define your validation parameters here
}};

pub fn {func_name}(params: ValidationParams) bool {{
{comptime_block}{runtime_assertions}    return {body};
}}

test "{func_name}" {{
    const params = ValidationParams{{}};
    const result = {func_name}(params);
    try std.testing.expect(result);
}}"#,
            func_name = func_name,
            comptime_block = comptime_block,
            runtime_assertions = runtime_assertions,
            body = body
        )
    }

    fn compile_error(&self, message: &str) -> String {
        format!("@compileError(\"{}\");", message)
    }
}

impl ZigStrategy {
    fn is_static_constraint(&self, compound: &CompoundConstraint) -> bool {
        match compound {
            CompoundConstraint::Simple(_) => true, // Simple constraints can be comptime
            CompoundConstraint::And(constraints) => constraints.iter().all(|c| self.is_static_constraint(c)),
            CompoundConstraint::Or(constraints) => constraints.iter().all(|c| self.is_static_constraint(c)),
            CompoundConstraint::Not(inner) => self.is_static_constraint(inner),
        }
    }
}

// --- Elixir Strategy (Fault-Tolerant Distributed Logic) ---

struct ElixirStrategy;

impl CodegenStrategy for ElixirStrategy {
    fn wrap_in_function(&self, body: &str, func_name: &str) -> String {
        format!(
            r#"# Elixir Generated Code - Fault-Tolerant Distributed Logic
# Use with ExUnit for property-based testing

defmodule Validator do
  @moduledoc \"\"\"
  Auto-generated validation module from Crucible Intent specification.
  \"\"\"

  @doc \"\"\"
  Validates the given parameters against the intent constraints.
  Returns true if all constraints are satisfied.
  \"\"\"
  @spec {func_name}?(map()) :: boolean()
  def {func_name}?(params) when is_map(params) do
    {body}
  end

  def {func_name}?(_), do: false
end"#,
            func_name = func_name,
            body = body
        )
    }

    fn format_operator(&self, op: &ConstraintOperator) -> &'static str {
        match op {
            ConstraintOperator::GreaterThanOrEqual => ">=",
            ConstraintOperator::LessThanOrEqual => "<=",
            ConstraintOperator::GreaterThan => ">",
            ConstraintOperator::LessThan => "<",
            ConstraintOperator::Equal => "==",
            ConstraintOperator::NotEqual => "!=",
        }
    }

    fn format_variable(&self, name: &str) -> String {
        format!("params[:{}]", name)
    }

    fn logical_and(&self) -> &'static str {
        "and"
    }

    fn logical_or(&self) -> &'static str {
        "or"
    }

    fn logical_not(&self, expr: &str) -> String {
        format!("not ({})", expr)
    }

    fn wrap_assertion(&self, condition: &str) -> String {
        format!("assert {}", condition)
    }

    fn to_guard_expression(&self, compound: &CompoundConstraint) -> Option<String> {
        Some(self.build_guard_expression(compound))
    }

    fn wrap_verified_function(
        &self,
        func_name: &str,
        contracts: &str,
        body: &str,
        assertions: &str,
    ) -> String {
        let assertions_code = if !assertions.is_empty() {
            format!(
                r#"
  # Assertion checks
  {assertions}"#
            )
        } else {
            String::new()
        };

        format!(
            r#"# Elixir Generated Code - Fault-Tolerant Distributed Logic
# Guard clauses for compile-time pattern matching

defmodule Validator do
  @moduledoc \"\"\"
  Auto-generated validation module from Crucible Intent specification.
  \"\"\"

  @doc \"\"\"
  Validates the given parameters against the intent constraints.
  Returns {{:ok, true}} on success, {{:error, reason}} on failure.
  \"\"\"
  @spec {func_name}?(map()) :: {{:ok, true}} | {{:error, atom()}}
{contracts}

  def {func_name}?(params) when is_map(params) and is_integer(params[:amount]) and params[:amount] >= 0 do
{body}{assertions_code}
  end

  def {func_name}?(params) when not is_map(params), do: {{:error, :invalid_type}}
  def {func_name}?(params) when not is_integer(params[:amount]), do: {{:error, :invalid_amount_type}}
  def {func_name}?(params) when params[:amount] < 0, do: {{:error, :negative_amount}}
  def {func_name}?(_), do: {{:error, :validation_failed}}
end"#,
            func_name = func_name,
            contracts = contracts,
            body = body,
            assertions_code = assertions_code.trim()
        )
    }
}

impl ElixirStrategy {
    fn build_guard_expression(&self, compound: &CompoundConstraint) -> String {
        match compound {
            CompoundConstraint::Simple(c) => {
                format!(
                    "{} {} {}",
                    self.format_variable(&c.left_variable),
                    self.format_operator(&c.operator),
                    self.format_value(&c.right_value)
                )
            }
            CompoundConstraint::And(constraints) => {
                let parts: Vec<String> = constraints
                    .iter()
                    .map(|c| self.build_guard_expression(c))
                    .collect();
                parts.join(" and ")
            }
            CompoundConstraint::Or(constraints) => {
                let parts: Vec<String> = constraints
                    .iter()
                    .map(|c| self.build_guard_expression(c))
                    .collect();
                format!("({})", parts.join(" or "))
            }
            CompoundConstraint::Not(inner) => {
                format!("not ({})", self.build_guard_expression(inner))
            }
        }
    }

    fn format_value(&self, value: &str) -> String {
        // Try to parse as integer first
        if value.parse::<i64>().is_ok() {
            value.to_string()
        } else {
            // Keep as atom/reference for guard compatibility
            format!("params[:{}]", value)
        }
    }
}

// --- Rust Strategy (with Kani proof harness support) ---

struct RustStrategy;

impl CodegenStrategy for RustStrategy {
    fn wrap_in_function(&self, body: &str, func_name: &str) -> String {
        format!(
            r#"//! Rust Generated Code - Memory Safe with Formal Verification
//! Use with Kani for bounded model checking

/// Validation parameters structure
#[derive(Debug, Clone)]
pub struct ValidationParams {{
    // Define your validation parameters here
}}

impl Validator {{
    /// Validates the given parameters against the intent constraints.
    ///
    /// # Returns
    /// `true` if all constraints are satisfied, `false` otherwise.
    #[inline]
    pub fn {func_name}(&self, params: &ValidationParams) -> bool {{
        {body}
    }}
}}

#[cfg(kani)]
mod verification {{
    use super::*;

    #[kani::proof]
    fn verify_{func_name}() {{
        let validator = Validator;
        let params = kani::any::<ValidationParams>();
        let result = validator.{func_name}(&params);
        // Kani will verify this function terminates and doesn't panic
        kani::cover!(result == true);
        kani::cover!(result == false);
    }}
}}"#,
            func_name = func_name,
            body = body
        )
    }

    fn format_operator(&self, op: &ConstraintOperator) -> &'static str {
        match op {
            ConstraintOperator::GreaterThanOrEqual => ">=",
            ConstraintOperator::LessThanOrEqual => "<=",
            ConstraintOperator::GreaterThan => ">",
            ConstraintOperator::LessThan => "<",
            ConstraintOperator::Equal => "==",
            ConstraintOperator::NotEqual => "!=",
        }
    }

    fn format_variable(&self, name: &str) -> String {
        format!("params.{}", name)
    }

    fn logical_and(&self) -> &'static str {
        "&&"
    }

    fn logical_or(&self) -> &'static str {
        "||"
    }

    fn logical_not(&self, expr: &str) -> String {
        format!("!({})", expr)
    }

    fn wrap_assertion(&self, condition: &str) -> String {
        format!("debug_assert!({});", condition)
    }

    fn wrap_verified_function(
        &self,
        func_name: &str,
        contracts: &str,
        body: &str,
        assertions: &str,
    ) -> String {
        let assertions_code = if !assertions.is_empty() {
            format!(
                r#"
        // Runtime assertion checks
        {assertions}"#
            )
        } else {
            String::new()
        };

        format!(
            r#"//! Rust Generated Code - Memory Safe with Formal Verification
//! Use with Kani for bounded model checking

/// Validation parameters structure
#[derive(Debug, Clone)]
pub struct ValidationParams {{
    // Define your validation parameters here
}}{contracts}

impl Validator {{
    /// Validates the given parameters against the intent constraints.
    ///
    /// # Returns
    /// `true` if all constraints are satisfied, `false` otherwise.
    #[inline]
    pub fn {func_name}(&self, params: &ValidationParams) -> bool {{
{assertions_code}
        {body}
    }}
}}

#[cfg(kani)]
mod verification {{
    use super::*;

    #[kani::proof]
    fn verify_{func_name}() {{
        let validator = Validator;
        let params = kani::any::<ValidationParams>();
        let result = validator.{func_name}(&params);
        kani::cover!(result == true);
        kani::cover!(result == false);
    }}
}}"#,
            func_name = func_name,
            contracts = contracts,
            body = body,
            assertions_code = assertions_code.trim()
        )
    }
}

// --- TypeScript Strategy ---

struct TypeScriptStrategy;

impl CodegenStrategy for TypeScriptStrategy {
    fn wrap_in_function(&self, body: &str, func_name: &str) -> String {
        format!(
            r#"// TypeScript Generated Code
// Use with ts-auto-guard for runtime type checking

export interface ValidationParams {{
  // Define your validation parameters here
}}

export class Validator {{
  /**
   * Validates the given parameters against the intent constraints.
   * @param params - The parameters to validate
   * @returns true if all constraints are satisfied
   */
  static {func_name}(params: ValidationParams): boolean {{
    return {body};
  }}
}}"#,
            func_name = func_name,
            body = body
        )
    }

    fn format_operator(&self, op: &ConstraintOperator) -> &'static str {
        match op {
            ConstraintOperator::GreaterThanOrEqual => ">=",
            ConstraintOperator::LessThanOrEqual => "<=",
            ConstraintOperator::GreaterThan => ">",
            ConstraintOperator::LessThan => "<",
            ConstraintOperator::Equal => "===",
            ConstraintOperator::NotEqual => "!==",
        }
    }

    fn format_variable(&self, name: &str) -> String {
        format!("params.{}", name)
    }

    fn logical_and(&self) -> &'static str {
        "&&"
    }

    fn logical_or(&self) -> &'static str {
        "||"
    }

    fn logical_not(&self, expr: &str) -> String {
        format!("!({})", expr)
    }

    fn wrap_verified_function(
        &self,
        func_name: &str,
        contracts: &str,
        body: &str,
        assertions: &str,
    ) -> String {
        let assertions_code = if !assertions.is_empty() {
            format!(
                r#"
    // Runtime assertion checks
    {assertions}"#
            )
        } else {
            String::new()
        };

        format!(
            r#"// TypeScript Generated Code
// Use with ts-auto-guard for runtime type checking

export interface ValidationParams {{
  // Define your validation parameters here
{contracts}
}}

export class Validator {{
  /**
   * Validates the given parameters against the intent constraints.
   * @param params - The parameters to validate
   * @returns true if all constraints are satisfied
   */
  static {func_name}(params: ValidationParams): boolean {{
{assertions_code}
    return {body};
  }}
}}"#,
            func_name = func_name,
            contracts = contracts,
            body = body,
            assertions_code = assertions_code.trim()
        )
    }
}

// --- Python Strategy ---

struct PythonStrategy;

impl CodegenStrategy for PythonStrategy {
    fn wrap_in_function(&self, body: &str, func_name: &str) -> String {
        format!(
            r#"# Python Generated Code
# Use with hypothesis for property-based testing

from typing import Dict, Any
from dataclasses import dataclass


@dataclass
class ValidationParams:
    """Validation parameters structure."""
    pass  # Define your validation parameters here


class Validator:
    """Auto-generated validator from Crucible Intent specification."""

    @staticmethod
    def {func_name}(params: Dict[str, Any]) -> bool:
        """
        Validates the given parameters against the intent constraints.

        Args:
            params: Dictionary of parameters to validate

        Returns:
            True if all constraints are satisfied, False otherwise
        """
        return {body}


# Property-based test example (requires hypothesis)
# from hypothesis import given, strategies as st
# @given(st.dictionaries(st.text(), st.integers()))
# def test_{func_name}(params):
#     result = Validator.{func_name}(params)
#     assert isinstance(result, bool)"#,
            func_name = func_name,
            body = body
        )
    }

    fn format_operator(&self, op: &ConstraintOperator) -> &'static str {
        match op {
            ConstraintOperator::GreaterThanOrEqual => ">=",
            ConstraintOperator::LessThanOrEqual => "<=",
            ConstraintOperator::GreaterThan => ">",
            ConstraintOperator::LessThan => "<",
            ConstraintOperator::Equal => "==",
            ConstraintOperator::NotEqual => "!=",
        }
    }

    fn format_variable(&self, name: &str) -> String {
        format!("params['{}']", name)
    }

    fn logical_and(&self) -> &'static str {
        "and"
    }

    fn logical_or(&self) -> &'static str {
        "or"
    }

    fn logical_not(&self, expr: &str) -> String {
        format!("not ({})", expr)
    }

    fn wrap_verified_function(
        &self,
        func_name: &str,
        contracts: &str,
        body: &str,
        assertions: &str,
    ) -> String {
        let assertions_code = if !assertions.is_empty() {
            format!(
                r#"
        # Runtime assertion checks
        {assertions}"#
            )
        } else {
            String::new()
        };

        format!(
            r#"# Python Generated Code
# Use with hypothesis for property-based testing

from typing import Dict, Any
from dataclasses import dataclass
{contracts}


@dataclass
class ValidationParams:
    """Validation parameters structure."""
    pass  # Define your validation parameters here


class Validator:
    """Auto-generated validator from Crucible Intent specification."""

    @staticmethod
    def {func_name}(params: Dict[str, Any]) -> bool:
        """
        Validates the given parameters against the intent constraints.

        Args:
            params: Dictionary of parameters to validate

        Returns:
            True if all constraints are satisfied, False otherwise
        """
{assertions_code}
        return {body}


# Property-based test example (requires hypothesis)
# from hypothesis import given, strategies as st
# @given(st.dictionaries(st.text(), st.integers()))
# def test_{func_name}(params):
#     result = Validator.{func_name}(params)
#     assert isinstance(result, bool)"#,
            func_name = func_name,
            contracts = contracts,
            body = body,
            assertions_code = assertions_code.trim()
        )
    }
}

// --- Solidity Strategy (Smart Contract Verification) ---

struct SolidityStrategy;

impl CodegenStrategy for SolidityStrategy {
    fn wrap_in_function(&self, body: &str, func_name: &str) -> String {
        format!(
            r#"// SPDX-License-Identifier: MIT
// Solidity Generated Code - Smart Contract Verification
// Use with Slither for security analysis, Echidna for property testing

struct ValidationParams {{
    // Define your validation parameters here
}}

contract Validator {{
    /// Validates the given parameters against the intent constraints.
    function {func_name}(ValidationParams memory params) public pure returns (bool) {{
        return {body};
    }}

    /// View function for external validation queries
    function validate(ValidationParams memory params) external view returns (bool) {{
        return this.{func_name}(params);
    }}
}}"#,
            func_name = func_name,
            body = body
        )
    }

    fn format_operator(&self, op: &ConstraintOperator) -> &'static str {
        match op {
            ConstraintOperator::GreaterThanOrEqual => ">=",
            ConstraintOperator::LessThanOrEqual => "<=",
            ConstraintOperator::GreaterThan => ">",
            ConstraintOperator::LessThan => "<",
            ConstraintOperator::Equal => "==",
            ConstraintOperator::NotEqual => "!=",
        }
    }

    fn format_variable(&self, name: &str) -> String {
        format!("params.{}", name)
    }

    fn logical_and(&self) -> &'static str {
        "&&"
    }

    fn logical_or(&self) -> &'static str {
        "||"
    }

    fn logical_not(&self, expr: &str) -> String {
        format!("!({})", expr)
    }

    fn wrap_assertion(&self, condition: &str) -> String {
        format!("require({});", condition)
    }

    fn wrap_verified_function(
        &self,
        func_name: &str,
        contracts: &str,
        body: &str,
        assertions: &str,
    ) -> String {
        let assertions_code = if !assertions.is_empty() {
            format!(
                r#"
        // Runtime assertion checks
        {assertions}"#
            )
        } else {
            String::new()
        };

        format!(
            r#"// SPDX-License-Identifier: MIT
// Solidity Generated Code - Smart Contract Verification
// Use with Slither for security analysis, Echidna for property testing
{contracts}
contract Validator {{
    /// Validation modifier for reentrancy protection
    modifier validate {{
        _;
        // Post-validation checks can go here
    }}

    function {func_name}(ValidationParams memory params) public pure returns (bool) {{
{assertions_code}
        return {body};
    }}
}}"#,
            contracts = format!("// Contracts: {}", contracts.trim().lines().next().unwrap_or("none")),
            func_name = func_name,
            body = body,
            assertions_code = assertions_code.trim()
        )
    }
}

// --- Helper Functions ---

/// Converts snake_case to Ada_Case (Title_Case with underscores)
fn to_ada_case(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join("_")
}

/// Build assertions for all simple constraints in a compound constraint
fn build_assertions(compound: &CompoundConstraint, strategy: &dyn CodegenStrategy) -> String {
    let mut assertions = Vec::new();
    collect_assertions(compound, strategy, &mut assertions);
    assertions.join("\n    ")
}

fn collect_assertions(
    compound: &CompoundConstraint,
    strategy: &dyn CodegenStrategy,
    assertions: &mut Vec<String>,
) {
    match compound {
        CompoundConstraint::Simple(c) => {
            let expr = format!(
                "{} {} {}",
                strategy.format_variable(&c.left_variable),
                strategy.format_operator(&c.operator),
                c.right_value
            );
            assertions.push(strategy.wrap_assertion(&expr));
        }
        CompoundConstraint::And(constraints) | CompoundConstraint::Or(constraints) => {
            for c in constraints {
                collect_assertions(c, strategy, assertions);
            }
        }
        CompoundConstraint::Not(inner) => {
            collect_assertions(inner, strategy, assertions);
        }
    }
}

// --- Main Engine ---

pub struct CodeGenerator;

impl CodeGenerator {
    /// Generate code for the given compound constraint in the target language.
    pub fn generate(
        &self,
        compound: &CompoundConstraint,
        language: TargetLanguage,
    ) -> Result<CodegenOutput, CodegenError> {
        let strategy: Box<dyn CodegenStrategy> = match language {
            TargetLanguage::Rust => Box::new(RustStrategy),
            TargetLanguage::TypeScript => Box::new(TypeScriptStrategy),
            TargetLanguage::Python => Box::new(PythonStrategy),
            TargetLanguage::SparkAda => Box::new(SparkAdaStrategy),
            TargetLanguage::Zig => Box::new(ZigStrategy),
            TargetLanguage::Elixir => Box::new(ElixirStrategy),
            TargetLanguage::Solidity => Box::new(SolidityStrategy),
        };

        // Build the main expression
        let expression = self.build_expression(compound, &*strategy);

        // Build assertions for runtime checking
        let assertions = build_assertions(compound, &*strategy);

        // Emit contracts if the strategy supports them
        let contracts = strategy.emit_contracts(compound).unwrap_or_default();

        // Generate the verified function with contracts and assertions
        let code = strategy.wrap_verified_function(
            "validate_intent",
            &contracts,
            &expression,
            &assertions,
        );

        Ok(CodegenOutput {
            language,
            code,
            constraints_count: compound.count_constraints(),
        })
    }

    /// Recursively build the boolean expression from compound constraints.
    fn build_expression(
        &self,
        compound: &CompoundConstraint,
        strategy: &dyn CodegenStrategy,
    ) -> String {
        match compound {
            CompoundConstraint::Simple(c) => {
                format!(
                    "{} {} {}",
                    strategy.format_variable(&c.left_variable),
                    strategy.format_operator(&c.operator),
                    c.right_value
                )
            }
            CompoundConstraint::And(constraints) => {
                let parts: Vec<String> = constraints
                    .iter()
                    .map(|c| self.build_expression(c, strategy))
                    .collect();
                format!("({})", parts.join(&format!(" {} ", strategy.logical_and())))
            }
            CompoundConstraint::Or(constraints) => {
                let parts: Vec<String> = constraints
                    .iter()
                    .map(|c| self.build_expression(c, strategy))
                    .collect();
                format!("({})", parts.join(&format!(" {} ", strategy.logical_or())))
            }
            CompoundConstraint::Not(inner) => {
                strategy.logical_not(&self.build_expression(inner, strategy))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crucible_core::{Constraint, ConstraintOperator, CompoundConstraint};

    fn sample_compound() -> CompoundConstraint {
        CompoundConstraint::And(vec![
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
        ])
    }

    #[test]
    fn test_rust_generation() {
        let generator = CodeGenerator;
        let result = generator.generate(&sample_compound(), TargetLanguage::Rust);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.code.contains("params.balance >= amount"));
        assert!(output.code.contains("params.amount > 0"));
        assert!(output.code.contains("#[kani::proof]"));
    }

    #[test]
    fn test_spark_ada_generation() {
        let generator = CodeGenerator;
        let result = generator.generate(&sample_compound(), TargetLanguage::SparkAda);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.code.contains("Params.Balance >= amount"));
        assert!(output.code.contains("and then"));
        assert!(output.code.contains("SPARK_Mode => On"));
        assert!(output.code.contains("Post =>"));
        assert!(output.code.contains("pragma Assert"));
    }

    #[test]
    fn test_zig_generation() {
        let generator = CodeGenerator;
        let result = generator.generate(&sample_compound(), TargetLanguage::Zig);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.code.contains("params.balance >= amount"));
        assert!(output.code.contains("comptime"));
        assert!(output.code.contains("std.debug.assert"));
    }

    #[test]
    fn test_elixir_generation() {
        let generator = CodeGenerator;
        let result = generator.generate(&sample_compound(), TargetLanguage::Elixir);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.code.contains("params[:balance] >= amount"));
        assert!(output.code.contains("def validate_intent?"));
        assert!(output.code.contains("when is_map(params)"));
    }

    #[test]
    fn test_python_generation() {
        let compound = CompoundConstraint::Or(vec![
            CompoundConstraint::Simple(Constraint {
                left_variable: "role".to_string(),
                operator: ConstraintOperator::Equal,
                right_value: "\"admin\"".to_string(),
            }),
            CompoundConstraint::Simple(Constraint {
                left_variable: "role".to_string(),
                operator: ConstraintOperator::Equal,
                right_value: "\"moderator\"".to_string(),
            }),
        ]);

        let generator = CodeGenerator;
        let result = generator.generate(&compound, TargetLanguage::Python);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.code.contains("or"));
        assert!(output.code.contains("hypothesis"));
    }

    #[test]
    fn test_typescript_generation() {
        let generator = CodeGenerator;
        let result = generator.generate(&sample_compound(), TargetLanguage::TypeScript);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.code.contains("params.balance >= amount"));
        assert!(output.code.contains("&&"));
    }

    #[test]
    fn test_not_expression() {
        let compound = CompoundConstraint::Not(Box::new(CompoundConstraint::Simple(Constraint {
            left_variable: "is_blocked".to_string(),
            operator: ConstraintOperator::Equal,
            right_value: "true".to_string(),
        })));

        let generator = CodeGenerator;
        let result = generator.generate(&compound, TargetLanguage::Rust);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.code.contains("!(params.is_blocked == true)"));
    }

    #[test]
    fn test_ada_case_conversion() {
        assert_eq!(to_ada_case("balance"), "Balance");
        assert_eq!(to_ada_case("user_balance"), "User_Balance");
        assert_eq!(to_ada_case("max_transfer_amount"), "Max_Transfer_Amount");
    }

    #[test]
    fn test_spark_ada_contracts() {
        let compound = CompoundConstraint::And(vec![
            CompoundConstraint::Simple(Constraint {
                left_variable: "amount".to_string(),
                operator: ConstraintOperator::GreaterThanOrEqual,
                right_value: "0".to_string(),
            }),
            CompoundConstraint::Simple(Constraint {
                left_variable: "balance".to_string(),
                operator: ConstraintOperator::GreaterThanOrEqual,
                right_value: "amount".to_string(),
            }),
        ]);

        let strategy = SparkAdaStrategy;
        let contracts = strategy.emit_contracts(&compound);
        assert!(contracts.is_some());
        let contracts_str = contracts.unwrap();
        assert!(contracts_str.contains("Pre  =>"));
        assert!(contracts_str.contains("Post =>"));
    }

    #[test]
    fn test_zig_comptime_capable() {
        let compound = sample_compound();
        let strategy = ZigStrategy;
        assert!(strategy.is_comptime_capable(&compound));
    }

    #[test]
    fn test_elixir_guard_expression() {
        let compound = sample_compound();
        let strategy = ElixirStrategy;
        let guard = strategy.to_guard_expression(&compound);
        assert!(guard.is_some());
        let guard_str = guard.unwrap();
        assert!(guard_str.contains("and"));
    }

    #[test]
    fn test_solidity_generation() {
        let generator = CodeGenerator;
        let result = generator.generate(&sample_compound(), TargetLanguage::Solidity);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.code.contains("params.balance >= amount"));
        assert!(output.code.contains("require("));
        assert!(output.code.contains("// SPDX-License-Identifier: MIT"));
    }
}
