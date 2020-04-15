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

use crate::locc::states::{State, StateInitial};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use crossbeam_channel as chan;
use crossbeam_utils::thread;

use super::{Config, EXT_TO_LANG};

/// TODO: Documentation
pub type LOCCount<'a> = HashMap<&'a str, i32>;

/// TODO: Documentation
#[derive(Debug)]
struct CountResult(&'static str, i32);

/// TODO: Documentation
struct Coordinator<'a> {
    config: &'a Config,
    tx: chan::Sender<PathBuf>,
    rx: chan::Receiver<CountResult>,
}

impl<'a> Coordinator<'a> {
    /// TODO: Documentation
    fn run(self) -> io::Result<LOCCount<'a>> {
        self.walk_paths()?;
        self.aggregate_results()
    }

    /// Drop the sending end of the path channel and loop through workers threads' results,
    /// aggregating them in a `super::LOCCount`.
    fn aggregate_results(self) -> io::Result<LOCCount<'a>> {
        // Drop the sending-end of the channel to signal workers that
        // they will not be receiving any more paths to process.
        drop(self.tx);

        // Now loop over the receiving-end of the results channel, aggregating all of them into the
        // final LOCCount object that is going to be returned.
        let mut ret = HashMap::new();
        eprintln!("[COORDINATOR][aggregate_results] Blocking on res_rx...");
        while let Ok(res) = self.rx.recv() {
            eprintln!(
                "[COORDINATOR][aggregate_results] Received '{:?}'. Blocking on res_rx again...",
                res
            );
            ret.entry(res.0)
                .and_modify(|cnt| *cnt += res.1)
                .or_insert(res.1);
        }
        eprintln!("[COORDINATOR][aggregate_results] res_rs looks disconnected and empty!'");
        Ok(ret)
    }

    /// Walk the filesystem paths given and feed the worker threads with all related subdirectories
    /// and files.
    fn walk_paths(&self) -> io::Result<()> {
        for path in self.config.paths.iter() {
            if path.is_file() {
                eprintln!("[COORDINATOR][walk_paths] Sending {:?}...", path);
                self.tx.send(path.to_owned()).unwrap();
            } else if path.is_dir() {
                eprintln!("[COORDINATOR][walk_paths] Diving into {:?}...", path);
                self.__walk(path)?;
            } else {
                // FIXME(ckatsak): logger or something
                eprintln!(
                    "[COORDINATOR][walk_paths] Skipping non-regular file {:?}.",
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
                eprintln!("[COORDINATOR][__walk] Sending {:?}...", direntry);
                self.tx.send(direntry).unwrap();
            } else if direntry.is_dir() {
                eprintln!("[COORDINATOR][__walk] Diving into {:?}...", direntry);
                self.__walk(&direntry)?;
            } else {
                // FIXME(ckatsak): logger or something
                eprintln!(
                    "[COORDINATOR][__walk] Skipping non-regular file {:?}.",
                    direntry
                );
            }
        }
        Ok(())
    }
}

/// TODO: Implementation
/// TODO: Documentation
struct Worker<'a> {
    id: usize, // FIXME just for devel
    tx: chan::Sender<CountResult>,
    rx: chan::Receiver<PathBuf>,
    state: &'a dyn State,
}

impl<'a> Worker<'a> {
    /// TODO: Documentation
    fn run(self) -> io::Result<()> {
        eprintln!("[WORKER-{}][run] Blocking on paths_rx...", self.id);
        while let Ok(path) = self.rx.recv() {
            eprintln!(
                "[WORKER-{}][run] Received {:?} from paths_rx!",
                self.id, path
            );

            //let res = self.process(&path)?; // error handling
            match self.process(&path) {
                Ok(res) => {
                    eprintln!(
                        "[WORKER-{}][run] Sending '{:?}' down on res_rx...",
                        self.id, res
                    );
                    self.tx.send(res).unwrap(); // FIXME error handling?
                    eprintln!(
                        "[WORKER-{}][run] Sent! Now blocking on paths_rx again...",
                        self.id,
                    );
                }
                Err(err) => {
                    // FIXME proper logging?
                    eprintln!(
                        "[WORKER-{}][run] Error while processing file {:?}: {:#?}",
                        self.id, path, err
                    );
                }
            };
        }
        eprintln!(
            "[WORKER-{}][run] paths_rx looks disconnected and empty!",
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
    fn process(&self, path: &PathBuf) -> io::Result<CountResult> {
        let ext = path.extension().unwrap(); // FIXME error handling
        let lang = EXT_TO_LANG.get(&ext.to_str().unwrap()).unwrap(); // FIXME error handling

        //Err(io::Error::new(io::ErrorKind::Other, "UNIMPLEMENTED"))
        Ok(CountResult(lang.name, 1)) // FIXME: Count files for now
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
            let worker = Worker {
                id,
                tx: res_tx.clone(),
                rx: paths_rx.clone(),
                state: &StateInitial {},
            };
            s.spawn(|_| worker.run()); // TODO really ignore thread handles?
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
            let mut handles = vec![];
            for id in 0..me.config.num_threads {
                let morcker = Morcker {
                    id,
                    tx: res_tx.clone(),
                    rx: paths_rx.clone(),
                };
                handles.push(s.spawn(|_| morcker.run()));
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
