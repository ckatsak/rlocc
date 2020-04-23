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
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::rc::Rc;

use crossbeam_channel as chan;
use crossbeam_utils::thread;

use super::languages::{self, Language};
use super::states::*;
use super::Config;

const BUF_SIZE: usize = 1 << 14;

/// TODO: Documentation
pub type LOCCount<'a> = HashMap<&'a str, i32>;

//#[derive(Debug)]
//struct CountResult {
//    total: i32,
//    code: i32,
//    comments: i32,
//    blank: i32,
//}
/// The result of counting a single file.
type CountResult = (&'static str, i32);

/// TODO: Documentation
struct Coordinator<'a> {
    config: &'a Config,
    tx: chan::Sender<PathBuf>,
    rx: chan::Receiver<CountResult>,
}

impl<'a> Coordinator<'a> {
    /// Entry point for the Coordinator thread.
    #[inline]
    fn run(self) -> io::Result<LOCCount<'a>> {
        self.walk_paths()?;
        self.aggregate_results()
    }

    /// Drop the sending end of the path channel and loop through workers threads' results,
    /// aggregating them in a `rlocc::LOCCount`.
    fn aggregate_results(self) -> io::Result<LOCCount<'a>> {
        // Drop the sending-end of the channel to signal workers that
        // they will not be receiving any more paths to process.
        drop(self.tx);

        // Now loop over the receiving-end of the results channel, aggregating all of them into the
        // final LOCCount object that is going to be returned.
        let mut ret = HashMap::new();
        #[cfg(debug_assertions)]
        eprintln!(
            "[{}:{}][COORDINATOR][aggregate_results] Blocking on res_rx...",
            file!(),
            line!()
        );
        while let Ok(res) = self.rx.recv() {
            #[cfg(debug_assertions)]
            eprintln!(
                "[{}:{}][COORDINATOR][aggregate_results] Received '{:?}'. Blocking on res_rx again...",
                file!(),
                line!(),
                res
            );
            ret.entry(res.0)
                .and_modify(|cnt| *cnt += res.1)
                .or_insert(res.1);
        }
        #[cfg(debug_assertions)]
        eprintln!(
            "[{}:{}][COORDINATOR][aggregate_results] res_rs looks disconnected and empty!",
            file!(),
            line!()
        );
        Ok(ret)
    }

    /// Walk the filesystem paths given and feed the worker threads with all related subdirectories
    /// and files.
    fn walk_paths(&self) -> io::Result<()> {
        for path in self.config.paths.iter() {
            if path.is_file() {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[{}:{}][COORDINATOR][walk_paths] Sending {:?}...",
                    file!(),
                    line!(),
                    path
                );
                self.tx.send(path.to_owned()).unwrap();
            } else if path.is_dir() {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[{}:{}][COORDINATOR][walk_paths] Diving into {:?}...",
                    file!(),
                    line!(),
                    path
                );
                self.__walk(path)?;
            } else {
                // FIXME(ckatsak): logger or something
                #[cfg(debug_assertions)]
                eprintln!(
                    "[{}:{}][COORDINATOR][walk_paths] Skipping non-regular file {:?}.",
                    file!(),
                    line!(),
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
                #[cfg(debug_assertions)]
                eprintln!(
                    "[{}:{}][COORDINATOR][__walk] Sending {:?}...",
                    file!(),
                    line!(),
                    direntry
                );
                self.tx.send(direntry).unwrap();
            } else if direntry.is_dir() {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[{}:{}][COORDINATOR][__walk] Diving into {:?}...",
                    file!(),
                    line!(),
                    direntry
                );
                self.__walk(&direntry)?;
            } else {
                // FIXME(ckatsak): logger or something
                #[cfg(debug_assertions)]
                eprintln!(
                    "[{}:{}][COORDINATOR][__walk] Skipping non-regular file {:?}.",
                    file!(),
                    line!(),
                    direntry
                );
            }
        }
        Ok(())
    }
}

/// TODO: Implementation
/// TODO: Documentation
pub struct ParsingState<'file> {
    pub curr_line: Option<&'file str>,
    pub curr_line_counted: bool,
    pub curr_lang: &'file Language,
}

impl<'file> ParsingState<'file> {
    /// TODO: Implementation
    /// TODO: Documentation
    #[inline]
    pub fn new(lang: &'file Language) -> Self {
        ParsingState {
            curr_line: None,
            curr_line_counted: false,
            curr_lang: lang,
        }
    }
}

/// TODO: Implementation
/// TODO: Documentation
pub struct Worker {
    id: usize, // FIXME just for devel? useful for logging too
    tx: chan::Sender<CountResult>,
    rx: chan::Receiver<PathBuf>,

