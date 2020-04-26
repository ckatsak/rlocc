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
use std::io;

use rlocc::locc::{self, Config, LOCCount};

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let config = Config::new(args.into_iter(), 0).unwrap_or_default();
    #[cfg(debug_assertions)]
    eprintln!("{:#?}", config);

    let ret = locc::count_all(&config)?;
    print_results(&ret)?;
    Ok(())
}

#[inline(always)]
fn print_results(loccount: &LOCCount) -> io::Result<()> {
    // FIXME Buffered IO ?
    println!("{}", loccount);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_arg() {
        let orig_args = env::args().collect::<Vec<_>>();
        let mut skip_args = env::args().skip(1).collect::<Vec<_>>();
        eprintln!("Original arguments: {:#?}", orig_args);
        eprintln!("Skipped arguments: {:#?}", skip_args);

        assert_eq!(
            orig_args
                .iter()
                .map(|arg| arg.to_string())
                .skip(1)
                .collect::<Vec<_>>(),
            skip_args
        );

        skip_args.push("yolo".to_owned());
        assert_ne!(orig_args.into_iter().skip(1).collect::<Vec<_>>(), skip_args);
    }
}
