; Helix Tree-sitter Highlighting Queries for Widya-Lang

; Function and Return
[
  "fungsi" "fn" "func"
] @keyword.function

[
  "kembalikan" "return"
] @keyword.control.return

; Conditionals & Loops
[
  "jika" "if"
  "selain_itu" "else"
  "kalau_tidak"
  "lainnya"
] @keyword.control.conditional

[
  "selama" "while"
  "untuk" "for"
  "dalam" "in"
] @keyword.control.repeat

[
  "henti" "berhenti" "break"
  "lanjut" "continue"
] @keyword.control

; Declarations
[
  "misal" "let" "biar"
  "tetap" "const"
  "struktur" "struct"
  "pilihan" "enum"
  "cocokkan" "match"
  "sifat" "trait"
  "terapkan" "impl"
] @keyword.storage.type

[
  "impor" "import"
] @keyword.control.import

; Types
(type_identifier) @type
(type) @type

; Functions
(function_declaration
  name: (identifier) @function)

(call_expression
  function: (identifier) @function.call)
(call_expression
  function: (member_expression
    property: (identifier) @function.method))

; Parameters and Variables
(parameter
  name: (identifier) @variable.parameter)
(variable_declaration
  name: (identifier) @variable)
(for_statement
  variable: (identifier) @variable)

(identifier) @variable

; Literals
(number_literal) @constant.numeric
(string_literal) @string
(boolean_literal) @constant.builtin.boolean
(nil_literal) @constant.builtin

; Attributes
(attribute_item
  name: (identifier) @attribute)
"#" @attribute
"[" @attribute
"]" @attribute

; Operators
[
  "+" "-" "*" "/" "%"
  "==" "!=" "<" "<=" ">" ">="
  "&&" "||" "dan" "atau"
  "=" "+=" "-=" "*=" "/="
  "!" "bukan"
  "->" "=>" "::"
] @operator

; Punctuation
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
