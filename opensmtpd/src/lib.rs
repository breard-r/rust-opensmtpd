mod entry;
mod errors;
mod event_handlers;

use log::{debug, error, warn};
use std::collections::HashMap;
use std::io;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

pub use crate::entry::{Entry, Event};
pub use crate::errors::Error;
pub use crate::event_handlers::{Callback, EventHandler, MatchEvent};
pub use opensmtpd_derive::event;

#[macro_export]
macro_rules! handlers {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(($x)());
            )*
            temp_vec
        }
    };
}

pub enum Response {
    None,
}

#[derive(Clone, Default)]
pub struct NoContext;

struct SessionHandler<T> {
    entry_rx: mpsc::Receiver<Entry>,
    event_handlers: Vec<EventHandler<T>>,
}

impl<T: Clone + Default> SessionHandler<T> {
    fn new(entry_rx: mpsc::Receiver<Entry>, handlers_rx: &mpsc::Receiver<EventHandler<T>>) -> Self {
        debug!(
            "New thread for session {}",
            thread::current().name().unwrap()
        );
        let mut event_handlers = Vec::new();
        for h in handlers_rx.iter() {
            debug!("Event handler registered");
            event_handlers.push(h);
        }
        SessionHandler {
            entry_rx,
            event_handlers,
        }
    }

    fn read_entries(&self) {
        let mut context: T = Default::default();
        for e in self.entry_rx.iter() {
            for h in self.event_handlers.iter() {
                if h.is_callable(&e.event) {
                    h.call(&e, &mut context);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SmtpIn<T> {
    sessions: HashMap<u64, (mpsc::Sender<Entry>, thread::JoinHandle<()>)>,
    event_handlers: Vec<EventHandler<T>>,
}

impl<T: Clone + Default + 'static> SmtpIn<T> {
    /// Read a line from the standard input.
    /// Since EOF should not append, it is considered as an error.
    fn read(&self) -> Result<String, Error> {
        let mut input = String::new();
        let nb = io::stdin().read_line(&mut input)?;
        match nb {
            0 => Err(Error::new("end of file")),
            _ => Ok(input),
        }
    }

    /// Dispatch the entry into its session's thread. If such thread does not
    /// already exists, creates it.
    fn dispatch(&mut self, input: &str) -> Result<(), Error> {
        let entry = Entry::from_str(input)?;
        let id = entry.session_id;
        let disconnect = entry.event == Event::LinkDisconnect;
        let channel = match self.sessions.get(&id) {
            Some((r, _)) => r,
            None => {
                let (handlers_tx, handlers_rx) = mpsc::channel();
                let (entry_tx, entry_rx) = mpsc::channel();
                let name = entry.session_id.to_string();
                let handle = thread::Builder::new().name(name).spawn(move || {
                    SessionHandler::new(entry_rx, &handlers_rx).read_entries();
                })?;
                for h in self.event_handlers.iter() {
                    handlers_tx.send(h.clone())?;
                }
                self.sessions.insert(entry.session_id, (entry_tx, handle));
                let (r, _) = &self.sessions[&entry.session_id];
                r
            }
        };
        channel.send(entry)?;
        if disconnect {
            let _ = self.sessions.remove(&id);
        }
        Ok(())
    }

    /// Allow each child thread to exit gracefully. First, the session table is
    /// drained so all the references to the senders are dropped, which will
    /// cause the receivers threads to exit. Then, we uses the join handlers in
    /// order to wait for the actual exit.
    fn graceful_exit_children(&mut self) {
        let mut handles = Vec::new();
        for (_, (_, h)) in self.sessions.drain() {
            handles.push(h);
        }
        for h in handles {
            let _ = h.join();
        }
    }

    pub fn new() -> Self {
        SmtpIn {
            sessions: HashMap::new(),
            event_handlers: Vec::new(),
        }
    }

    pub fn event_handlers(&mut self, handlers: Vec<EventHandler<T>>) -> &mut Self {
        self.event_handlers = handlers.to_owned();
        self
    }

    /// Run the infinite loop that will read and process input from stdin.
    pub fn run(&mut self) {
        loop {
            let line = match self.read() {
                Ok(l) => l,
                Err(e) => {
                    self.graceful_exit_children();
                    error!("{}", e);
                    std::process::exit(1);
                }
            };
            match self.dispatch(&line) {
                Ok(_) => {}
                Err(e) => warn!("{}", e),
            }
        }
    }
}
