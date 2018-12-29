mod entry;
mod errors;

use crate::entry::Entry;
use crate::errors::Error;
use std::collections::HashMap;
use std::io;
use std::sync::mpsc;
use std::thread;

/// Read a line from the standard input.
/// Since EOF should not append, it is considered as an error.
fn read() -> Result<String, Error> {
    let mut input = String::new();
    let nb = io::stdin().read_line(&mut input)?;
    match nb {
        0 => Err(Error::new("end of file")),
        _ => Ok(input),
    }
}

/// Dispatch the entry into its session's thread. If such thread does not
/// already exists, creates it.
fn dispatch(
    sessions: &mut HashMap<u64, (mpsc::Sender<Entry>, thread::JoinHandle<()>)>,
    input: &str,
) -> Result<(), Error> {
    let entry = Entry::from_str(input)?;
    let channel = match sessions.get(&entry.session_id) {
        Some((r, _)) => r,
        None => {
            let (tx, rx) = mpsc::channel();
            let name = entry.session_id.to_string();
            let handle = thread::Builder::new().name(name).spawn(move || {
                println!("New thread: {}", thread::current().name().unwrap());
                for e in rx.iter() {
                    println!(
                        "Debug: thread {}: {:?}",
                        thread::current().name().unwrap(),
                        e
                    );
                }
            })?;
            sessions.insert(entry.session_id, (tx, handle));
            let (r, _) = sessions.get(&entry.session_id).unwrap();
            r
        }
    };
    channel.send(entry)?;
    Ok(())
}

/// Allow each child thread to exit gracefully. First, the session table is
/// drained so all the references to the senders are dropped, which will
/// cause the receivers threads to exit. Then, we uses the join handlers in
/// order to wait for the actual exit.
fn graceful_exit_children(
    sessions: &mut HashMap<u64, (mpsc::Sender<Entry>, thread::JoinHandle<()>)>,
) {
    let mut handles = Vec::new();
    for (_, (_, h)) in sessions.drain() {
        handles.push(h);
    }
    for h in handles {
        let _ = h.join();
    }
}

/// Run the infinite loop that will read and process input from stdin.
pub fn run() {
    let mut sessions = HashMap::new();
    loop {
        let line = match read() {
            Ok(l) => l,
            Err(e) => {
                graceful_exit_children(&mut sessions);
                e.display();
                std::process::exit(1);
            }
        };
        match dispatch(&mut sessions, &line) {
            Ok(_) => {}
            Err(e) => e.display(),
        }
    }
}
