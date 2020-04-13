// This file is part of rlocc.
//
// Copyright (C) 2020 Christos Katsakioris
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;

//use lazy_static::lazy_static;
//
//lazy_static! {
//    pub static ref EXT_TO_LANG: HashMap<&'static str, &'static Language> = {
//        let mut ext2lang = HashMap::new();
//        for lang in LANG_ARRAY.iter() {
//            for ext in lang.extensions {
//                ext2lang.insert(*ext, lang);
//            }
//        }
//        ext2lang
//    };
//}

use once_cell::sync::Lazy;

/// TODO: Documentation
pub static EXT_TO_LANG: Lazy<HashMap<&'static str, &'static Language>> = Lazy::new(|| {
    let mut ext2lang = HashMap::new();
    for lang in LANG_ARRAY.iter() {
        for ext in lang.extensions {
            ext2lang.insert(*ext, lang);
        }
    }
    ext2lang
});

/// TODO: Documentation
#[derive(Debug)]
pub struct Language {
    name: &'static str,
    extensions: &'static [&'static str],

    inline_comment_tokens: &'static [&'static str],
    multiline_comment_start_tokens: &'static [&'static str],
    multiline_comment_end_tokens: &'static [&'static str],
}

/// TODO: Documentation
pub static LANG_ARRAY: [Language; 43] = [
    Language {
        name: "Ada",
        extensions: &["adb", "ads"],
        inline_comment_tokens: &["--"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Assembly",
        extensions: &["asm", "s", "S"],
        inline_comment_tokens: &[";"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "AWK",
        extensions: &["awk"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "C",
        extensions: &["c", "h"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*/"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "C++",
        extensions: &[
            "cc", "hh", "C", "H", "cpp", "hpp", "cxx", "hxx", "c++", "h++",
        ],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "C#",
        extensions: &["cs"],
        inline_comment_tokens: &["//", "///"],
        multiline_comment_start_tokens: &["/*", "/**"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "D",
        extensions: &["d"],
        inline_comment_tokens: &["//", "///"],
        multiline_comment_start_tokens: &["/*", "/+"],
        multiline_comment_end_tokens: &["*/", "+/"],
    },
    Language {
        name: "Delphi",
        extensions: &["p", "pp", "pas"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["(*", "{"],
        multiline_comment_end_tokens: &["*)", "}"],
    },
    Language {
        name: "Dockerfile",
        extensions: &["Dockerfile"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Eiffel",
        extensions: &["e"],
        inline_comment_tokens: &["--"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Elixir",
        extensions: &["ex", "exs"],
        inline_comment_tokens: &["%"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Erlang",
        extensions: &["erl", "hrl"],
        inline_comment_tokens: &["%"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Go",
        extensions: &["go"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Haskell",
        extensions: &["hs", "lhs"],
        inline_comment_tokens: &["--"],
        multiline_comment_start_tokens: &["{-"], // nesting unsupported
        multiline_comment_end_tokens: &["-}"],   // nesting unsupported
    },
    Language {
        name: "HTML",
        extensions: &["html", "htm"],
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &["<!--"],
        multiline_comment_end_tokens: &["-->"],
    },
    Language {
        name: "Java",
        extensions: &["java"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*", "/**"],
        multiline_comment_end_tokens: &["/*"],
    },
    Language {
        name: "Javascript",
        extensions: &["js"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "JSON",
        extensions: &["json"],
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Kotlin",
        extensions: &["kt", "kts"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Lisp",
        extensions: &["lisp", "lsp", "l", "fasl"],
        inline_comment_tokens: &[";"],
        multiline_comment_start_tokens: &["#|"],
        multiline_comment_end_tokens: &["|#"],
    },
    Language {
        name: "Makefile",
        extensions: &["Makefile"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "MAL",
        extensions: &["mal"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Markdown",
        extensions: &["md"],
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Matlab",
        extensions: &["m"],
        inline_comment_tokens: &["%"],
        multiline_comment_start_tokens: &["%{"],
        multiline_comment_end_tokens: &["}%"],
    },
    Language {
        name: "OCaml",
        extensions: &["ml", "mli"],
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &["(*"], // nesting unsupported
        multiline_comment_end_tokens: &["*)"],   // nesting unsupported
    },
    Language {
        name: "OpenCL",
        extensions: &["cl"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Perl",
        extensions: &["pl", "pm", "t", "pod"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &["=begin"], // __END__ unsupport
        multiline_comment_end_tokens: &["=cut"],
    },
    Language {
        name: "PHP",
        extensions: &["php"],
        inline_comment_tokens: &["#", "//"],
        multiline_comment_start_tokens: &["/*", "/**"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "PowerShell",
        extensions: &["ps1"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &["<#"],
        multiline_comment_end_tokens: &["#>"],
    },
    Language {
        name: "Protocol Buffers",
        extensions: &["proto"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Python",
        extensions: &["py"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[r#"""""#, "'''"],
        multiline_comment_end_tokens: &[r#"""""#, "'''"],
    },
    Language {
        name: "R",
        extensions: &["r", "R", "RData", "rds", "rda"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Ruby",
        extensions: &["rb"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &["=begin"],
        multiline_comment_end_tokens: &["=end"],
    },
    Language {
        name: "Rust",
        extensions: &["rs, rlib"],
        inline_comment_tokens: &["//", "///", "//!"],
        multiline_comment_start_tokens: &["/*", "/**", "/*!"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Scala",
        extensions: &["scala", "sc"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Scheme",
        extensions: &["scm", "ss"],
        inline_comment_tokens: &[";"],
        multiline_comment_start_tokens: &["#|"],
        multiline_comment_end_tokens: &["|#"],
    },
    Language {
        name: "Shell",
        extensions: &["sh", "bash", "zsh", "ksh", "csh"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "SQL",
        extensions: &["sql"],
        inline_comment_tokens: &["--"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "SML",
        extensions: &["sml"],
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &["(*"],
        multiline_comment_end_tokens: &["*)"],
    },
    Language {
        name: "TeX",
        extensions: &["tex"],
        inline_comment_tokens: &["%"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "plain text",
        extensions: &["txt"],
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Tcl",
        extensions: &["tcl", "tbc"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "YAML",
        extensions: &["yaml", "yml"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn print_lang_array() {
        //eprintln!("{:#?}", languages::LANG_ARRAY);
        for (i, lang) in LANG_ARRAY.iter().enumerate() {
            eprintln!("{}: {:#?}", i, lang);
        }
    }

    #[test]
    fn dummy_check_hashmap() {
        for lang in LANG_ARRAY.iter() {
            for ext in lang.extensions {
                assert_eq!(EXT_TO_LANG.get(ext).unwrap().name, lang.name);
                eprintln!("{} --> {}", *ext, lang.name);
            }
        }
    }

    #[test]
    fn print_in_threads() {
        eprintln!("PARENT:\tLANG_ARRAY[13] = {:#?}", LANG_ARRAY[13]);

        let handle = thread::spawn(|| {
            eprintln!("CHILD:\tHey!");
            eprintln!("CHILD:\tLANG_ARRAY[14] = {:#?}", LANG_ARRAY[14]);
            eprintln!("CHILD:\tDone!");
        });

        thread::sleep(Duration::from_millis(50));
        eprintln!("PARENT:\tLANG_ARRAY[15] = {:#?}", LANG_ARRAY[15]);

        handle.join().unwrap();
        eprintln!("PARENT:\tDone!");
    }
}
