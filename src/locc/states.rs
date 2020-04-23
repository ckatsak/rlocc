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

use std::cell::RefCell;
use std::rc::Rc;

use super::count::{CountResult, ParsingState};
use super::languages::Language;

/// TODO Implementation
/// TODO Documentation
#[derive(Debug)]
pub struct LOCStateMachine {
    state: Option<Rc<RefCell<dyn State>>>,
    states: [Rc<RefCell<dyn State>>; NUM_STATES],
}

impl LOCStateMachine {
    /// TODO Documentation
    #[inline]
    pub fn new() -> Self {
        LOCStateMachine {
            state: None,
            states: [
                Rc::new(RefCell::new(StateInitial {})),
                Rc::new(RefCell::new(StateMultiLineComment {})),
                Rc::new(RefCell::new(StateCode {})),
            ],
        }
    }

    /// TODO Documentation
    #[inline]
    pub fn set_state(&mut self, state_no: usize) {
        self.state = Some(Rc::clone(&self.states[state_no]));
    }

    /// TODO Implementation?
    /// TODO Documentation
    #[inline]
    pub fn reset(&mut self) {
        self.set_state(STATE_INITIAL);
    }

    /// TODO Implementation
    /// TODO Documentation
    #[inline]
    pub fn process(&mut self, ps: &mut ParsingState, res: &mut CountResult) {
        if let Some(state) = self.state.take() {
            let state = state.borrow_mut();
            while state.process(self, ps, res) {}
        }
    }
}

// FIXME This doesn't look very Rusty...
const STATE_INITIAL: usize = 0;
const STATE_MULTI_LINE_COMMENT: usize = 1;
const STATE_CODE: usize = 2;
const NUM_STATES: usize = 3;

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

/// TODO Implementation?
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
trait State: Sync + Send + std::fmt::Debug {
    /// TODO: Implementation?
    /// TODO: Documentation
    ///
    /// Returns false when the State is done processing the current line and is ready to move to
    /// the next one.
    fn process(
        &self,
        sm: &mut LOCStateMachine,
        ps: &mut ParsingState,
        res: &mut CountResult,
    ) -> bool;
}

/// The initial `State` in which all `crate::locc::count::Worker`s start in.
#[derive(Debug)]
struct StateInitial {}

impl State for StateInitial {
    /// TODO: Implementation
    /// TODO: Documentation
    fn process(
        &self,
        sm: &mut LOCStateMachine,
        ps: &mut ParsingState,
        res: &mut CountResult,
    ) -> bool {
        let line = ps.curr_line.unwrap();
        if line.is_empty() {
            return false;
        }
        let first_inline_tkn = find_inline(&line, ps.curr_lang);
        if first_inline_tkn.is_none() {
            return false;
        }
        let first_inline_tkn = first_inline_tkn.unwrap();

        //worker.set_state(STATE_CODE);

        false
    }
}

/// TODO: Implementation
/// TODO: Documentation
#[derive(Debug)]
struct StateMultiLineComment {}

impl State for StateMultiLineComment {
    /// TODO: Implementation
    /// TODO: Documentation
    fn process(
        &self,
        _sm: &mut LOCStateMachine,
        _ps: &mut ParsingState,
        _res: &mut CountResult,
    ) -> bool {
        false
    }
}

/// TODO: Implementation
/// TODO: Documentation
#[derive(Debug)]
struct StateCode {}

impl State for StateCode {
    /// TODO: Implementation
    /// TODO: Documentation
    fn process(
        &self,
        _sm: &mut LOCStateMachine,
        _ps: &mut ParsingState,
        _res: &mut CountResult,
    ) -> bool {
        false
    }
}
