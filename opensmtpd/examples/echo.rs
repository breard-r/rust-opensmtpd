use opensmtpd::entry::Entry;
use opensmtpd::{register_no_context, report, simple_filter};

register_no_context!();

#[report(v1, smtp_in, match(all))]
fn echo(entry: &Entry) {
    log::info!("New entry: {:?}", entry);
}

fn main() {
    simple_filter!([echo]);
}
