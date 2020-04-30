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

#[macro_use]
macro_rules! rlocc_dbg_log {
    ( $fmt:expr ) => {
        #[cfg(debug_assertions)]
        eprintln!( concat!( "[", file!(), ":", line!(), "]\t", $fmt));
    };

    ( $fmt:expr, $( $s:expr ),+ ) => {
        #[cfg(debug_assertions)]
        eprintln!( concat!( "[", file!(), ":", line!(), "]\t", $fmt), $($s),+ );
    };
}

mod config;
mod count;
mod languages;
mod states;

pub use self::config::Config;
pub use self::count::{count_all, LOCCount};
pub use self::languages::guess_language;
