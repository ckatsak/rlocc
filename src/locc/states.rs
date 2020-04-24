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

/// TODO Implementation?
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
    fn set_state(&mut self, state_no: usize) {
        #[cfg(debug_assertions)]
        eprintln!("[LOCStateMachine][set_state] to {}", state_no);
        self.state = Some(Rc::clone(&self.states[state_no]));
    }

    /// TODO Documentation
    #[inline]
    pub fn reset(&mut self) {
        self.set_state(STATE_INITIAL);
    }

    /// TODO Implementation?
    /// TODO Documentation
    #[inline]
    pub fn process(&mut self, ps: &mut ParsingState, res: &mut CountResult) {
        if let Some(state) = self.state.take() {
            let state = state.borrow_mut();
            while state.process(self, ps, res) {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[{}:{}][LOCStateMachine][process] state.process loop iteration",
                    file!(),
                    line!()
                );
            }
            //while state.process(self, ps, res) && !ps.curr_line_counted {}
            // TODO? ^ Maybe this could be more Rusty if State.process() returned a State enum
            // instead of a bool, so we wouldn't need the ParsingState.curr_line_counted either.
            // But how would this be performance-wise? How does memory allocation work for enums?

            // FIXME It is probably *here* where we should also update self.state, somehow...
            //self.set_state(/* TODO */);
            //loop {
            //    let new_state = state.process(self, ps, res);
            //    if new_state != state {
            //        self.set_state()
            //    }
            //}
        }
        //while let Some(state) = self.state.take() {
        //    let s = state.borrow_mut();
        //    while s.process(self, ps, res) {}
        //    //drop(s);
        //    //self.state = Some(state);
        //}
    }
}

// FIXME This doesn't look very Rusty...
const STATE_INITIAL: usize = 0;
const STATE_MULTI_LINE_COMMENT: usize = 1;
const STATE_CODE: usize = 2;
const NUM_STATES: usize = 3;

/// TODO Implementation?
/// TODO Documentation
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

/// TODO Implementation?
/// TODO Documentation
///
/// Searches `line` for a multi-line comment token (starting or ending, depending on the given
/// `self::MultiLine` variant) and returns an `std::Option` that wraps the index at which the first
/// token was found, along with the token.
fn find_multiline(kind: MultiLine, line: &str, lang: &Language) -> Option<(usize, &'static str)> {
    let mut ret: (usize, &str) = (line.len(), "");
    let tokens: &[&str] = match kind {
        MultiLine::Start => &lang.multiline_comment_start_tokens,
        MultiLine::End => &lang.multiline_comment_end_tokens,
    };
    for &token in tokens {
        if let Some(index) = line.find(token) {
            if index < ret.0 {
                ret = (index, token);
            }
        }
    }
    if ret.0 != line.len() {
        Some(ret)
    } else {
        None
    }
}

enum MultiLine {
    Start,
    End,
}

/// The current state of the LOC counting procedure of a `self::LOCStateMachine`.
/// The state of a Worker may change multiple times while processing a single line.
///
/// TODO: Implementation?
trait State: Sync + Send + std::fmt::Debug {
    /// TODO: Implementation?
    /// TODO: Documentation
    ///
    /// Returns false when the State is done processing the current line and is ready to move
    /// to the next one, or true when there is more processing to be done in the same line.
    fn process(
        &self,
        sm: &mut LOCStateMachine,
        ps: &mut ParsingState,
        cr: &mut CountResult,
    ) -> bool;

    /// Returns the unique index of this State in every `self::LOCStateMachine`'s `states` array.
    fn get_state_no(&self) -> usize;
}

/// The initial `State` in which all `self::LOCStateMachine` start in.
#[derive(Debug)]
struct StateInitial {}

impl State for StateInitial {
    #[inline]
    fn get_state_no(&self) -> usize {
        STATE_INITIAL
    }

