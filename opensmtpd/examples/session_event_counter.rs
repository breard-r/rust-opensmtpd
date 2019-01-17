use log::{info, Level};
use opensmtpd::{handlers, report, Entry, SmtpIn, SmtpdLogger};

#[derive(Clone, Default)]
struct MyContext {
    nb: usize,
}

#[report(Any)]
fn on_report(ctx: &mut MyContext, entry: &Entry) {
    ctx.nb += 1;
    info!("Event received: {}, {}", entry.session_id, ctx.nb);
}

fn main() {
    let _ = SmtpdLogger::new().set_level(Level::Debug).init();
    SmtpIn::new().event_handlers(handlers!(on_report)).run();
}
