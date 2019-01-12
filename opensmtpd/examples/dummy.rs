use env_logger::{Builder, Env};
use log::{debug, info};
use opensmtpd::{event, handlers, Entry, SmtpIn};

#[event(Any)]
fn on_event(entry: &Entry) {
    debug!("Event received: {:?}", entry);
}

#[event(LinkConnect)]
fn on_connect(entry: &Entry) {
    info!("New client on session {:x}.", entry.session_id);
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    SmtpIn::new()
        .event_handlers(handlers!(on_event, on_connect))
        .run();
}