    /// TODO: Implementation?
    /// TODO: Documentation
    fn process(
        &self,
        sm: &mut LOCStateMachine,
        ps: &mut ParsingState,
        cr: &mut CountResult,
    ) -> bool {
        #[cfg(debug_assertions)]
        eprintln!("[STATE_INITIAL][process]");

        // Whitespace must have already been trimmed in ps.curr_line when
        // populated in Worker.process_line().
        let line_rem = ps.curr_line.unwrap();
        if line_rem.is_empty() {
            // Count the line as blank and move on, but remain in StateInitial.
            cr.blank += 1;
            ps.curr_line_counted = true;
            sm.set_state(self.get_state_no());
            return false; // move on to the next line
        }

        // Find the index of the first inline comment token, if any.
        let first_inline_index = find_inline(&line_rem, ps.curr_lang);
        if let Some(0) = first_inline_index {
            // If the inline comment token is in the beginning of the line, count the
            // line as a comment, move on to the next line, but remain in StateInitial.
            cr.comments += 1;
            ps.curr_line_counted = true;
            sm.set_state(self.get_state_no());
            return false; // move on to the next line
        }

        // Find the index of the first multiline comment start token, if any.
        let first_multiline_start = find_multiline(MultiLine::Start, &line_rem, ps.curr_lang);
        if let Some((0, _)) = first_multiline_start {
            // If the multiline comment token is in the beginning of the line, don't count this
            // line yet (since we don't know where the comment ends), but change to StateMultiline.
            sm.set_state(STATE_MULTI_LINE_COMMENT);
            //return false;
            return true; // keep processing the same line
        }

        // If the line hasn't been blank and doesn't start with an inline or a multiline
        // comment, then count it as code, and figure out the next state.
        cr.code += 1;
        ps.curr_line_counted = true;
        if first_inline_index.is_none() && first_multiline_start.is_none() {
            // The line is pure code, so...
            sm.set_state(STATE_CODE); // change to StateCode
            false // and just move on to the next line
        } else if first_inline_index.is_none() && first_multiline_start.is_some() {
            // The line doesn't contain an inline comment token, and it contains a starting
            // multiline comment token. Therefore, line remainder is updated, state is changed
            // to StateMultiLineComment, and we have to keep processing the same line.
            if let Some((index, token)) = first_multiline_start {
                ps.curr_line.replace(&line_rem[index + token.len()..]); // update line remainder
            }
            sm.set_state(STATE_MULTI_LINE_COMMENT); // change state
            true // keep processing the same line
        } else if first_inline_index.is_some() && first_multiline_start.is_none() {
            // The line starts with code and ends with some inline comment, so...
            sm.set_state(STATE_CODE); // change to StateCode
            false // move on to the next line
        } else if first_inline_index.is_some() && first_multiline_start.is_some() {
            // The line contains both a multiline comment starting token and an inline
            // comment token. If the latter precedes the first, it "invalidates" it.
            let first_inline_index = first_inline_index.unwrap();
            let (multi_start_index, multi_start_token) = first_multiline_start.unwrap();
            if multi_start_index < first_inline_index {
                // A multiline comment starts in this line (at some point, after the code), so...
                ps.curr_line // update line remainder
                    .replace(&line_rem[multi_start_index + multi_start_token.len()..]);
                sm.set_state(STATE_MULTI_LINE_COMMENT); // change state
                true // keep processing the same line
            } else {
                // The line starts with code and ends with some inline comment, so...
                sm.set_state(STATE_CODE); // change to StateCode
                false // move on to the next line
            }
        } else {
            panic!("UNREACHABLE");
        }
    }
}

/// TODO: Implementation
/// TODO: Documentation
#[derive(Debug)]
struct StateMultiLineComment {}

impl State for StateMultiLineComment {
    #[inline]
    fn get_state_no(&self) -> usize {
        STATE_MULTI_LINE_COMMENT
    }

    /// TODO: Implementation
    /// TODO: Documentation
    fn process(
        &self,
        _sm: &mut LOCStateMachine,
        _ps: &mut ParsingState,
        _cr: &mut CountResult,
    ) -> bool {
        #[cfg(debug_assertions)]
        eprintln!("[STATE_MULTI_LINE_COMMENT][process]");
        false
    }
}

/// TODO: Implementation
/// TODO: Documentation
#[derive(Debug)]
struct StateCode {}

impl State for StateCode {
    #[inline]
    fn get_state_no(&self) -> usize {
        STATE_CODE
    }

