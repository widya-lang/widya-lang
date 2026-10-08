/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'widya',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  word: $ => $.identifier,

  conflicts: $ => [
    [$.type, $.identifier],
    [$.primary_expression, $.type_identifier],
    [$.pattern, $.expression],
  ],

  rules: {
    source_file: $ => repeat($._statement),

    _statement: $ => choice(
      $.attribute_item,
      $.function_declaration,
      $.variable_declaration,
      $.struct_declaration,
      $.enum_declaration,
      $.trait_declaration,
      $.impl_block,
      $.import_statement,
      $.if_statement,
      $.while_statement,
      $.for_statement,
      $.match_statement,
      $.return_statement,
      $.break_statement,
      $.continue_statement,
      $.expression_statement,
      $.block
    ),

    // Attributes #[tag("...")]
    attribute_item: $ => seq(
      '#',
      '[',
      field('name', $.identifier),
      optional(seq('(', commaSeparated($._literal), ')')),
      ']'
    ),

    // Comments
    comment: $ => token(choice(
      seq('//', /.*/),
      seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')
    )),

    // Function Declaration
    function_declaration: $ => seq(
      optional($.attribute_item),
      choice('fungsi', 'fn', 'func'),
      field('name', $.identifier),
      optional($.type_parameters),
      field('parameters', $.parameter_list),
      optional(seq('->', field('return_type', $.type))),
      field('body', $.block)
    ),

    type_parameters: $ => seq(
      '<',
      commaSeparated1(field('param', $.type_identifier)),
      '>'
    ),

    parameter_list: $ => seq(
      '(',
      commaSeparated($.parameter),
      ')'
    ),

    parameter: $ => seq(
      field('name', $.identifier),
      optional(seq(':', field('type', $.type)))
    ),

    // Variable Declaration
    variable_declaration: $ => seq(
      choice('misal', 'let', 'biar', 'tetap', 'const'),
      field('name', $.identifier),
      optional(seq(':', field('type', $.type))),
      optional(seq('=', field('value', $.expression))),
      optional(';')
    ),

    // Struct Declaration
    struct_declaration: $ => seq(
      optional($.attribute_item),
      choice('struktur', 'struct'),
      field('name', $.type_identifier),
      optional($.type_parameters),
      '{',
      commaSeparated($.struct_field),
      '}'
    ),

    struct_field: $ => seq(
      field('name', $.identifier),
      ':',
      field('type', $.type)
    ),

    // Enum Declaration
    enum_declaration: $ => seq(
      optional($.attribute_item),
      choice('pilihan', 'enum'),
      field('name', $.type_identifier),
      optional($.type_parameters),
      '{',
      commaSeparated($.enum_variant),
      '}'
    ),

    enum_variant: $ => seq(
      field('name', $.type_identifier),
      optional(seq('(', commaSeparated($.type), ')'))
    ),

    // Trait / Sifat Declaration
    trait_declaration: $ => seq(
      choice('sifat', 'trait'),
      field('name', $.type_identifier),
      '{',
      repeat($._statement),
      '}'
    ),

    // Impl / Terapkan Block
    impl_block: $ => seq(
      choice('terapkan', 'impl'),
      field('trait', optional(seq($.type_identifier, choice('untuk', 'for')))),
      field('target', $.type_identifier),
      '{',
      repeat($._statement),
      '}'
    ),

    // Import Statement
    import_statement: $ => seq(
      choice('impor', 'import'),
      choice($.string_literal, $.identifier),
      optional(';')
    ),

    // Control Flow
    if_statement: $ => seq(
      choice('jika', 'if'),
      field('condition', $.expression),
      field('consequence', $.block),
      optional(seq(
        choice('selain_itu', 'else', 'kalau_tidak', 'lainnya'),
        field('alternative', choice($.if_statement, $.block))
      ))
    ),

    while_statement: $ => seq(
      choice('selama', 'while'),
      field('condition', $.expression),
      field('body', $.block)
    ),

    for_statement: $ => seq(
      choice('untuk', 'for'),
      field('variable', $.identifier),
      choice('dalam', 'in'),
      field('iterable', $.expression),
      field('body', $.block)
    ),

    match_statement: $ => seq(
      choice('cocokkan', 'match'),
      field('value', $.expression),
      '{',
      repeat($.match_arm),
      '}'
    ),

    match_arm: $ => seq(
      field('pattern', $.pattern),
      optional(seq(choice('jika', 'if'), field('guard', $.expression))),
      '=>',
      choice($.block, seq($.expression, optional(','))),
      optional(',')
    ),

    pattern: $ => choice(
      $._literal,
      $.identifier,
      seq($.type_identifier, optional(seq('::', $.type_identifier)), optional(seq('(', commaSeparated($.pattern), ')'))),
      '_'
    ),

    return_statement: $ => seq(
      choice('kembalikan', 'return'),
      optional($.expression),
      optional(';')
    ),

    break_statement: $ => seq(
      choice('henti', 'berhenti', 'break'),
      optional(';')
    ),

    continue_statement: $ => seq(
      choice('lanjut', 'continue'),
      optional(';')
    ),

    expression_statement: $ => seq(
      $.expression,
      optional(';')
    ),

    block: $ => seq(
      '{',
      repeat($._statement),
      '}'
    ),

    // Expressions
    expression: $ => choice(
      $.binary_expression,
      $.unary_expression,
      $.primary_expression
    ),

    binary_expression: $ => prec.left(1, seq(
      field('left', $.expression),
      field('operator', choice(
        '+', '-', '*', '/', '%',
        '==', '!=', '<', '<=', '>', '>=',
        '&&', '||', 'dan', 'atau',
        '=', '+=', '-=', '*=', '/='
      )),
      field('right', $.expression)
    )),

    unary_expression: $ => prec(2, seq(
      field('operator', choice('-', '!', 'bukan')),
      field('argument', $.expression)
    )),

    primary_expression: $ => choice(
      $._literal,
      $.identifier,
      $.type_identifier,
      $.call_expression,
      $.member_expression,
      $.index_expression,
      $.array_literal,
      $.map_literal,
      $.parenthesized_expression
    ),

    call_expression: $ => prec(3, seq(
      field('function', choice($.identifier, $.member_expression)),
      field('arguments', $.argument_list)
    )),

    argument_list: $ => seq(
      '(',
      commaSeparated($.expression),
      ')'
    ),

    member_expression: $ => prec(4, seq(
      field('object', $.primary_expression),
      choice('.', '::'),
      field('property', choice($.identifier, $.type_identifier))
    )),

    index_expression: $ => prec(4, seq(
      field('array', $.primary_expression),
      '[',
      field('index', $.expression),
      ']'
    )),

    parenthesized_expression: $ => seq(
      '(',
      $.expression,
      ')'
    ),

    array_literal: $ => seq(
      '[',
      commaSeparated($.expression),
      ']'
    ),

    map_literal: $ => seq(
      '{',
      commaSeparated($.map_entry),
      '}'
    ),

    map_entry: $ => seq(
      field('key', choice($.string_literal, $.identifier)),
      ':',
      field('value', $.expression)
    ),

    // Literals
    _literal: $ => choice(
      $.number_literal,
      $.string_literal,
      $.boolean_literal,
      $.nil_literal
    ),

    number_literal: $ => token(choice(
      /\d+(\.\d+)?([eE][+-]?\d+)?/,
      /0x[0-9a-fA-F]+/,
      /0b[01]+/
    )),

    string_literal: $ => token(choice(
      seq('"', repeat(choice(/[^"\\]/, seq('\\', /./))), '"'),
      seq("'", repeat(choice(/[^'\\]/, seq('\\', /./))), "'")
    )),

    boolean_literal: $ => choice('benar', 'salah', 'true', 'false'),

    nil_literal: $ => choice('kosong', 'null', 'nil'),

    // Types
    type: $ => choice(
      $.type_identifier,
      seq($.type_identifier, '<', commaSeparated1($.type), '>'),
      seq('[', $.type, ']'),
      seq('{', $.type, ':', $.type, '}')
    ),

    // Identifiers
    identifier: $ => /[a-z_][a-zA-Z0-9_]*/,
    type_identifier: $ => /[A-Z][a-zA-Z0-9_]*/,
  }
});

function commaSeparated(rule) {
  return optional(commaSeparated1(rule));
}

function commaSeparated1(rule) {
  return seq(rule, repeat(seq(',', rule)), optional(','));
}
