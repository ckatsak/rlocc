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
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::ops;
use std::path::PathBuf;

use crossbeam_channel as chan;
use crossbeam_utils::thread;

use super::languages::{self, Language};
use super::states::*;
use super::Config;

/// TODO: Documentation
const BUF_SIZE: usize = 1 << 16;

/// TODO: Documentation
#[derive(Debug)]
pub struct LOCCount<'a>(HashMap<&'a str, (CountResult, usize)>);

impl<'a> ops::AddAssign<CountResult> for LOCCount<'a> {
    /// Add-assign a `self::CountResult` to the `self::LOCCount`.
    #[inline]
    fn add_assign(&mut self, rhs: CountResult) {
        self.0
            .entry(rhs.lang)
            .and_modify(|(cnt_res, num_files)| {
                *cnt_res += rhs;
                *num_files += 1;
            })
            .or_insert((rhs, 1));
    }
}

impl<'a, 'b: 'a> ops::AddAssign<LOCCount<'b>> for LOCCount<'a> {
    /// Add-assign a `self::LOCCount` to `self::LOCCount`.
    #[inline]
    fn add_assign(&mut self, rhs: LOCCount<'b>) {
        for (lang, content) in rhs.0.iter() {
            self.0
                .entry(lang)
                .and_modify(|(cnt_res, num_files)| {
                    *cnt_res += content.0;
                    *num_files += content.1;
                })
                .or_insert((content.0, content.1));
        }
    }
}

/// The result of counting a single file.
#[derive(Debug, Copy, Clone)]
pub struct CountResult {
    lang: &'static str,

    pub total: usize,
    pub code: usize,
    pub comments: usize,
    pub blank: usize,
}

//impl ops::Add<CountResult> for CountResult {
//    type Output = CountResult;
//
//    /// Add a `self::CountResult` to the `self::CountResult`.
//    fn add(mut self, rhs: CountResult) -> Self {
//        if cfg!(debug_assertions) {
//            // exclude the case of result aggregation
//            if self.lang != "Total" {
//                debug_assert_eq!(self.lang, rhs.lang);
//            }
//        }
//        self.total += rhs.total;
//        self.code += rhs.code;
//        self.comments += rhs.comments;
//        self.blank += rhs.blank;
//        self
//    }
//}

impl ops::AddAssign for CountResult {
    /// Add-assign a `self::CountResult` to the `self::CountResult`.
    fn add_assign(&mut self, rhs: Self) {
        if cfg!(debug_assertions) {
            // exclude the case of result aggregation
            if self.lang != "Total" {
                debug_assert_eq!(self.lang, rhs.lang);
            }
        }
        self.total += rhs.total;
        self.code += rhs.code;
        self.comments += rhs.comments;
        self.blank += rhs.blank;
    }
}

impl CountResult {
    #[inline]
    pub fn new(lang: &'static str) -> Self {
        CountResult {
            lang,
            total: 0,
            code: 0,
            comments: 0,
            blank: 0,
        }
    }
}

/// TODO: Documentation
#[derive(Debug)]
struct Coordinator<'coord> {
    config: &'coord Config,
    tx: chan::Sender<PathBuf>,
    rx: chan::Receiver<LOCCount<'coord>>,
}