    /// TODO: Implementation?
    /// TODO: Documentation
    fn process(
        &self,
        sm: &mut LOCStateMachine,
        ps: &mut ParsingState,
        cr: &mut CountResult,
    ) -> bool {
        #[cfg(debug_assertions)]
        eprintln!("[STATE_CODE][process]");

        // Whitespace must have already been trimmed in ps.curr_line when
        // populated in Worker.process_line().
        let line_rem = ps.curr_line.unwrap();
        if line_rem.is_empty() {
            // Count the line as blank and move on, but remain in StateCode.
            cr.blank += 1;
            ps.curr_line_counted = true;
            sm.set_state(self.get_state_no());
            return false; // move on to the next line
        }

        // Find the index of the first inline comment token, if any.
        let first_inline_index = find_inline(&line_rem, ps.curr_lang);
        if let Some(0) = first_inline_index {
            // If the inline comment token is in the beginning of the line, count the
            // line as a comment, move on to the next line, but remain in StateCode.
            cr.comments += 1;
            ps.curr_line_counted = true;
            sm.set_state(self.get_state_no());
            return false; // move on to the next line
        }

        // Find the index of the first multiline comment start token, if any.
        let first_multiline_start = find_multiline(MultiLine::Start, &line_rem, ps.curr_lang);
        if let Some((0, _)) = first_multiline_start {
            // If the multiline comment token is in the beginning of the line, don't count this
            // line yet (since we don't know where the comment ends), but change to StateMultiline.
            sm.set_state(STATE_MULTI_LINE_COMMENT);
            //return false;
            return true; // keep processing the same line
        }

        // If the line hasn't been blank and doesn't start with an inline or a multiline
        // comment, then count it as code, and figure out the next state.
        cr.code += 1;
        ps.curr_line_counted = true;
        if first_inline_index.is_none() && first_multiline_start.is_none() {
            // The line is pure code, so...
            sm.set_state(STATE_CODE); // change to StateCode
            false // and just move on to the next line
        } else if first_inline_index.is_none() && first_multiline_start.is_some() {
            // The line doesn't contain an inline comment token, and it contains a starting
            // multiline comment token. Therefore, line remainder is updated, state is changed
            // to StateMultiLineComment, and we have to keep processing the same line.
            if let Some((index, token)) = first_multiline_start {
                ps.curr_line.replace(&line_rem[index + token.len()..]); // update line remainder
            }
            sm.set_state(STATE_MULTI_LINE_COMMENT); // change state
            true // keep processing the same line
        } else if first_inline_index.is_some() && first_multiline_start.is_none() {
            // The line starts with code and ends with some inline comment, so...
            sm.set_state(STATE_CODE); // change to StateCode
            false // move on to the next line
        } else if first_inline_index.is_some() && first_multiline_start.is_some() {
            // The line contains both a multiline comment starting token and an inline
            // comment token. If the latter precedes the first, it "invalidates" it.
            let first_inline_index = first_inline_index.unwrap();
            let (multi_start_index, multi_start_token) = first_multiline_start.unwrap();
            if multi_start_index < first_inline_index {
                // A multiline comment starts in this line (at some point, after the code), so...
                ps.curr_line
                    .replace(&line_rem[multi_start_index + multi_start_token.len()..]); // update line remainder
                sm.set_state(STATE_MULTI_LINE_COMMENT); // change state
                true // keep processing the same line
            } else {
                // The line starts with code and ends with some inline comment, so...
                sm.set_state(STATE_CODE); // change to StateCode
                false // move on to the next line
            }
        } else {
            panic!("UNREACHABLE");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::locc::languages::EXT_TO_LANG;

    #[derive(Debug)]
    struct A<'a> {
        sm: LOCStateMachine,
        ps: ParsingState<'a>,
        cr: CountResult,
    }

    impl A<'_> {
        #[inline]
        fn new(lang: &'static Language) -> Self {
            let mut sm = LOCStateMachine::new();
            sm.set_state(STATE_INITIAL);
            A {
                sm,
                ps: ParsingState::new(lang),
                cr: CountResult::new(lang.name),
            }
        }
    }

    #[test]
    fn state_initial_empty_line() {
        let mut a = A::new(EXT_TO_LANG.get(&"rs").unwrap());
        //eprintln!("{:#?}", a);
        let lines = vec!["", "    "];
        for line in &lines {
            a.ps.curr_line = Some(line);
            //eprintln!("ps = {:?}", a.ps);
            //eprintln!("sm = {:?}", a.sm);
            //eprintln!("ps.curr_line = {:?}", a.ps.curr_line);
            if let Some(sinit) = a.sm.state.take() {
                assert_eq!(
                    false,
                    sinit.borrow_mut().process(&mut a.sm, &mut a.ps, &mut a.cr)
                );
            }
            a.sm.set_state(STATE_INITIAL);
        }
        assert_eq!(lines.len(), a.cr.blank);
        //eprintln!("cr = {:?}", a.cr);
    }
}