    //state: Option<&'a mut Box<dyn State>>,
    ////state: Option<&'a mut dyn State>,
    //states: [Box<dyn State>; NUM_STATES],
    ///////////////////////////
    state: Option<Rc<RefCell<dyn State>>>,
    states: [Rc<RefCell<dyn State>>; NUM_STATES],

    buffer: String,
}

impl<'worker, 'file: 'worker> Worker {
    /// Entry point for each Worker thread.
    fn run(mut self) -> io::Result<()> {
        #[cfg(debug_assertions)]
        eprintln!(
            "[{}:{}][WORKER-{}][run] Blocking on paths_rx...",
            file!(),
            line!(),
            self.id
        );
        while let Ok(path) = self.rx.recv() {
            #[cfg(debug_assertions)]
            eprintln!(
                "[{}:{}][WORKER-{}][run] Received {:?} from paths_rx!",
                file!(),
                line!(),
                self.id,
                path
            );

            match self.process_file(&path) {
                Ok(res) => {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "[{}:{}][WORKER-{}][run] Sending '{:?}' down on res_rx...",
                        file!(),
                        line!(),
                        self.id,
                        res
                    );
                    self.tx.send(res).unwrap(); // FIXME error handling?
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "[{}:{}][WORKER-{}][run] Sent! Now blocking on paths_rx again...",
                        file!(),
                        line!(),
                        self.id,
                    );
                }
                Err(err) => {
                    // FIXME proper logging?
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "[{}:{}][WORKER-{}][run] Error while processing file {:?}: {:#?}",
                        file!(),
                        line!(),
                        self.id,
                        path,
                        err
                    );
                }
            };
        }
        #[cfg(debug_assertions)]
        eprintln!(
            "[{}:{}][WORKER-{}][run] paths_rx looks disconnected and empty!",
            file!(),
            line!(),
            self.id,
        );
        // At this point, the paths' channel must have been disconnected by the Coordinator and
        // emptied by the Worker(s).

        // Release the sending-end of the channel to signal Coordinator
        // that he will not be receiving any more results from me.
        drop(self.tx);

        Ok(())
    }

    /// TODO: Implementation
    /// TODO: Documentation
    fn process_file(&mut self, path: &PathBuf) -> io::Result<CountResult> {
        let (_, lang) = languages::guess_language(path)?;

        let mut ret: CountResult = (lang.name, 1); // FIXME Count files for now
        let mut pd = ParsingState::new(lang);
        self.set_state(STATE_INITIAL);

        let mut file_rd = BufReader::with_capacity(BUF_SIZE, File::open(path)?);
        loop {
            self.buffer.clear();
            match file_rd.read_line(&mut self.buffer) {
                Ok(0) => {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "[{}:{}][WORKER-{}][process_file] Reached EOF in file {}",
                        file!(),
                        line!(),
                        self.id,
                        path.display()
                    );
                    // TODO somehow reset self.state ?
                    break;
                }
                Ok(_) => {
                    self.process_line(&mut pd, &mut ret)?
                    // TODO somehow update self.state ?
                }
                Err(err) => {
                    // FIXME proper logging?
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "[{}:{}][WORKER-{}][process_file] Error reading lines in file {}: {}",
                        file!(),
                        line!(),
                        self.id,
                        path.display(),
                        err,
                    );
                    return Err(err);
                }
            }
        }

        //Err(io::Error::new(io::ErrorKind::Other, "UNIMPLEMENTED"))
        //Ok(CountResult {
        //    total: 0,
        //    code: 0,
        //    comments: 0,
        //    blank: 0,
        //})
        Ok(ret)
    }

    /// TODO: Implementation
    /// TODO: Documentation
    fn process_line(&mut self, ps: &mut ParsingState, result: &mut CountResult) -> io::Result<()> {
        ps.curr_line_counted = false;
        ps.curr_line = Some(&self.buffer[..]);
        if let Some(state) = self.state.take() {
            let state = state.borrow_mut();
            while state.process(ps, self) {}
        }

        // TODO

        Err(io::Error::new(io::ErrorKind::Other, "UNIMPLEMENTED"))
    }

    /// TODO Documentation
    #[inline]
    pub fn set_state(&mut self, state_no: usize) {
        //self.state = Some(&mut self.states[state_no]);
        self.state = Some(Rc::clone(&self.states[state_no]));
    }
}

/// TODO: Implementation?
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

                    state: None,
                    states: [
                        Rc::new(RefCell::new(StateInitial {})),
                        Rc::new(RefCell::new(StateMultiLineComment {})),
                        Rc::new(RefCell::new(StateCode {})),
                    ],

                    buffer: String::new(),
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
