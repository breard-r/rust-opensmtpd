use log;
use opensmtpd::entry::Entry;
use opensmtpd::{report, simple_filter};

#[derive(Clone, Default)]
struct MyContext {
    nb: usize,
}

#[report(v1, smtp_in, match(all))]
fn on_report(ctx: &mut MyContext, entry: &Entry) {
    ctx.nb += 1;
    log::info!("Event received: {}, {}", entry.get_session_id(), ctx.nb);
    Ok(())
}

fn main() {
    simple_filter!(MyContext, [on_report]);
}
