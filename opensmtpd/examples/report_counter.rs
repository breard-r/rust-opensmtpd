use log;
use opensmtpd::entry::Entry;
use opensmtpd::{register_contexts, report, simple_filter};

#[derive(Clone, Default)]
struct MyCounter {
    nb: usize,
}

register_contexts!(MyCounter, MyCounter);

#[report(v1, smtp_in, match(all))]
fn on_report(entry: &Entry, total: &mut MyCounter, session: &mut MyCounter) {
    total.nb += 1;
    session.nb += 1;
    log::info!(
        "Event received for session {}: {} (total: {})",
        entry.get_session_id(),
        session.nb,
        total.nb
    );
}

fn main() {
    simple_filter!(MyCounter, MyCounter, [on_report]);
}
