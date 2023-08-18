; Forked from https://github.com/tree-sitter/tree-sitter-rust
; Copyright (c) 2017 Maxim Sokolov
; Licensed under the MIT license.

; Identifier conventions

[ "default" "enum" "impl" "let" "move" "pub" "struct" "trait" "type" "union" "unsafe" "where" ] @keyword
["(" ")" "[" "]" "{" "}"] @punctuation.bracket
(inner_attribute_item ["!" "#"] @punctuation.special)
(type_arguments  ["<" ">"] @punctuation.bracket)
(type_parameters ["<" ">"] @punctuation.bracket)
(bracketed_type ["<" ">"] @punctuation.bracket)
(for_lifetimes ["<" ">"] @punctuation.bracket)
(closure_parameters "|" @punctuation.bracket)
(attribute_item "#" @punctuation.special)
["," "." ":" "::" ";" "->" "=>"] @punctuation.delimiter
((identifier) @constant.builtin (#match? @constant.builtin "Some"))
((identifier) @constant.builtin (#match? @constant.builtin "None"))
((identifier) @constant.builtin (#match? @constant.builtin "Ok"))
((identifier) @constant.builtin (#match? @constant.builtin "Err"))
"$" @function.macro
[ "use" "mod" ] @include
[ "async" "await" ] @keyword.coroutine
[ "const" "static" "dyn" "extern" ] @storageclass
"fn" @keyword.function
[ "return" "yield" ] @keyword.return
[ "if" "else" "match" ] @conditional
[ "break" "continue" "in" "loop" "while" ] @repeat
[ "!" "!=" "%" "%=" "&" "&&" "&=" "*" "*=" "+" "+=" "-" "-=" ".." "..=" "/" "/=" "<" "<<" "<<=" "<=" "=" "==" ">" ">=" ">>" ">>=" "?" "@" "^" "^=" "|" "|=" "||" ] @operator
"for" @keyword
; -----------------------------------------------


((scoped_type_identifier path: (identifier) @type name: (type_identifier) @constant) (#match? @type "^[A-Z]") (#match? @constant "^[A-Z]"))
((scoped_identifier path: (identifier) @type name: (identifier) @constant) (#match? @type "^[A-Z]") (#match? @constant "^[A-Z]"))
((match_arm pattern: (match_pattern (scoped_identifier name: (identifier) @constant))) (#match? @constant "^[A-Z]"))
(call_expression function: (scoped_identifier "::" name: (identifier) @constant) (#match? @constant "^[A-Z]"))
((scoped_identifier name: (identifier) @constant) (#match? @constant "^[A-Z][A-Z0-9_]*$"))
((match_arm pattern: (match_pattern (identifier) @constant)) (#match? @constant "^[A-Z]"))
((identifier) @constant (#match? @constant "^[A-Z][A-Z0-9_]*$"))
((field_identifier) @constant (#match? @constant "^[A-Z]"))
((identifier) @type (#match? @type "^[A-Z]"))
(enum_variant name: (identifier) @constant)
(const_item name: (identifier) @constant)

(scoped_type_identifier path: (identifier) @type (#match? @type "^[A-Z]"))
((scoped_identifier path: (identifier) @type) (#match? @type "^[A-Z]"))
((scoped_identifier name: (identifier) @type) (#match? @type "^[A-Z]"))
(use_as_clause alias: (identifier) @type (#match? @type "^[A-Z]"))
(use_list (identifier) @type (#match? @type "^[A-Z]"))
[ "ref" (mutable_specifier) ] @type.qualifier
(primitive_type) @type.builtin
(type_identifier) @type


(self) @variable.builtin
(use_as_clause "as" @include)
(loop_label ["'" (identifier)] @label)

(generic_function function: (field_expression field: (field_identifier) @function.call))
(call_expression function: (field_expression field: (field_identifier) @function.call))
(generic_function function: (scoped_identifier name: (identifier) @function.call))
(call_expression function: (scoped_identifier (identifier) @function.call .))
(macro_invocation macro: (scoped_identifier (identifier) @function.macro .))
(attribute (scoped_identifier (identifier) @function.macro .))
(attribute_item (attribute (identifier) @function.macro))
(generic_function function: (identifier) @function.call)
(call_expression function: (identifier) @function.call)
(macro_invocation macro: (identifier) @function.macro)
(macro_definition "macro_rules!" @function.macro)
(function_signature_item (identifier) @function)
(function_item (identifier) @function)
(macro_invocation "!" @function.macro)
(metavariable) @function.macro

(parameter (identifier) @parameter)
(closure_parameters (_) @parameter)

(scoped_type_identifier (scoped_identifier name: (identifier) @namespace))
(scoped_identifier (scoped_identifier name: (identifier) @namespace))
(scoped_use_list path: (scoped_identifier (identifier) @namespace))
(use_list (scoped_identifier (identifier) @namespace . (_)))
(visibility_modifier [(crate) (super) (self)] @namespace)
(scoped_identifier [(crate) (super) (self)] @namespace)
(scoped_type_identifier path: (identifier) @namespace)
(scoped_identifier path: (identifier) @namespace)
(scoped_use_list path: (identifier) @namespace)
(mod_item name: (identifier) @namespace)
(scoped_use_list (self) @namespace)
[ (crate) (super) ] @namespace
(use_list (self) @namespace)

((block_comment) @comment.documentation (#match? @comment.documentation "^/[*][*][^*].*[*]/$"))
((block_comment) @comment.documentation (#match? @comment.documentation "^/[*][!]"))
((line_comment) @comment.documentation (#match? @comment.documentation "^///[^/]"))
((line_comment) @comment.documentation (#match? @comment.documentation "^///$"))
((line_comment) @comment.documentation (#match? @comment.documentation "^//!"))
[ (line_comment) (block_comment) ] @comment @spell

[ (raw_string_literal) (string_literal) ] @string
(escape_sequence) @string.escape
(boolean_literal) @boolean
(integer_literal) @number
(float_literal) @float
(char_literal) @character

(lifetime ["'" (identifier)] @storageclass.lifetime)

(type_cast_expression "as" @keyword.operator)
(qualified_type "as" @keyword.operator)

(for_expression "for" @repeat)


(empty_type "!" @type.builtin)

(macro_invocation macro: (identifier) @exception "!" @exception (#contains? @exception "assert"))
(macro_invocation macro: (identifier) @exception "!" @exception (#eq? @exception "panic"))
(macro_invocation macro: (identifier) @debug "!" @debug (#eq? @debug "dbg"))

(shorthand_field_initializer (identifier) @field)
(field_identifier) @field

(identifier) @variable
