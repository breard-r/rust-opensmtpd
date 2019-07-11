// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::entry::Entry;
use crate::event_handlers::EventHandler;
use log::debug;
use std::sync::mpsc;
use std::thread;

pub struct SessionHandler<T> {
    entry_rx: mpsc::Receiver<Entry>,
    event_handlers: Vec<EventHandler<T>>,
}

impl<T: Clone + Default> SessionHandler<T> {
    pub fn new(
        entry_rx: mpsc::Receiver<Entry>,
        handlers_rx: &mpsc::Receiver<EventHandler<T>>,
    ) -> Self {
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

    pub fn read_entries(&self) {
        let mut context: T = Default::default();
        for e in self.entry_rx.iter() {
            for h in self.event_handlers.iter() {
                if h.is_callable(&e.get_event()) {
                    h.call(&e, &mut context);
                }
            }
        }
    }
}
