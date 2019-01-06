use env_logger::{Builder, Env};
use log::{debug, info};
use opensmtpd::{event, handlers, Entry, Response, SmtpIn};

#[event(Any)]
fn on_event(entry: &Entry) -> Response {
    debug!("Event received: {:?}", entry);
    Response::None
}

#[event(LinkConnect)]
fn on_connect(entry: &Entry) -> Response {
    info!("New client on session {:x}.", entry.session_id);
    Response::None
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    SmtpIn::new()
        .event_handlers(handlers!(on_event, on_connect))
        .run();
}