impl<'coord> Coordinator<'coord> {
    /// Entry point for the Coordinator thread.
    #[inline]
    fn run(self) -> io::Result<LOCCount<'coord>> {
        self.walk_paths()?;
        self.aggregate_results()
    }

    /// Drop the sending end of the path channel and loop through workers threads' results,
    /// aggregating them in a `rlocc::LOCCount`.
    #[inline]
    fn aggregate_results(self) -> io::Result<LOCCount<'coord>> {
        // Drop the sending-end of the channel to signal workers that
        // they will not be receiving any more paths to process.
        drop(self.tx);

        // Now loop over the receiving-end of the results channel, aggregating all of them into the
        // final LOCCount object that is going to be returned.
        let mut ret: LOCCount<'coord> = LOCCount(HashMap::new());
        rlocc_dbg_log!("[Coordinator][aggregate_results] Blocking on res_rx...");
        while let Ok(res) = self.rx.recv() {
            rlocc_dbg_log!(
                "[Coordinator][aggregate_results] Received '{:?}'. Blocking on res_rx again...",
                res
            );
            ret += res;
        }
        rlocc_dbg_log!("[Coordinator][aggregate_results] res_rs looks disconnected and empty!");
        Ok(ret)
    }

    /// Walk the filesystem paths given and feed the worker threads with all related subdirectories
    /// and files.
    #[inline]
    fn walk_paths(&self) -> io::Result<()> {
        for path in self.config.paths.iter() {
            if path.is_file() {
                rlocc_dbg_log!("[Coordinator][walk_paths] Sending {:?}...", path);
                self.tx.send(path.to_owned()).unwrap(); // FIXME error handling?
            } else if path.is_dir() && !languages::is_vcs(&path) {
                rlocc_dbg_log!("[Coordinator][walk_paths] Diving into {:?}...", path);
                self.__walk(path)?;
            } else {
                rlocc_dbg_log!(
                    "[Coordinator][walk_paths] Skipping non-regular file {:?}.",
                    path
                );
            }
        }
        Ok(())
    }

    /// Auxiliary method used by `self::Coordinator::walk_paths()` to implement recursive
    /// filesystem walk.
    fn __walk(&self, path: &PathBuf) -> io::Result<()> {
        for direntry in fs::read_dir(path)? {
            let direntry = direntry?.path();
            if direntry.is_file() {
                rlocc_dbg_log!("[Coordinator][__walk] Sending {:?}...", direntry);
                self.tx.send(direntry).unwrap();
            } else if direntry.is_dir() && !languages::is_vcs(&direntry) {
                rlocc_dbg_log!("[Coordinator][__walk] Diving into {:?}...", direntry);
                self.__walk(&direntry)?;
            } else {
                rlocc_dbg_log!(
                    "[Coordinator][__walk] Skipping non-regular file {:?}.",
                    direntry
                );
            }
        }
        Ok(())
    }
}

/// TODO: Documentation
#[derive(Debug)]
pub struct ParsingState<'line> {
    pub curr_line: Option<&'line str>,
    pub curr_line_counted: bool,
    pub curr_lang: &'line Language,
}

impl<'line> ParsingState<'line> {
    /// TODO: Documentation
    #[inline]
    pub fn new(lang: &'line Language) -> Self {
        ParsingState {
            curr_line: None,
            curr_line_counted: false,
            curr_lang: lang,
        }
    }
}

/// TODO: Documentation
#[derive(Debug)]
struct Worker<'w> {
    id: usize,
    tx: chan::Sender<LOCCount<'w>>,
    rx: chan::Receiver<PathBuf>,
    sm: LOCStateMachine,
    buffer: String,
}

