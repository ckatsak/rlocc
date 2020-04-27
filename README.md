# rlocc


Just another blazing fast LOC counter.

Largely works, but it is not finished yet. Check the known issues below if you wish to be sure whether you wish to use it.


### Disclaimer

This is my first ever project in Rust; developed while reading the Rust Book!
My sincere apologies to the Rust community for possibly abusing the language; at the time `rlocc` is written I've been still fighting the borrow checker.



## Known issues & TODOs

- *TODO:* Implement count for Makefile, Dockerfile, gitignore, License

- *TODO:* some sort of macro for cleaner logging with conditional compilation

- *TODO:* This README...

- For now, when a token that begins a multi-line comment appears inside a string (in any supported language that supports both multi-line comments and strings) , `rlocc` cannot handle it, and the results of LOC count for that whole file get calculated wrong with high probability.

- Nested comments are not handled. (I don't think I'm gonna fix this, since it's not a use case for me.)
