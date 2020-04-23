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

use std::env;
use std::path::PathBuf;

use num_cpus;

/// TODO: Documentation
#[derive(Debug)]
pub struct Config {
    pub paths: Vec<PathBuf>,
    pub num_threads: usize,
}

/// TODO: Documentation
impl Config {
    /// TODO: Documentation
    pub fn new<T>(args: T, num_threads: usize) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        let mut args = args.peekable();
        if args.peek().is_none() {
            return Err("no paths given");
        }

        Ok(Config {
            paths: args.map(PathBuf::from).collect(),
            num_threads: if num_threads > 1 {
                num_threads
            } else {
                num_cpus::get()
            },
        })
    }
}

/// TODO: Documentation
impl Default for Config {
    #[inline]
    fn default() -> Self {
        Config {
            paths: vec![env::current_dir().unwrap()],
            num_threads: num_cpus::get(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_default() {
        let args = vec!["skata", "re"];
        let c = Config::new(args.iter().map(|s| s.to_string()), 2).unwrap_or_default();
        eprintln!("config1: {:#?}", c);
        assert_eq!(
            c.paths,
            args.iter()
                .map(|arg| PathBuf::from_str(arg).unwrap())
                .collect::<Vec<PathBuf>>()
        );
        assert_eq!(c.num_threads, 2);

        let args: Vec<String> = vec![ /* no arguments given */ ];
        let c = Config::new(args.iter().map(|s| s.to_string()), 0).unwrap_or_default();
        eprintln!("config2: {:#?}", c);
        assert_eq!(c.paths, vec![env::current_dir().unwrap()]);
        assert_eq!(c.num_threads, num_cpus::get());
    }
}
