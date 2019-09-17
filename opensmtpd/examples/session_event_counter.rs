use opensmtpd::{report, simple_filter};

#[derive(Clone, Default)]
struct MyContext {
    nb: usize,
}

#[report(Any)]
fn on_report(ctx: &mut MyContext, entry: &Entry) {
    ctx.nb += 1;
    info!("Event received: {}, {}", entry.get_session_id(), ctx.nb);
}

fn main() {
    simple_filter!(vec![on_report]);
}
