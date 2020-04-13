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
use std::fs;
use std::io;
use std::path::PathBuf;

use crossbeam_channel as chan;
use crossbeam_utils::thread;

use super::Config;

/// TODO: Documentation
pub type LOCCount<'a> = HashMap<&'a str, i32>;
/// TODO: Documentation
type CountResult = (&'static str, i32);

/// TODO: Documentation
struct Coordinator<'a> {
    config: &'a Config,
    tx: chan::Sender<PathBuf>,
    rx: chan::Receiver<CountResult>,
}

/// TODO: Implementation
/// TODO: Documentation
impl<'a> Coordinator<'a> {
    /// TODO: Documentation
    fn run(self) -> io::Result<LOCCount<'a>> {
        self.walk_paths()?;
        self.aggregate_results()
        //Err(io::Error::new(io::ErrorKind::Other, "UNIMPLEMENTED"))
    }

    /// Drop the sending end of the path channel and loop through workers threads' results,
    /// aggregating them in a `super::LOCCount`.
    fn aggregate_results(self) -> io::Result<LOCCount<'a>> {
        // Drop the sending-end of the channel to signal workers that
        // they will not be receiving any more paths to process.
        drop(self.tx);

        let mut ret = HashMap::new();
        while let Ok(res) = self.rx.recv() {
            eprintln!("COORDINATOR: Received '{:#?}'", res); // FIXME lose it
            ret.entry(res.0)
                .and_modify(|cnt| *cnt += res.1)
                .or_insert(res.1);
        }
        Ok(ret)
    }

    /// Walk the given filesystem paths to feed worker threads with all their subdirectories and
    /// their files.
    fn walk_paths(&self) -> io::Result<()> {
        for path in self.config.paths.iter() {
            if path.is_file() {
                self.tx.send(path.to_owned()).unwrap();
            } else if path.is_dir() {
                self.__walk(path)?;
            } else {
                // FIXME(ckatsak): logger or something
                eprintln!("Skipping non-regular file '{}'.", path.display());
            }
        }
        Ok(())
    }

    /// Auxiliary method used by `self::Coordinator::walk_paths()` to implement recursive walk.
    fn __walk(&self, path: &PathBuf) -> io::Result<()> {
        for direntry in fs::read_dir(path)? {
            let direntry = direntry?.path();
            if direntry.is_file() {
                self.tx.send(direntry).unwrap();
            } else if direntry.is_dir() {
                self.__walk(&direntry)?;
            } else {
                // FIXME(ckatsak): logger or something
                eprintln!("Skipping non-regular file '{}'.", direntry.display());
            }
        }
        Ok(())
    }
}

/// TODO: Implementation
/// TODO: Documentation
struct Worker {
    tx: chan::Sender<CountResult>,
    rx: chan::Receiver<PathBuf>,
}

/// TODO: Implementation
/// TODO: Documentation
impl Worker {
    /// TODO: Implementation
    /// TODO: Documentation
    pub fn run(self) {
        //
        // TODO
        //

        // Release the sending-end of the channel to signal Coordinator
        // that he will not be receiving any more results from me.
        drop(self.tx);
    }
}

/// TODO: Implementation
/// TODO: Documentation
pub fn count_all(config: &Config) -> io::Result<LOCCount> {
    let (paths_tx, paths_rx) = chan::unbounded();
    let (res_tx, res_rx) = chan::unbounded();

    let me = Coordinator {
        config,
        tx: paths_tx,
        rx: res_rx,
    };

    thread::scope(|s| {
        let mut handles = vec![];
        for _ in 0..config.num_threads {
            let worker = Worker {
                tx: res_tx.clone(),
                rx: paths_rx.clone(),
            };
            handles.push(s.spawn(|_| worker.run()));
        }
        //for h in handles {
        //    h.join().unwrap();
        //}
    })
    .unwrap();

    drop(paths_rx); // I will not receive paths; I'm the sender.
    drop(res_tx); // I will not send results; I'm the receiver.
    me.run()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    //#[test]
    //TODO
}
