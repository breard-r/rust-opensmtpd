// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::entry::{Entry, Event};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum MatchEvent {
    Evt(Vec<Event>),
    All,
}

#[derive(Clone)]
pub enum Callback<T> {
    NoCtx(fn(&Entry)),
    Ctx(fn(&T, &Entry)),
    CtxMut(fn(&mut T, &Entry)),
}

#[derive(Clone)]
pub struct EventHandler<T> {
    pub(crate) event: MatchEvent,
    callback: Callback<T>,
}

fn get_events_from_string(event_str: &str) -> MatchEvent {
    let mut events = Vec::new();
    for name in event_str.split(" , ") {
        match name {
            "Any" | "All" => {
                return MatchEvent::All;
            }
            _ => {
                if let Ok(e) = Event::from_str(name) {
                    events.push(e);
                }
            }
        }
    }
    MatchEvent::Evt(events)
}

impl<T: Clone + Default> EventHandler<T> {
    pub fn new(event_str: &str, callback: Callback<T>) -> Self {
        EventHandler {
            event: get_events_from_string(event_str),
            callback,
        }
    }

    pub fn is_callable(&self, event: &Event) -> bool {
        match &self.event {
            MatchEvent::All => true,
            MatchEvent::Evt(v) => v.contains(&event),
        }
    }

    pub fn call(&self, entry: &Entry, context: &mut T) {
        match self.callback {
            Callback::NoCtx(f) => f(entry),
            Callback::Ctx(f) => f(context, entry),
            Callback::CtxMut(f) => f(context, entry),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_eventhandler_build_noctx() {
        EventHandler::new("Any", Callback::NoCtx::<NoContext>(|_entry: &Entry| {}));
    }

    #[test]
    fn test_eventhandler_build_ctx() {
        EventHandler::new(
            "Any",
            Callback::Ctx(|_context: &NoContext, _entry: &Entry| {}),
        );
    }

    #[test]
    fn test_eventhandler_build_ctxmut() {
        EventHandler::new(
            "Any",
            Callback::CtxMut(|_context: &mut NoContext, _entry: &Entry| {}),
        );
    }
}
