#!/usr/bin/env python3
"""
Z3 Constraint Generator for Crucible Engine
Phase 2: Parser & Basic Verification - v0.2.0-alpha

This module generates Z3 SMT constraints from parsed requirements.
"""

from z3 import *

def generate_constraints(requirements):
    """
    Generate Z3 constraints from a list of parsed requirements.
    
    Args:
        requirements: List of requirement dictionaries
        
    Returns:
        Tuple of (Solver, Dict of variables, constraints string)
    """
    solver = Solver()
    variables = {}
    constraints_log = []
    
    for req in requirements:
        # Extract constraint information
        subject = req.get('subject', 'x')
        action = req.get('action', {})
        condition = req.get('condition')
        constraint = req.get('constraint')
        
        # Create variables based on constraint expressions
        if condition:
            left_var = condition.get('left_variable', 'x')
            right_val = condition.get('right_value', '0')
            operator = condition.get('operator', '>=')
            
            # Ensure variable exists
            if left_var not in variables:
                variables[left_var] = Int(left_var)
            
            # Parse right value as integer
            try:
                right_int = int(right_val)
                right_expr = right_int
            except ValueError:
                if right_val not in variables:
                    variables[right_val] = Int(right_val)
                right_expr = variables[right_val]
            
            # Add constraint based on operator
            if operator == '>=':
                solver.add(variables[left_var] >= right_expr)
                constraints_log.append(f"({left_var} >= {right_val})")
            elif operator == '<=':
                solver.add(variables[left_var] <= right_expr)
                constraints_log.append(f"({left_var} <= {right_val})")
            elif operator == '>':
                solver.add(variables[left_var] > right_expr)
                constraints_log.append(f"({left_var} > {right_val})")
            elif operator == '<':
                solver.add(variables[left_var] < right_expr)
                constraints_log.append(f"({left_var} < {right_val})")
            elif operator == '==':
                solver.add(variables[left_var] == right_expr)
                constraints_log.append(f"({left_var} == {right_val})")
        
        if constraint:
            left_var = constraint.get('left_variable', 'y')
            right_val = constraint.get('right_value', '0')
            operator = constraint.get('operator', '>=')
            
            # Ensure variable exists
            if left_var not in variables:
                variables[left_var] = Int(left_var)
            
            # Parse right value
            try:
                right_int = int(right_val)
                right_expr = right_int
            except ValueError:
                if right_val not in variables:
                    variables[right_val] = Int(right_val)
                right_expr = variables[right_val]
            
            # Add constraint
            if operator == '>=':
                solver.add(variables[left_var] >= right_expr)
                constraints_log.append(f"({left_var} >= {right_val})")
            elif operator == '<=':
                solver.add(variables[left_var] <= right_expr)
                constraints_log.append(f"({left_var} <= {right_val})")
    
    # Format constraints for display
    constraints_str = '\n'.join([f"(assert {c})" for c in constraints_log])
    
    return solver, variables, constraints_str


def verify_constraints(constraints_output):
    """
    Verify constraints using Z3 solver.
    
    Args:
        constraints_output: Path to file containing constraints OR raw constraint string
        
    Returns:
        Dict with verification results
    """
    # Parse constraints from file or string
    if isinstance(constraints_output, str) and '\n' in constraints_output:
        constraints_str = constraints_output
    else:
        with open(constraints_output, 'r') as f:
            constraints_str = f.read()
    
    # Create a new solver
    solver = Solver()
    
    # Parse and add constraints
    for line in constraints_str.strip().split('\n'):
        line = line.strip()
        if line.startswith('(assert') and line.endswith(')'):
            constraint = line[7:-1]  # Remove "(assert " and ")"
            # Parse simple constraints like "(x >= 0)"
            import re
            match = re.match(r'\((\w+)\s*(>=|<=|==|>|<)\s*(\w+)\)', constraint)
            if match:
                left, op, right = match.groups()
                try:
                    right_val = int(right)
                    right_expr = right_val
                except ValueError:
                    right_expr = Int(right)
                
                if op == '>=':
                    solver.add(Int(left) >= right_expr)
                elif op == '<=':
                    solver.add(Int(left) <= right_expr)
                elif op == '>':
                    solver.add(Int(left) > right_expr)
                elif op == '<':
                    solver.add(Int(left) < right_expr)
                elif op == '==':
                    solver.add(Int(left) == right_expr)
    
    # Check satisfiability
    result = solver.check()
    
    if result == sat:
        model = solver.model()
        return {
            'status': 'SAT',
            'satisfiable': True,
            'model': {str(d): model[d] for d in model.decls() if d.name() != 'true' and d.name() != 'false'}
        }
    else:
        return {
            'status': 'UNSAT',
            'satisfiable': False,
            'message': 'Constraints are unsatisfiable - requirements are contradictory'
        }


if __name__ == '__main__':
    # Test with sample requirements
    sample_requirements = [
        {
            'subject': 'User',
            'action': {'verb': 'withdraw'},
            'condition': {
                'left_variable': 'balance',
                'operator': '>=',
                'right_value': 'amount'
            }
        },
        {
            'subject': 'User',
            'action': {'verb': 'withdraw'},
            'constraint': {
                'left_variable': 'amount',
                'operator': '>',
                'right_value': '0'
            }
        }
    ]
    
    print("=== Z3 Constraint Generator Test ===")
    print(f"Input requirements: {len(sample_requirements)}")
    
    solver, variables, constraints = generate_constraints(sample_requirements)
    
    print(f"\nGenerated variables: {list(variables.keys())}")
    print(f"\nConstraints:\n{constraints}")
    
    print(f"\nZ3 Solver check: {solver.check()}")
    if solver.check() == sat:
        print(f"Model: {solver.model()}")
