use env_logger::{Builder, Env};
use log::{debug, info};
use opensmtpd::{event, handlers, Entry, NoContext, SmtpIn};

#[event(Any)]
fn on_event(_context: &mut NoContext, entry: &Entry) {
    debug!("Event received: {:?}", entry);
}

#[event(LinkConnect)]
fn on_connect(_context: &mut NoContext, entry: &Entry) {
    info!("New client on session {:x}.", entry.session_id);
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    SmtpIn::new()
        .event_handlers(handlers!(on_event, on_connect))
        .run();
}
