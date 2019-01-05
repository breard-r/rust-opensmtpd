use crate::entry::{Entry, Event};

#[derive(Clone, Debug, PartialEq)]
pub enum MatchEvent {
    Evt(Vec<Event>),
    All,
}

#[derive(Clone)]
pub struct EventHandler {
    event: MatchEvent,
    callback: (fn(&Entry) -> bool),
}

impl EventHandler {
    pub fn new(event: MatchEvent, callback: (fn(&Entry) -> bool)) -> Self {
        EventHandler { event, callback }
    }

    pub fn is_callable(&self, event: Event) -> bool {
        match &self.event {
            MatchEvent::All => true,
            MatchEvent::Evt(v) => v.contains(&event),
        }
    }

    pub fn call(&self, entry: &Entry) -> bool {
        (self.callback)(entry)
    }
}
