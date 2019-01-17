use log::{info, Level};
use opensmtpd::{event, handlers, Entry, SmtpIn, SmtpdLogger};

#[derive(Clone, Default)]
struct MyContext {
    nb: usize,
}

#[event(Any)]
fn on_event(ctx: &mut MyContext, entry: &Entry) {
    ctx.nb += 1;
    info!("Event received: {}, {}", entry.session_id, ctx.nb);
}

fn main() {
    let _ = SmtpdLogger::new().set_level(Level::Debug).init();
    SmtpIn::new().event_handlers(handlers!(on_event)).run();
}