impl<'w, 'line, 'worker: 'line> Worker<'w> {
    /// Entry point for each Worker thread.
    fn run(mut self) -> io::Result<()> {
        rlocc_dbg_log!("[Worker-{}][run] Blocking on paths_rx...", self.id);
        let mut ret = LOCCount(HashMap::new());
        while let Ok(path) = self.rx.recv() {
            rlocc_dbg_log!(
                "[Worker-{}][run] Received {:?} from paths_rx!",
                self.id,
                path
            );

            match self.process_file(&path) {
                Ok(res) => {
                    rlocc_dbg_log!(
                        "[Worker-{}][run] Calculation for file {:?} has been completed!",
                        self.id,
                        path
                    );
                    ret += res;
                    rlocc_dbg_log!(
                        "[Worker-{}][run] Sent! Now blocking on paths_rx again...",
                        self.id
                    );
                }
                Err(_err) => {
                    rlocc_dbg_log!(
                        "[Worker-{}][run] Error while processing file {:?}: {:#?}",
                        self.id,
                        path,
                        _err
                    );
                }
            };
        }
        rlocc_dbg_log!(
            "[Worker-{}][run] paths_rx looks disconnected and empty!",
            self.id
        );
        // At this point, the paths' channel must have been disconnected by the Coordinator and
        // emptied by the Worker(s).

        rlocc_dbg_log!(
            "[Worker-{}][run] Sending '{:?}' down on res_rx...",
            self.id,
            ret
        );
        self.tx.send(ret).unwrap(); // FIXME error handling?
        rlocc_dbg_log!("[Worker-{}][run] Sent! My job is done...", self.id);

        // Release the sending-end of the channel to signal Coordinator
        // that he will not be receiving any more results from me.
        drop(self.tx);

        Ok(())
    }

    /// TODO: Documentation
    fn process_file(&mut self, path: &PathBuf) -> io::Result<CountResult> {
        let (_, lang) = languages::guess_language(path)?; // FIXME non ext-based guess
        let mut ret = CountResult::new(lang.name);
        self.sm.reset();

        let mut file_rd = BufReader::with_capacity(BUF_SIZE, File::open(path)?);
        loop {
            let mut ps = ParsingState::new(lang);
            self.buffer.clear();
            match file_rd.read_line(&mut self.buffer) {
                Ok(0) => {
                    rlocc_dbg_log!(
                        "[worker-{}][process_file] Reached EOF in file {:?}",
                        self.id,
                        path
                    );
                    break;
                }
                Ok(_) => {
                    self.process_line(&mut ps, &mut ret)?;
                    // TODO somehow update self.state ?

                    // FIXME Do I actually need to explicitly drop ps here to keep stack memory
                    //       from growing crazy until the loop ends? Or does drop only makes sense
                    //       for memory allocated in the heap? Is the shadowing in the beginning of
                    //       the loop enough to automatically free the memory for ps in the stack?
                    //       According to docs.rs, drop() does effectively nothing for types that
                    //       implement the Copy trait; but might leaving ps non-Copy really be the
                    //       solution to keep stack allocation low?
                    // UPDATE According to `std::mem::needs_drop::<ParsingState>()`, dropping ps
                    //        certainly has no side effect. So, the memory allocated in the stack
                    //        must probably be redeemed at the end of each loop iteration (probably
                    //        reused for the new ps in the next iteration). This can't just be an
                    //        unavoidable memory leak.
                    // TODO Benchmark memory usage to verify it.
                    //drop(ps);
                }
                Err(err) => {
                    rlocc_dbg_log!(
                        "[worker-{}][process_file] Error reading lines in file {:?}: {}",
                        self.id,
                        path,
                        err
                    );
                    return Err(err);
                }
            }
        }

        Ok(ret)
    }

    /// TODO: Documentation
    #[inline]
    fn process_line(
        &'worker mut self,
        ps: &mut ParsingState<'line>,
        cr: &mut CountResult,
    ) -> io::Result<()> {
        ps.curr_line = Some(&self.buffer.trim_start());
        self.sm.process(ps, cr);
        cr.total += 1;
        debug_assert_eq!(cr.total, cr.code + cr.comments + cr.blank);
        Ok(())
    }
}

/// TODO: Documentation
pub fn count_all(config: &Config) -> io::Result<LOCCount> {
    let mut ret: Option<io::Result<LOCCount>> = None;

    thread::scope(|s| {
        let (paths_tx, paths_rx) = chan::unbounded();
        let (res_tx, res_rx) = chan::unbounded();

        for id in 0..config.num_threads {
            let tx = res_tx.clone();
            let rx = paths_rx.clone();
            s.spawn(move |_| {
                let worker = Worker {
                    id,
                    tx,
                    rx,
                    sm: LOCStateMachine::new(),
                    buffer: String::with_capacity(BUF_SIZE),
                };

                worker.run()
            }); // TODO really ignore thread handles? Find out how does that work
        }

        drop(paths_rx); // Nobody else receives paths; Coordinator is the sender.
        drop(res_tx); // Nobody else sends results; Coordinator is the receiver.

        let coord = Coordinator {
            config,
            tx: paths_tx,
            rx: res_rx,
        };
        ret = Some(coord.run());
    })
    .unwrap(); // TODO is there a better way to handle this?

    ret.unwrap()
}

