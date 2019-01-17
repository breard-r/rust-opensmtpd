use log::{debug, info, Level};
use opensmtpd::{event, handlers, Entry, SmtpIn, SmtpdLogger};

#[event(Any)]
fn on_event(entry: &Entry) {
    debug!("Event received: {:?}", entry);
}

#[event(LinkConnect)]
fn on_connect(entry: &Entry) {
    info!("New client on session {:x}.", entry.session_id);
}

fn main() {
    let _ = SmtpdLogger::new().set_level(Level::Debug).init();
    SmtpIn::new()
        .event_handlers(handlers!(on_event, on_connect))
        .run();
}
