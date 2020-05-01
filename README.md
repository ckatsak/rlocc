# rlocc


Just another blazing fast LOC counter.

Largely works, but it is not finished yet.
Check the [known issues below](#known-issues) if you want to be sure whether you wish to use it.

![Build Status (master)](https://img.shields.io/travis/com/ckatsak/rlocc/master?label=master&style=for-the-badge)
![Build Status (develop)](https://img.shields.io/travis/com/ckatsak/rlocc/develop?label=develop&style=for-the-badge)
![GitHub](https://img.shields.io/github/license/ckatsak/rlocc?style=for-the-badge)


### Disclaimer

This is my first ever project in Rust; developed while reading the Rust Book!
My sincere apologies to the Rust community for possibly abusing the language; at the time `rlocc` is written I've been still fighting the borrow checker.



## Contents

- [Installation](#installation)
- [Usage](#usage)
- [Platforms](#platforms)
- [Supported File Types](#supported-file-types)
- [Known issues & TODOs](#known-issues)



## Installation <a name="installation"></a>

Assuming Rust is installed, it can be installed using the Makefile:

```text
$ make
```

which simply uses cargo as usual:

```text
$ cargo build --release
```



## Usage <a name="usage"></a>

Although `rlocc` has been developed as a library, it was mostly meant to be run through its accompanying binary.
In other words, its API is not really well thought for use outside the provided binary.

As a command line tool, rlocc is very simple to use: it receives any number of file or directory names as a command line input, and walks through them counting them.

For example, to count files `file1`, `../file3` and all files under `~/dir2`, one can issue:

```text
$ rlocc file1 ~/dir2 ../file3
```

No command line flags are supported at this time.



## Platforms <a name="platforms"></a>

So far `rlocc` has only been tested on `linux/amd64` with Rust `1.42.0` or later.



## Supported File Types <a name="supported-file-types"></a>

Currently `rlocc` supports 75 types of files.
It guesses the file type mostly via file name extensions, with very few exceptions (for Makefile, Dockerfile, etc).

The exhaustive list of all supported file types:

- Ada
- Assembly
- Autoconf
- AWK
- Batch
- C
- C++
- C/C++ Header
- C#
- Clojure
- CMake
- COBOL
- CSV
- CSS
- D
- Dart
- Delphi
- Dockerfile
- Eiffel
- Elm
- Elixir
- Erlang
- F#
- .gitignore
- Go
- Haskell
- HTML
- Java
- Javascript
- JSON
- Julia
- Jupyter
- Kotlin
- License files
- Lisp
- Lua
- Makefile
- MAL (MonetDB)
- Markdown
- Matlab
- Nim
- Nix
- OCaml
- OpenCL
- Pascal
- Perl
- PHP
- Plain Text
- Pony
- PowerShell
- Protocol Buffers
- Python
- R
- ReStructuredText
- Ruby
- Rust
- Scala
- Scheme
- Sed
- Shell
- SML
- Solidity
- SQL
- Swift
- TeX
- Tcl
- TOML
- TypeScript
- V
- Vala
- VimL
- WebAssembly (text format)
- YAML
- XML
- Zig



## Known issues & TODOs <a name="known-issues"></a>

- *TODO:* For now, when a token that begins a multi-line comment appears inside a string (in any supported language that supports both multi-line comments and strings) , `rlocc` cannot handle it and the results of LOC count for that whole file get calculated wrong with high probability.

- Nested comments are not handled. (I don't think I'm gonna fix this, since it's not really a use case for me.)
