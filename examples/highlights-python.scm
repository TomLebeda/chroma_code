;; From tree-sitter-python licensed under MIT License
; Copyright (c) 2016 Max Brunsfeld

[ "-" "-=" ":=" "!=" "*" "**" "**=" "*=" "/" "//" "//=" "/=" "&" "&=" "%" "%=" "^" "^=" "+" "+=" "<" "<<" "<<=" "<=" "<>" "=" "==" ">" ">=" ">>" ">>=" "@" "@=" "|" "|=" "~" "->" ] @operator


; Keywords
[ "and" "in" "is" "not" "or" "is not" "not in" "del" ] @keyword.operator

[ "def" "lambda" ] @keyword.function

[ "assert" "class" "exec" "global" "nonlocal" "pass" "print" "with" "as" ] @keyword

[ "async" "await" ] @keyword.coroutine

[ "return" "yield" ] @keyword.return

(yield "from" @keyword.return)

((attribute attribute: (identifier) @field) (#match? @field "^[a-z_].*$"))
((class_definition body: (block (expression_statement (assignment left: (identifier) @field)))) (#match? @field "^[a-z].*$"))
((class_definition body: (block (expression_statement (assignment left: (_ (identifier) @field))))) (#match? @field "^[a-z].*$"))

((identifier) @constant.builtin (#eq? @constant.builtin "NotImplemented"))
((identifier) @constant.builtin (#eq? @constant.builtin "Ellipsis"))
((identifier) @constant.builtin (#eq? @constant.builtin "quit"))
((identifier) @constant.builtin (#eq? @constant.builtin "exit"))
((identifier) @constant.builtin (#eq? @constant.builtin "credits"))
((identifier) @constant.builtin (#eq? @constant.builtin "license"))

;; Builtin functions

((call function: (identifier) @function.builtin) (#eq? @function.builtin "abs"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "all"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "any"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "ascii"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "bin"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "bool"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "breakpoint"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "bytearray"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "bytes"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "callable"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "chr"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "classmethod"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "compile"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "complex"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "delattr"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "dict"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "dir"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "divmod"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "enumerate"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "eval"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "exec"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "filter"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "float"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "format"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "frozenset"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "getattr"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "globals"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "hasattr"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "hash"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "help"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "hex"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "id"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "input"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "int"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "isinstance"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "issubclass"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "iter"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "len"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "list"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "locals"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "map"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "max"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "memoryview"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "min"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "next"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "object"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "oct"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "open"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "ord"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "pow"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "print"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "propery"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "range"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "repr"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "reversed"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "set"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "setattr"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "slice"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "sorted"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "staticmethod"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "str"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "sum"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "super"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "tuple"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "type"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "vars"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "zip"))
((call function: (identifier) @function.builtin) (#eq? @function.builtin "__import__"))

((class_definition (block (function_definition name: (identifier) @constructor))) (#eq? @constructor "__new__"))
((class_definition (block (function_definition name: (identifier) @constructor))) (#eq? @constructor "__init__"))

((call function: (identifier) @constructor) (#match? @constructor "^[A-Z]"))

((call function: (attribute attribute: (identifier) @constructor)) (#match? @constructor "^[A-Z]"))


; Variables

((identifier) @variable.builtin (#eq? @variable.builtin "self"))

((identifier) @variable.builtin (#eq? @variable.builtin "cls"))


;; Normal parameters
(parameters (identifier) @parameter)

;; Lambda parameters
(lambda_parameters (identifier) @parameter)

(lambda_parameters (tuple_pattern (identifier) @parameter))

; Default parameters
(keyword_argument name: (identifier) @parameter)

; Naming parameters on call-site
(default_parameter name: (identifier) @parameter)

(typed_parameter (identifier) @parameter)

(typed_default_parameter (identifier) @parameter)

; Variadic parameters *args, **kwargs
(parameters (list_splat_pattern (identifier) @parameter))

(parameters (dictionary_splat_pattern (identifier) @parameter))

; Reset highlighting in f-string interpolations
(interpolation) @none


;; Identifier naming conventions
((identifier) @type (#match? @type "^[A-Z].*[a-z]"))

((identifier) @constant (#match? @constant "^[A-Z][A-Z_0-9]*$"))

((identifier) @constant.builtin (#match? @constant.builtin "^__[a-zA-Z0-9_]*__$"))



((assignment left: (identifier) @type.definition (type (identifier) @_annotation)) (#eq? @_annotation "TypeAlias"))

((assignment left: (identifier) @type.definition right: (call function: (identifier) @_func)) (#eq? @_func "TypeVar"))
((assignment left: (identifier) @type.definition right: (call function: (identifier) @_func)) (#eq? @_func "NewType"))

; Function calls

(call function: (identifier) @function.call)

(call function: (attribute attribute: (identifier) @method.call))


;; Decorators

((decorator "@" @attribute) (#set! "priority" 101))

(decorator (identifier) @attribute)

(decorator (attribute attribute: (identifier) @attribute))

(decorator (call (identifier) @attribute))

(decorator (call (attribute attribute: (identifier) @attribute)))

((decorator (identifier) @attribute.builtin) (#match? @attribute.builtin "classmethod"))
((decorator (identifier) @attribute.builtin) (#match? @attribute.builtin "property"))


;; Function definitions

(function_definition name: (identifier) @function)

(type (identifier) @type)

(type (subscript (identifier) @type)) ; type subscript: Tuple[int]

((call function: (identifier) @_isinstance arguments: (argument_list (_) (identifier) @type)) (#eq? @_isinstance "isinstance"))


;; Literals

(none) @constant.builtin

[(true) (false)] @boolean

(integer) @number
(float) @float

(comment) @comment @spell

((module . (comment) @preproc) (#match? @preproc "^#!/"))

(module . (expression_statement (string) @string.documentation @spell))

(class_definition body: (block . (expression_statement (string) @string.documentation @spell)))

(function_definition body: (block . (expression_statement (string) @string.documentation @spell)))

(string) @string

(escape_sequence) @string.escape

; doc-strings

; Tokens


(future_import_statement "from" @include "__future__" @constant.builtin)

(import_from_statement "from" @include)

"import" @include

(aliased_import "as" @include)

["if" "elif" "else" "match" "case"] @conditional

["for" "while" "break" "continue"] @repeat

[ "try" "except" "except*" "raise" "finally" ] @exception

(raise_statement "from" @exception)

(try_statement (else_clause "else" @exception))

["(" ")" "[" "]" "{" "}"] @punctuation.bracket

(interpolation "{" @punctuation.special "}" @punctuation.special)

(type_conversion) @function.macro

["," "." ":" ";" (ellipsis)] @punctuation.delimiter

;; Class definitions

(class_definition name: (identifier) @type)

(class_definition body: (block (function_definition name: (identifier) @method)))

(class_definition superclasses: (argument_list (identifier) @type))



((identifier) @type.builtin (#match? @type.builtin "BaseException"))
((identifier) @type.builtin (#match? @type.builtin "Exception"))
((identifier) @type.builtin (#match? @type.builtin "ArithmeticError"))
((identifier) @type.builtin (#match? @type.builtin "BufferError"))
((identifier) @type.builtin (#match? @type.builtin "LookupError"))
((identifier) @type.builtin (#match? @type.builtin "AssertionError"))
((identifier) @type.builtin (#match? @type.builtin "AttributeError"))
((identifier) @type.builtin (#match? @type.builtin "EOFError"))
((identifier) @type.builtin (#match? @type.builtin "FloatingPointError"))
((identifier) @type.builtin (#match? @type.builtin "GeneratorExit"))
((identifier) @type.builtin (#match? @type.builtin "ImportError"))
((identifier) @type.builtin (#match? @type.builtin "ModuleNotFoundError"))
((identifier) @type.builtin (#match? @type.builtin "IndexError"))
((identifier) @type.builtin (#match? @type.builtin "KeyError"))
((identifier) @type.builtin (#match? @type.builtin "KeyboardInterrupt"))
((identifier) @type.builtin (#match? @type.builtin "MemoryError"))
((identifier) @type.builtin (#match? @type.builtin "NameError"))
((identifier) @type.builtin (#match? @type.builtin "NotImplementedError"))
((identifier) @type.builtin (#match? @type.builtin "OSError"))
((identifier) @type.builtin (#match? @type.builtin "OverflowError"))
((identifier) @type.builtin (#match? @type.builtin "RecursionError"))
((identifier) @type.builtin (#match? @type.builtin "ReferenceError"))
((identifier) @type.builtin (#match? @type.builtin "RuntimeError"))
((identifier) @type.builtin (#match? @type.builtin "StopIteration"))
((identifier) @type.builtin (#match? @type.builtin "StopAsyncIteration"))
((identifier) @type.builtin (#match? @type.builtin "SyntaxError"))
((identifier) @type.builtin (#match? @type.builtin "IndentationError"))
((identifier) @type.builtin (#match? @type.builtin "TabError"))
((identifier) @type.builtin (#match? @type.builtin "SystemError"))
((identifier) @type.builtin (#match? @type.builtin "SystemExit"))
((identifier) @type.builtin (#match? @type.builtin "TypeError"))
((identifier) @type.builtin (#match? @type.builtin "UnboundLocalError"))
((identifier) @type.builtin (#match? @type.builtin "UnicodeError"))
((identifier) @type.builtin (#match? @type.builtin "UnicodeEncodeError"))
((identifier) @type.builtin (#match? @type.builtin "UnicodeDecodeError"))
((identifier) @type.builtin (#match? @type.builtin "UnicodeTranslateError"))
((identifier) @type.builtin (#match? @type.builtin "ValueError"))
((identifier) @type.builtin (#match? @type.builtin "ZeroDivisionError"))
((identifier) @type.builtin (#match? @type.builtin "EnvironmentError"))
((identifier) @type.builtin (#match? @type.builtin "IOError"))
((identifier) @type.builtin (#match? @type.builtin "WindowsError"))
((identifier) @type.builtin (#match? @type.builtin "BlockingIOError"))
((identifier) @type.builtin (#match? @type.builtin "ChildProcessError"))
((identifier) @type.builtin (#match? @type.builtin "ConnectionError"))
((identifier) @type.builtin (#match? @type.builtin "BrokenPipeError"))
((identifier) @type.builtin (#match? @type.builtin "ConnectionAbortedError"))
((identifier) @type.builtin (#match? @type.builtin "ConnectionRefusedError"))
((identifier) @type.builtin (#match? @type.builtin "ConnectionResetError"))
((identifier) @type.builtin (#match? @type.builtin "FileExistsError"))
((identifier) @type.builtin (#match? @type.builtin "FileNotFoundError"))
((identifier) @type.builtin (#match? @type.builtin "InterruptedError"))
((identifier) @type.builtin (#match? @type.builtin "IsADirectoryError"))
((identifier) @type.builtin (#match? @type.builtin "NotADirectoryError"))
((identifier) @type.builtin (#match? @type.builtin "PermissionError"))
((identifier) @type.builtin (#match? @type.builtin "ProcessLookupError"))
((identifier) @type.builtin (#match? @type.builtin "TimeoutError"))
((identifier) @type.builtin (#match? @type.builtin "Warning"))
((identifier) @type.builtin (#match? @type.builtin "UserWarning"))
((identifier) @type.builtin (#match? @type.builtin "DeprecationWarning"))
((identifier) @type.builtin (#match? @type.builtin "PendingDeprecationWarning"))
((identifier) @type.builtin (#match? @type.builtin "SyntaxWarning"))
((identifier) @type.builtin (#match? @type.builtin "RuntimeWarning"))
((identifier) @type.builtin (#match? @type.builtin "FutureWarning"))
((identifier) @type.builtin (#match? @type.builtin "ImportWarning"))
((identifier) @type.builtin (#match? @type.builtin "UnicodeWarning"))
((identifier) @type.builtin (#match? @type.builtin "BytesWarning"))
((identifier) @type.builtin (#match? @type.builtin "ResourceWarning"))
((identifier) @type.builtin (#match? @type.builtin "bool"))
((identifier) @type.builtin (#match? @type.builtin "int"))
((identifier) @type.builtin (#match? @type.builtin "float"))
((identifier) @type.builtin (#match? @type.builtin "complex"))
((identifier) @type.builtin (#match? @type.builtin "list"))
((identifier) @type.builtin (#match? @type.builtin "tuple"))
((identifier) @type.builtin (#match? @type.builtin "range"))
((identifier) @type.builtin (#match? @type.builtin "str"))
((identifier) @type.builtin (#match? @type.builtin "bytes"))
((identifier) @type.builtin (#match? @type.builtin "bytearray"))
((identifier) @type.builtin (#match? @type.builtin "memoryview"))
((identifier) @type.builtin (#match? @type.builtin "set"))
((identifier) @type.builtin (#match? @type.builtin "frozenset"))
((identifier) @type.builtin (#match? @type.builtin "dict"))
((identifier) @type.builtin (#match? @type.builtin "type"))
((identifier) @type.builtin (#match? @type.builtin "object"))

;; Error
(ERROR) @error

(identifier) @variable
