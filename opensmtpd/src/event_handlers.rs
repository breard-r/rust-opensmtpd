use crate::entry::{Entry, Event};
use crate::Response;

#[derive(Clone, Debug, PartialEq)]
pub enum MatchEvent {
    Evt(Vec<Event>),
    All,
}

#[derive(Clone)]
pub struct EventHandler {
    event: MatchEvent,
    callback: (fn(&Entry) -> Response),
}

impl EventHandler {
    fn get_events_from_string(event_str: &String) -> MatchEvent {
        let mut events = Vec::new();
        for name in event_str.split(" , ") {
            match name {
                "Any" | "All" => {
                    return MatchEvent::All;
                }
                _ => match Event::from_str(name) {
                    Ok(e) => {
                        events.push(e);
                    }
                    Err(_) => {}
                },
            }
        }
        MatchEvent::Evt(events)
    }

    pub fn new(event_str: String, callback: (fn(&Entry) -> Response)) -> Self {
        EventHandler {
            event: EventHandler::get_events_from_string(&event_str),
            callback,
        }
    }

    pub fn is_callable(&self, event: Event) -> bool {
        match &self.event {
            MatchEvent::All => true,
            MatchEvent::Evt(v) => v.contains(&event),
        }
    }

    pub fn call(&self, entry: &Entry) {
        match (self.callback)(entry) {
            Response::None => {}
        };
    }
}
