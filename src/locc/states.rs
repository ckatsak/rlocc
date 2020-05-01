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
                Rc::new(RefCell::new(StateMultiLineComment {
                    tkn_buf: String::with_capacity(8), // > 6 == longest multiline comm token currently
                    ready: false,
                })),
                Rc::new(RefCell::new(StateCode {})),
            ],
        }
    }

    /// TODO Documentation
    #[inline]
    fn set_state(&mut self, state_no: usize) {
        rlocc_dbg_log!("[LOCStateMachine][set_state] to {}", state_no);
        self.state = Some(Rc::clone(&self.states[state_no]));
    }

    /// TODO Documentation
    #[inline]
    pub fn reset(&mut self) {
        self.set_state(STATE_CODE);
    }

    /// TODO Documentation
    #[inline]
    pub fn process(&mut self, ps: &mut ParsingState, res: &mut CountResult) {
        loop {
            rlocc_dbg_log!("[LOCStateMachine][process] re-entering loop...");
            if let Some(state) = self.state.take() {
                let mut s = state.borrow_mut();
                if !s.process(self, ps, res) {
                    rlocc_dbg_log!(
                        "[LOCStateMachine][process] state.process loop iteration\tstate = {:?}",
                        state
                    );
                    break;
                }
            }
        }
    }
}

// FIXME This doesn't look very Rusty...
const STATE_MULTI_LINE_COMMENT: usize = 0;
const STATE_CODE: usize = 1;
const NUM_STATES: usize = 2;

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

