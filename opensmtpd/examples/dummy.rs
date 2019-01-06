use env_logger::{Builder, Env};
use log::{debug, info};
use opensmtpd::{event, handlers, Entry, SmtpIn};

#[event(Any)]
fn on_event(entry: &Entry) -> bool {
    debug!("Event received: {:?}", entry);
    true
}

#[event(LinkConnect)]
fn on_connect(entry: &Entry) -> bool {
    info!("New client on session {:x}.", entry.session_id);
    true
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    SmtpIn::new()
        .event_handlers(handlers!(on_event, on_connect))
        .run();
}
