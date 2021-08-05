# nom-parser-lexer-example
Using nom to lex and parse a calculator.
There are two types of basic terms:
- Natural numbers (including 0). E.g. `1`, `10`, etc.
- Identifiers. They can start with any letter or an underscore (`_`), and then any sequence of letters, numbers nad underscores. E.g. `MyInt`, `My_int_2`, etc.
Though there is no way to define them, they are part of the grammar to exemplify how to work with tokens that need to be "captured", and how to manage the lifetimes.

The project structure is straightforward.
The tokens are defined in `token.rs`, which are used in `lexer.rs`, the implementation of the lexer.
The AST structure is in `ast.rs` and the parser is implemented in `parser.rs`.
