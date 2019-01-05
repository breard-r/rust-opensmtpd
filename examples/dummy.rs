use log::debug;
use env_logger::{Builder, Env};
use opensmtpd::{handlers, Entry, EventHandler, MatchEvent, SmtpIn};

fn on_event(entry: &Entry) -> bool {
    debug!("Event received: {:?}", entry);
    true
}

// This function should be replaced by a procedural macro on
// the `on_event` function.
fn on_event_builder() -> EventHandler {
    EventHandler::new(MatchEvent::All, on_event)
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    SmtpIn::new()
        .event_handlers(handlers!(on_event_builder))
        .run();
}