/// TODO More Documentation
///
/// Searches `line` for a multi-line comment token (starting or ending, depending on the given
/// `self::MultiLine` variant) and returns an `std::Option` that wraps the index at which the first
/// token was found, along with the token.
fn find_multiline(
    kind: &mut MultiLine,
    line: &str,
    lang: &Language,
) -> Option<(usize, &'static str)> {
    let mut ret: (usize, &str) = (line.len(), "");

    let tokens: &[&str] = match kind {
        MultiLine::Start => &lang.multiline_comment_start_tokens,
        MultiLine::End((ref mut in_tkn, ref mut ready)) => {
            if lang.name == "Delphi"
                || lang.name == "HTML"
                || lang.name == "Lua"
                || lang.name == "Pascal"
                || lang.name == "Perl"
                || lang.name == "Ruby"
                || lang.name == "XML"
            {
                &lang.multiline_comment_end_tokens
            } else {
                if !*ready {
                    // Construct the multi-line comment end token, without allocating extra memory:

                    // First, take a reference to the given starting token str, the
                    // value of which should now be the same as the value of in_tkn.
                    let start_tkn: &str = lang.multiline_comment_start_tokens[lang
                        .multiline_comment_start_tokens
                        .iter()
                        .position(|t| t == in_tkn)
                        .unwrap()];

                    // Then calculate the corresponding ending token and assign it to in_tkn.
                    in_tkn.clear();
                    in_tkn.extend(start_tkn.chars().rev().map(|c| match c {
                        '(' => ')',
                        '{' => '}',
                        '<' => '>',
                        _ => c,
                    }));

                    // XXX This was moved in the search() closure:
                    ////let i = lang
                    ////    .multiline_comment_end_tokens
                    ////    .iter()
                    ////    .position(|&t| t == in_tkn)
                    ////    .unwrap(); // FIXME

                    ////// ...and subslice that single element.
                    ////&lang.multiline_comment_end_tokens[i..i + 1]

                    *ready = &true;
                }

                // Now, find the index of the ending token in the corresponding slice...
                // FIXME This may not be actually needed, but then cannot return &'static
                rlocc_dbg_log!(
                    "[find_multiline] now searching for registered ending token {:?} in {:?}",
                    in_tkn,
                    lang
                );
                let i = lang
                    .multiline_comment_end_tokens
                    .iter()
                    .position(|t| t == in_tkn);
                ////    .position(|&t| t == in_tkn)
                ////    .unwrap(); // FIXME

                ////// ...and subslice that single element.
                ////&lang.multiline_comment_end_tokens[i..i + 1]
                //
                // FIXME This  ^^^  is the correct way to handle this.
                //       But this  vvv  is the workaround until StateString.
                //       In other words, the workaround counts wrong if a starting multi-line
                //       comment token is found within a string in the source file examined.
                //       This makes it identical to glocc, i.e., wrong in the same way.
                if let Some(i) = i {
                    &lang.multiline_comment_end_tokens[i..i + 1]
                } else {
                    &lang.multiline_comment_end_tokens
                }
            }
        }
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

/// The current state of the LOC counting procedure of a `self::LOCStateMachine`.
/// The state of a Worker may change multiple times while processing a single line.
trait State: Sync + Send + std::fmt::Debug {
    /// TODO More Documentation
    ///
    /// Returns false when the State is done processing the current line and is ready to move
    /// to the next one, or true when there is more processing to be done in the same line.
    fn process(
        &mut self,
        sm: &mut LOCStateMachine,
        ps: &mut ParsingState,
        cr: &mut CountResult,
    ) -> bool;

    /// Returns the unique index of this State in every `self::LOCStateMachine`'s `states` array.
    fn get_state_no(&self) -> usize;

    fn set_token(&mut self, token: &'static str);
}

/// TODO More Documentation
#[derive(Debug)]
struct StateMultiLineComment {
    /// A state-local buffer to hold either the starting or the ending multi-line comment token
    /// (found or searched for, respectively).
    tkn_buf: String,

    /// Indicates whether the persisted `self.tkn_buf` is ready to be searched (i.e., the ending
    /// token has been already calculated and stored there) or if the calculation is still needed.
    ready: bool,
}

/// TODO Documentation
#[derive(Debug)]
enum MultiLine<'a> {
    Start,
    End((&'a mut String, &'a bool)),
}

impl State for StateMultiLineComment {
    #[inline]
    fn get_state_no(&self) -> usize {
        STATE_MULTI_LINE_COMMENT
    }

    #[inline]
    fn set_token(&mut self, token: &'static str) {
        self.tkn_buf.truncate(0); // btw, looks like that's what String.clear() boils down too
        self.tkn_buf.push_str(token);
        debug_assert_eq!(self.tkn_buf.len(), token.len());
    }

    /// TODO Documentation
    fn process(
        &mut self,
        sm: &mut LOCStateMachine,
        ps: &mut ParsingState,
        cr: &mut CountResult,
    ) -> bool {
        rlocc_dbg_log!(
            "[STATE_MULTI_LINE_COMMENT][process] state = {:?}\tline ({}) = {}",
            self,
            cr.total + 1,
            ps.curr_line.unwrap().trim_end()
        );

        // XXX First, trim the trailing whitespace too:
        let line_rem = ps.curr_line.unwrap().trim_end();
        if line_rem.is_empty() {
            rlocc_dbg_log!("[STATE_MULTI_LINE_COMMENT][process] line_rem empty - leaving!");
            // Count the line as (blank|comment) and move on to the next one,
            // but remain in StateMultiLineComment.
            if !ps.curr_line_counted {
                rlocc_dbg_log!("[STATE_MULTI_LINE_COMMENT] counting line as comment|blank");
                cr.comments += 1;
                //cr.blank += 1; // FIXME? comment or blank?
                ps.curr_line_counted = true;
            }
            sm.set_state(self.get_state_no()); // FIXME? refactor for StateString or use const?
            return false; // move on to the next line
        }

        rlocc_dbg_log!("starting token = {:?}", self.tkn_buf);
        let mut end_variant = MultiLine::End((&mut self.tkn_buf, &mut self.ready));
        let first_multiline_end = find_multiline(&mut end_variant, &line_rem, ps.curr_lang);
        if let MultiLine::End((_, r)) = end_variant {
            self.ready = *r;
        }
        rlocc_dbg_log!(
            "first_multiline_end = {:?}, self.tkn_buf = {:?}",
            first_multiline_end,
            self.tkn_buf
        );

        // Since the line is not blank, if it does not contain the ending token we should
        // count it as a comment and move on to the next line, remaining in StateMultiLine.
        if first_multiline_end.is_none() {
            if !ps.curr_line_counted {
                rlocc_dbg_log!("[STATE_MULTI_LINE_COMMENT] counting multi-line comment line");
                cr.comments += 1;
                ps.curr_line_counted = true;
            }
            sm.set_state(self.get_state_no()); // FIXME? refactor for StateString or use const?
            return false;
        }

        // The ending token has been found within this line, but we need to make sure that
        // there is no code in the same line before actualling counting it as a comment.
        let (index, token) = first_multiline_end.unwrap();
        // If the ending token is at the end of the line remainder, then we are good
        // to count the line as a comment and move on to the next line, in StateCode.
        if index + token.len() == line_rem.len() {
            if !ps.curr_line_counted {
                rlocc_dbg_log!("[STATE_MULTI_LINE_COMMENT] counting multi-line comment line");
                cr.comments += 1;
                ps.curr_line_counted = true;
            }
            sm.set_state(STATE_CODE);
            self.ready = false; // XXX Forget stored token
            return false;
        }
        // If the ending token is not at the end of the line remainder, we probably should
        // not count the line a comment, and pass the remainder to StateCode instead.
        ps.curr_line.replace(&line_rem[index + token.len()..]);
        sm.set_state(STATE_CODE);
        self.ready = false; // XXX Forget stored token
        true
    }
}

/// TODO Documentation
#[derive(Debug)]
struct StateCode {}

impl State for StateCode {
    #[inline]
    fn get_state_no(&self) -> usize {
        STATE_CODE
    }

    #[inline]
    fn set_token(&mut self, _: &str) {}

    /// TODO: Documentation
    fn process(
        &mut self,
        sm: &mut LOCStateMachine,
        ps: &mut ParsingState,
        cr: &mut CountResult,
    ) -> bool {
        rlocc_dbg_log!(
            "[STATE_CODE][process] line ({}) = {}",
            cr.total + 1,
            ps.curr_line.unwrap().trim_end()
        );

        // Whitespace must have already been trimmed in ps.curr_line when
        // populated in Worker.process_line().
        let line_rem = ps.curr_line.unwrap();
        if line_rem.is_empty() {
            // Count the line as blank and move on, but remain in StateCode.
            rlocc_dbg_log!("[STATE_CODE][process] counting line as blank");
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
            rlocc_dbg_log!("[STATE_CODE][process] counting line as comment");
            cr.comments += 1;
            ps.curr_line_counted = true;
            sm.set_state(self.get_state_no());
            return false; // move on to the next line
        }

        // Find the index of the first multiline comment start token, if any.
        let first_multiline_start = find_multiline(&mut MultiLine::Start, &line_rem, ps.curr_lang);
        if let Some((0, token)) = first_multiline_start {
            // If the multiline comment token is in the beginning of the line, don't count this
            // line yet (since we don't know where the comment ends), but change to StateMultiline,
            // after updating the line remainder to look past the found token.
            ps.curr_line.replace(&line_rem[token.len()..]); // update line remainder
            sm.set_state(STATE_MULTI_LINE_COMMENT);
            sm.states[STATE_MULTI_LINE_COMMENT]
                .borrow_mut()
                .set_token(token);
            return true; // keep processing the same line
        }

        // If the line hasn't been blank and doesn't start with an inline or a multiline
        // comment, then count it as code, and figure out the next state.
        if !ps.curr_line_counted {
            // Don't count it twice if it has been already counted.
            rlocc_dbg_log!("[STATE_CODE][process] counting line as code");
            cr.code += 1;
            ps.curr_line_counted = true;
        }
        if first_inline_index.is_none() && first_multiline_start.is_none() {
            // The line is pure code, so...
            sm.set_state(STATE_CODE); // change to StateCode
            false // and just move on to the next line
        } else if first_inline_index.is_none() && first_multiline_start.is_some() {
            // The line doesn't contain an inline comment token, and it contains a starting
            // multiline comment token. Therefore, line remainder is updated, state is changed
            // to StateMultiLineComment, and we have to keep processing the same line.
            let (index, token) = first_multiline_start.unwrap();
            ////if let Some((index, token)) = first_multiline_start {
            ////    ps.curr_line.replace(&line_rem[index + token.len()..]); // update line remainder
            ////}
            ps.curr_line.replace(&line_rem[index + token.len()..]); // update line remainder
            sm.set_state(STATE_MULTI_LINE_COMMENT); // change state
            sm.states[STATE_MULTI_LINE_COMMENT]
                .borrow_mut()
                .set_token(token);
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
                sm.states[STATE_MULTI_LINE_COMMENT]
                    .borrow_mut()
                    .set_token(multi_start_token);
                true // keep processing the same line
            } else {
                // The line starts with code and ends with some inline comment, so...
                sm.set_state(STATE_CODE); // change to StateCode
                false // move on to the next line
            }
        } else {
            unreachable!();
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
            sm.set_state(STATE_CODE);
            A {
                sm,
                ps: ParsingState::new(lang),
                cr: CountResult::new(lang.name),
            }
        }
    }

    //#[test]
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
            a.sm.set_state(STATE_CODE);
        }
        assert_eq!(lines.len(), a.cr.blank);
        //eprintln!("cr = {:?}", a.cr);
    }
}
