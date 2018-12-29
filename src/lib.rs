mod entry;
mod errors;

use crate::entry::Entry;
use crate::errors::Error;
use std::collections::HashMap;
use std::io;
use std::sync::mpsc;
use std::thread;

pub fn dispatch() -> Result<(), Error> {
    let mut sessions = HashMap::new();
    loop {
        let mut input = String::new();
        let nb = io::stdin().read_line(&mut input)?;
        if nb == 0 {
            continue;
        }
        let entry = Entry::from_str(input.as_str())?;
        let channel = match sessions.get(&entry.session_id) {
            Some(c) => c,
            None => {
                let (tx, rx) = mpsc::channel();
                let name = entry.session_id.to_string();
                thread::Builder::new().name(name).spawn(move || {
                    for e in rx.iter() {
                        println!(
                            "Debug: thread {}: {:?}",
                            thread::current().name().unwrap(),
                            e
                        );
                    }
                })?;
                sessions.insert(entry.session_id, tx);
                sessions.get(&entry.session_id).unwrap()
            }
        };
        channel.send(entry)?;
    }
}
