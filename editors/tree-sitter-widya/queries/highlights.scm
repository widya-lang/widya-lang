; Tree-sitter Highlighting Queries for Widya-Lang
; Kompatibel dengan Neovim, Helix, Zed, Emacs, dan GitHub

; Keywords
[
  "fungsi" "fn" "func"
] @keyword.function

[
  "kembalikan" "return"
] @keyword.return

[
  "jika" "if"
  "selain_itu" "else"
  "kalau_tidak"
  "lainnya"
] @keyword.conditional

[
  "selama" "while"
  "untuk" "for"
  "dalam" "in"
] @keyword.repeat

[
  "henti" "berhenti" "break"
  "lanjut" "continue"
] @keyword.control

[
  "misal" "let" "biar"
  "tetap" "const"
  "struktur" "struct"
  "pilihan" "enum"
  "cocokkan" "match"
  "sifat" "trait"
  "terapkan" "impl"
] @keyword

[
  "impor" "import"
] @keyword.import

; Types & Builtins
(type_identifier) @type
(type) @type

; Function names
(function_declaration
  name: (identifier) @function)

; Calls
(call_expression
  function: (identifier) @function.call)
(call_expression
  function: (member_expression
    property: (identifier) @function.method))

; Parameters & Variables
(parameter
  name: (identifier) @variable.parameter)
(variable_declaration
  name: (identifier) @variable)
(for_statement
  variable: (identifier) @variable)

(identifier) @variable

; Literals
(number_literal) @number
(string_literal) @string
(boolean_literal) @boolean
(nil_literal) @constant.builtin

; Attributes
(attribute_item
  name: (identifier) @attribute)
"#" @attribute
"[" @attribute
"]" @attribute

; Operators & Punctuation
[
  "+" "-" "*" "/" "%"
  "==" "!=" "<" "<=" ">" ">="
  "&&" "||" "dan" "atau"
  "=" "+=" "-=" "*=" "/="
  "!" "bukan"
  "->" "=>" "::"
] @operator

[
  "{" "}"
  "[" "]"
  "(" ")"
] @punctuation.bracket

[
  "," ";" ":" "."
] @punctuation.delimiter

; Comments
(comment) @comment
