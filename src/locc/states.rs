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

/// The current state of the LOC counting procedure of a `crate::locc::count::Worker`. The state of
/// a Worker may change multiple times while processing a single line.
///
/// TODO: Implementation
pub trait State: Sync {
    fn process(&self, line: &str) -> bool;
}

/// The initial `State` in which all `crate::locc::count::Worker`s start in.
pub struct StateInitial {}

impl State for StateInitial {
    /// TODO: Implementation
    /// TODO: Documentation
    fn process(&self, line: &str) -> bool {
        false
    }
}

/// TODO: Implementation
/// TODO: Documentation
pub struct StateMultiLineComment {}

impl State for StateMultiLineComment {
    /// TODO: Implementation
    /// TODO: Documentation
    fn process(&self, line: &str) -> bool {
        false
    }
}

/// TODO: Implementation
/// TODO: Documentation
pub struct StateCode {}

impl State for StateCode {
    /// TODO: Implementation
    /// TODO: Documentation
    fn process(&self, line: &str) -> bool {
        false
    }
}
