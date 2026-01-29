// Tree-Sitter Grammar for Natural Language Requirements
// Phase 2: Parser & Basic Verification - v0.2.0-alpha

module.exports = grammar({
  name: 'requirements',
  
  extras: $ => [
    $.whitespace,
    $.comment,
  ],
  
  word: $ => $.identifier,
  
  rules: {
    // Root: A requirements file contains one or more requirements
    source_file: $ => seq(
      repeat($.requirement),
      optional($.newline)
    ),
    
    // A single requirement line
    requirement: $ => seq(
      $.subject,
      $.modal_verb,
      $.action,
      optional($.condition),
      optional($.constraint),
      $.newline
    ),
    
    // Subject: Who or what the requirement applies to
    subject: $ => choice(
      'User',
      'System',
      'Admin',
      'Application',
      'Service',
      'API',
      $.identifier
    ),
    
    // Modal verb: Indicates obligation or capability
    modal_verb: $ => choice(
      'can',
      'must',
      'should',
      'shall',
      'will',
      'may'
    ),
    
    // Action: What the subject can/must do
    action: $ => seq(
      $.verb,
      $.object,
      optional($.preposition_phrase)
    ),
    
    // Verb: The action being performed
    verb: $ => choice(
      'create',
      'read',
      'update',
      'delete',
      'validate',
      'verify',
      'authenticate',
      'authorize',
      'encrypt',
      'decrypt',
      'send',
      'receive',
      'store',
      'retrieve',
      'process',
      'calculate',
      'generate',
      'export',
      'import',
      'withdraw',
      'deposit',
      'transfer',
      'login',
      'logout',
      'register',
      $.identifier
    ),
    
    // Object: What the action is performed on (using string literals to avoid conflicts)
    object: $ => choice(
      $.identifier,
      $.string,
      'data_record',
      'user_data',
      'transaction',
      'account_data',
      'password_data',
      'token_data',
      'certificate_data',
      'file_data',
      'document_data',
      'message_data'
    ),
    
    // Condition: Optional condition for the requirement
    condition: $ => seq(
      'if',
      $.constraint_expression
    ),
    
    // Constraint: Additional constraints on the requirement
    constraint: $ => seq(
      'where',
      $.constraint_expression
    ),
    
    // Constraint expression: Can be comparison or logical expression
    constraint_expression: $ => choice(
      $.comparison,
      $.logical_expression,
      $.arithmetic_expression
    ),
    
    // Comparison: Binary comparison operations
    comparison: $ => seq(
      $.left_expression,
      $.comparison_operator,
      $.right_expression
    ),
    
    // Logical expression: AND/OR combinations
    logical_expression: $ => choice(
      seq($.expression, 'and', $.expression),
      seq($.expression, 'or', $.expression),
      seq('not', $.expression)
    ),
    
    // Arithmetic expression: Basic math operations
    arithmetic_expression: $ => seq(
      $.left_expression,
      $.arithmetic_operator,
      $.right_expression
    ),
    
    // Left/right expressions for comparisons
    left_expression: $ => $.variable,
    right_expression: $ => choice($.variable, $.number, $.string),
    
    // Variable: A named variable in the constraint
    variable: $ => $.identifier,
    
    // Comparison operators
    comparison_operator: $ => choice(
      '==',
      '!=',
      '>',
      '<',
      '>=',
      '<=',
      'equals',
      'not_equals',
      'greater_than',
      'less_than',
      'at_least',
      'at_most',
      'is_set',
      'is_not_set',
      'contains',
      'does_not_contain'
    ),
    
    // Arithmetic operators
    arithmetic_operator: $ => choice('+', '-', '*', '/', '%'),
    
    // Preposition phrase: Additional context
    preposition_phrase: $ => seq(
      $.preposition,
      $.noun_phrase
    ),
    
    preposition: $ => choice(
      'from',
      'to',
      'in',
      'on',
      'at',
      'by',
      'with',
      'without',
      'for',
      'into',
      'onto',
      'through',
      'during',
      'before',
      'after'
    ),
    
    // Noun phrase: A group of words forming a noun
    noun_phrase: $ => repeat1(choice(
      $.identifier,
      $.string,
      $.number,
      'the'
    )),
    
    // Basic tokens
    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
    
    string: $ => seq(
      '"',
      repeat(choice(
        /[^"\\]+/,
        $.escape_sequence
      )),
      '"'
    ),
    
    escape_sequence: $ => seq(
      '\\',
      choice('n', 't', '"', '\\', '/')
    ),
    
    number: $ => /[0-9]+/,
    
    whitespace: $ => /\s+/,
    
    newline: $ => '\n',
    
    comment: $ => seq(
      '//',
      /.*/
    ),
    
    // Expression used in logical expressions
    expression: $ => choice(
      $.comparison,
      $.variable,
      $.number
    )
  }
});
