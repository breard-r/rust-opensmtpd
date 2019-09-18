use opensmtpd::entry::Entry;
use opensmtpd::{register_no_context, report, simple_filter};

register_no_context!();

#[report(v1, smtp_in, match(link_connect))]
fn hello(entry: &Entry) {
    log::info!("Hello {}!", entry.get_session_id());
}

fn main() {
    simple_filter!([hello]);
}
