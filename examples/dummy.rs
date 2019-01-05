use log::debug;
use env_logger::{Builder, Env};
use opensmtpd::{Entry, EventHandler, MatchEvent, SmtpIn};

fn on_event(entry: &Entry) -> bool {
    debug!("Event received: {:?}", entry);
    true
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    let h = vec![EventHandler::new(MatchEvent::All, on_event)];
    SmtpIn::new().event_handlers(h).run();
}
