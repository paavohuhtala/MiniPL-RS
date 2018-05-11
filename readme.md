# MiniPL-RS

[![Build Status](https://travis-ci.org/paavohuhtala/MiniPL-RS.svg?branch=master)](https://travis-ci.org/paavohuhtala/MiniPL-RS)

An interpreter for a small strongly typed scripting language. Written in Rust for the Compilers course at University of Helsinki during spring 2018.

The original specifications are available in the docs folder.

Consists of the following components:
* A lexer, backed by a char slice.
* A recursive descent parser which _should_ parse everything without backtracking.
  * Utilises a modified [shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm) for expression parsing.
* A type checker. Since the language doesn't support functions or structs and has only 3 built-in types it's rather simple.
* An AST interpreter.

# License

MIT, but don't copy this if you're on the same course. ;)
