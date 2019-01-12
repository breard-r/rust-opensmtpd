use env_logger::{Builder, Env};
use log::info;
use opensmtpd::{event, handlers, Entry, SmtpIn};

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
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    SmtpIn::new().event_handlers(handlers!(on_event)).run();
}
