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

use super::count::Worker;
use super::languages::Language;

// FIXME This doesn't look very Rusty...
pub const STATE_INITIAL: usize = 0;
pub const STATE_MULTI_LINE_COMMENT: usize = 1;
pub const STATE_CODE: usize = 2;
pub const NUM_STATES: usize = 3;

///// TODO: Implementation
///// TODO: Documentation
//pub enum StateVariant {
//    StateInitial,
//    StateMultiLineComment,
//    StateCode,
//    //StateInitial(StateInitial),
//    //StateMultiLineComment(StateMultiLineComment),
//    //StateCode(StateCode),
//}

/// TODO Implementation
/// TODO Documentation
#[inline]
fn find_inline(line: &str, lang: &Language) -> Option<usize> {
    let mut ret: usize = line.len();
    for &token in lang.inline_comment_tokens {
        if let Some(index) = line.find(token) {
            if index < ret {
                ret = index;
            }
        }
    }
    if ret != line.len() {
        Some(ret)
    } else {
        None
    }
}

/// The current state of the LOC counting procedure of a `crate::locc::count::Worker`. The state of
/// a Worker may change multiple times while processing a single line.
///
/// TODO: Implementation?
pub trait State: Sync + Send {
    fn process(&self, worker: &mut Worker) -> bool;
}

/// The initial `State` in which all `crate::locc::count::Worker`s start in.
pub struct StateInitial {}

impl State for StateInitial {
    /// TODO: Implementation
    /// TODO: Documentation
    fn process(&self, worker: &mut Worker) -> bool {
        let line = worker.curr_line.trim_start();
        if line.is_empty() {
            return true;
        }
        let first_inline_tkn = find_inline(&line, worker.curr_lang.unwrap());
        if first_inline_tkn.is_none() {
            return true;
        }
        let first_inline_tkn = first_inline_tkn.unwrap();

        worker.set_state(STATE_CODE);

        false
    }
}

/// TODO: Implementation
/// TODO: Documentation
pub struct StateMultiLineComment {}

impl State for StateMultiLineComment {
    /// TODO: Implementation
    /// TODO: Documentation
    fn process(&self, _worker: &mut Worker) -> bool {
        false
    }
}

/// TODO: Implementation
/// TODO: Documentation
pub struct StateCode {}

impl State for StateCode {
    /// TODO: Implementation
    /// TODO: Documentation
    fn process(&self, _worker: &mut Worker) -> bool {
        false
    }
}
