; Tree-sitter Scoping & Locals Queries for Widya-Lang
[
  (source_file)
  (block)
  (function_declaration)
] @local.scope

(function_declaration
  name: (identifier) @local.definition)

(parameter
  name: (identifier) @local.definition)

(variable_declaration
  name: (identifier) @local.definition)

(for_statement
  variable: (identifier) @local.definition)

(identifier) @local.reference
