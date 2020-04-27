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
use std::io;
use std::path::Path;

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

/// This function attempts to figure out whether the given path corresponds to a file whose
/// contained source code is written in a language supported by rlocc.
///
/// If it is, then the function returns a tuple containing the extension of the file (which was
/// used to guess the language) and a reference to the associated `rlocc::languages::Language`
/// struct.
pub fn guess_language<'a, 'b, P>(path: &'a P) -> io::Result<(&'a str, &'b Language)>
where
    P: 'a + AsRef<Path>,
{
    let path = path.as_ref();
    let ext = path
        .extension()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid extension in {}", path.display()),
            )
        })?
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Extension contains invalid UTF-8"))?;

    let lang = EXT_TO_LANG.get(&ext).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Unsupported extension '{}'", ext),
        )
    })?;

    Ok((ext, *lang))
}

/// TODO: Documentation
pub static VCS_DIRECTORIES: &[&str] = &[
    ".bzr", // bazaar
    ".cvs", // cvs
    ".git", // git
    ".hg",  // mercurial
    ".svn", // subversion
];

/// TODO: Documentation
#[inline]
pub fn is_vcs<P>(path: &P) -> bool
where
    P: AsRef<Path>,
{
    if let Some(basename) = path.as_ref().file_name() {
        if let Some(basename) = basename.to_str() {
            if VCS_DIRECTORIES.contains(&basename) {
                return true;
            }
        }
    }
    false
}

/// TODO: Documentation
#[derive(Debug)]
pub struct Language {
    pub name: &'static str,
    pub extensions: &'static [&'static str],

    pub inline_comment_tokens: &'static [&'static str],
    pub multiline_comment_start_tokens: &'static [&'static str],
    pub multiline_comment_end_tokens: &'static [&'static str],
}

/// TODO: Documentation
pub static LANG_ARRAY: [Language; 74] = [
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
        name: "Autoconf",
        extensions: &["in"],
        inline_comment_tokens: &["dnl", "#"],
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
        name: "Batch",
        extensions: &["bat"],
        inline_comment_tokens: &["REM", "::"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "C",
        extensions: &["c"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "C++",
        extensions: &["cc", "C", "cpp", "cxx", "c++"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "C/C++ Header",
        extensions: &["h", "hh", "H", "hpp", "hxx", "h++"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "C#",
        extensions: &["cs"],
        inline_comment_tokens: &["//", "///"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Clojure",
        extensions: &["clj", "cljs", "cljc", "edn"],
        inline_comment_tokens: &[";"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "CMake",
        extensions: &["cmake"], // FIXME CMakeLists.txt sadly goes to Plain Text
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "COBOL",
        extensions: &["cbl", "cob", "cpy", "cobol"],
        inline_comment_tokens: &["*>"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Comma-Separated Values",
        extensions: &["csv"],
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "CSS",
        extensions: &["css"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
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
        name: "Dart",
        extensions: &["dart"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Delphi",
        extensions: &["p", "pp"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["(*", "{"],
        multiline_comment_end_tokens: &["*)", "}"],
    },
    Language {
        name: "Dockerfile",
        extensions: &["Dockerfile"], // FIXME
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
        name: "Elm",
        extensions: &["elm"],
        inline_comment_tokens: &["--"],
        multiline_comment_start_tokens: &["{-"], // no nested
        multiline_comment_end_tokens: &["-}"],   // no nested
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
        name: "F#",
        extensions: &["fs", "fsi", "fsx", "fsscript"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["(*"],
        multiline_comment_end_tokens: &["*)"],
    },
    Language {
        name: ".gitignore",
        extensions: &[".gitignore"], // FIXME
        inline_comment_tokens: &["#"],
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
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
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
        name: "Julia",
        extensions: &["jl"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &["#="],
        multiline_comment_end_tokens: &["=#"],
    },
    Language {
        name: "Jupyter",
        extensions: &["ipynb", "jpynb"],
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
        name: "License",
        extensions: &["LICENSE", "COPYING"], // FIXME
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Lisp",
        extensions: &["lisp", "lsp", "fasl"],
        inline_comment_tokens: &[";"],
        multiline_comment_start_tokens: &["#|"],
        multiline_comment_end_tokens: &["|#"],
    },
    Language {
        name: "Lua",
        extensions: &["lua"],
        inline_comment_tokens: &["--"],
        multiline_comment_start_tokens: &["--[["], // NOTE All the funny weird stuff though
        multiline_comment_end_tokens: &["]]"],     // are not supported, including nesting.
    },
    Language {
        name: "Makefile",
        extensions: &["Makefile", "am"], // FIXME
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
        name: "Nim",
        extensions: &["nim"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &["#["],
        multiline_comment_end_tokens: &["]#"],
    },
    Language {
        name: "Nix",
        extensions: &["nix"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
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
        name: "Pascal",
        extensions: &["pas"],
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &["(*", "{"], // (* to } and { to *) are
        multiline_comment_end_tokens: &["*)", "}"],   // valid too, as they should
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
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Plain Text",
        extensions: &["txt", "text"],
        inline_comment_tokens: &[],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Pony",
        extensions: &["pony"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
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
        name: "ReStructuredText",
        extensions: &["rst"],
        inline_comment_tokens: &[],
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
        extensions: &["rs", "rlib"],
        inline_comment_tokens: &["//"], //, "///", "//!"],
        multiline_comment_start_tokens: &["/*"],
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
        name: "Sed",
        extensions: &["sed"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "Shell",
        extensions: &["sh", "bash", "zsh", "ksh", "csh"],
        inline_comment_tokens: &["#"],
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
        name: "Solidity",
        extensions: &["sol"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "SQL",
        extensions: &["sql"],
        inline_comment_tokens: &["--"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Swift",
        extensions: &["swift"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "TeX",
        extensions: &["tex"],
        inline_comment_tokens: &["%"],
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
        name: "TOML",
        extensions: &["toml"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &[],
        multiline_comment_end_tokens: &[],
    },
    Language {
        name: "TypeScript",
        extensions: &["ts"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "V",
        extensions: &["v"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "Vala",
        extensions: &["vala"],
        inline_comment_tokens: &["//"],
        multiline_comment_start_tokens: &["/*"],
        multiline_comment_end_tokens: &["*/"],
    },
    Language {
        name: "VimL",
        extensions: &["vim"],
        inline_comment_tokens: &["\""],
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
    Language {
        name: "XML",
        extensions: &["xml"],
        inline_comment_tokens: &["#"],
        multiline_comment_start_tokens: &["<!--"],
        multiline_comment_end_tokens: &["-->"],
    },
    Language {
        name: "Zig",
        extensions: &["zig"],
        inline_comment_tokens: &["//"],
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