impl fmt::Display for LOCCount<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const MAX_OUT_WIDTH: usize = 80;
        const LANG_WIDTH: usize = 25;
        const FILES_WIDTH: usize = 11;
        const LINES_WIDTH: usize = 11;
        const CODE_WIDTH: usize = 11;
        const COMM_WIDTH: usize = 11;
        const BLANK_WIDTH: usize = 11;

        writeln!(f, "{:-^max$}", "", max = MAX_OUT_WIDTH)?;
        writeln!(
            f,
            "{:<law$}{:>fw$}{:>liw$}{:>bw$}{:>cmw$}{:>cdw$}",
            "Language",
            "Files",
            "Lines",
            "Blanks",
            "Comments",
            "Code",
            law = LANG_WIDTH,
            fw = FILES_WIDTH,
            liw = LINES_WIDTH,
            bw = BLANK_WIDTH,
            cmw = COMM_WIDTH,
            cdw = CODE_WIDTH,
        )?;
        writeln!(f, "{:-^max$}", "", max = MAX_OUT_WIDTH)?;
        let mut total_cr = CountResult::new("Total");
        let mut total_files = 0;
        for (lang_name, (cr, fc)) in &self.0 {
            total_cr += *cr;
            total_files += fc;
            writeln!(
                f,
                "{:<law$}{:>fw$}{:>liw$}{:>bw$}{:>cmw$}{:>cdw$}",
                lang_name,
                fc,
                cr.total,
                cr.blank,
                cr.comments,
                cr.code,
                law = LANG_WIDTH,
                fw = FILES_WIDTH,
                liw = LINES_WIDTH,
                bw = BLANK_WIDTH,
                cmw = COMM_WIDTH,
                cdw = CODE_WIDTH,
            )?;
        }
        writeln!(f, "{:-^max$}", "", max = MAX_OUT_WIDTH)?;
        writeln!(
            f,
            "{:<law$}{:>fw$}{:>liw$}{:>bw$}{:>cmw$}{:>cdw$}",
            total_cr.lang,
            total_files,
            total_cr.total,
            total_cr.blank,
            total_cr.comments,
            total_cr.code,
            law = LANG_WIDTH,
            fw = FILES_WIDTH,
            liw = LINES_WIDTH,
            bw = BLANK_WIDTH,
            cmw = COMM_WIDTH,
            cdw = CODE_WIDTH,
        )?;
        write!(f, "{:-^max$}", "", max = MAX_OUT_WIDTH)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile;

    struct Morcker {
        id: usize,
        tx: chan::Sender<CountResult>,
        rx: chan::Receiver<PathBuf>,
    }

    //fn setup_empty_temp_files<P: AsRef<Path>>(dir: P, num: usize) -> io::Result<Vec<String>> {
    //    Err(io::Error::new(io::ErrorKind::Other, "UNIMPLEMENTED"))
    //}

    //fn setup_empty_temp_hier(num: usize) -> io::Result<Vec<String>> {
    //    let tempdir = tempfile::tempdir_in(PathBuf::from("../../tests/test_hier"))?;

    //    Err(io::Error::new(io::ErrorKind::Other, "UNIMPLEMENTED"))
    //}

    #[test]
    fn test_aggr_res() {
        let (paths_tx, paths_rx) = chan::unbounded();
        let (res_tx, res_rx) = chan::unbounded();
        let fake_dirs = vec!["p1".to_owned(), "p2".to_owned(), "p3".to_owned()];
        let me = Coordinator {
            config: &Config::new(fake_dirs.into_iter(), 2).unwrap(),
            tx: paths_tx,
            rx: res_rx,
        };

        thread::scope(|s| {
            for id in 0..me.config.num_threads {
                let morcker = Morcker {
                    id,
                    tx: res_tx.clone(),
                    rx: paths_rx.clone(),
                };
                //s.spawn(|_| morcker.run());
            }
            //for h in handles {
            //    h.join().unwrap();
            //}
        })
        .unwrap();

        drop(paths_rx); // I will not receive paths; I'm the sender.
        drop(res_tx); // I will not send results; I'm the receiver.

        for p in &me.config.paths {
            me.tx.send(p.to_owned()).unwrap();
        }
        me.aggregate_results();

        //me.run()
    }
}
*/
