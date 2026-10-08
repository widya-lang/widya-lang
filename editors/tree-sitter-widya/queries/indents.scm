; Tree-sitter Indentation Queries for Widya-Lang
[
  (block)
  (parameter_list)
  (argument_list)
  (array_literal)
  (map_literal)
] @indent.begin

[
  "}"
  "]"
  ")"
] @indent.end

[
  (comment)
] @indent.ignore
