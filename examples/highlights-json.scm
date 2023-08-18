[ (true) (false) ] @boolean

(null) @variable.builtin

(number) @number

; this is incorrectly tagged as @function on purpose to fit better with the colorscheme
(pair key: (string) @function)

(pair value: (string) @string)

(array (string) @string)

(ERROR) @error

["," ":"] @punctuation.delimiter

[ "[" "]" "{" "}" ] @punctuation.bracket

(escape_sequence) @string.escape
