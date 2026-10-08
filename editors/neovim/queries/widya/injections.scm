; Language Injections for Widya-Lang (e.g., embedded SQL, WGSL, or C code)

((call_expression
  function: (member_expression
    property: (identifier) @_method (#match? @_method "^(exec|query|execute)$"))
  arguments: (argument_list
    (string_literal) @injection.content))
 (#set! injection.language "sql"))

((call_expression
  function: (identifier) @_fn (#match? @_fn "^(wgsl|gpu_shader)$")
  arguments: (argument_list
    (string_literal) @injection.content))
 (#set! injection.language "wgsl"))
