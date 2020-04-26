# rlocc

My first ever project in Rust; developed while reading the Rust Book!

My sincere apologies to the Rust community for possibly abusing the language; at the time `rlocc` is written I've been still fighting the borrow checker.



## Known issues & TODOs

- *TODO:* Makefile, Dockerfile, gitignore, License

- *TODO:* This README...

- For now, when a token that begins a multi-line comment appears inside a string (in any supported language that supports both multi-line comments and strings) , `rlocc` cannot handle it, and the results of LOC count for that whole file get calculated wrong with high probability.

- Nested comments are not handled. (I don't think I'm gonna fix this, since it's not a use case for me.)
