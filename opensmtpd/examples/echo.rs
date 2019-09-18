use opensmtpd::entry::Entry;
use opensmtpd::{report, simple_filter};

#[derive(Clone, Default)]
struct MyContext {
    nb: usize,
}

#[report(v1, smtp_in, match(all))]
fn echo_handler(entry: &Entry) -> Result<(), String> {
    log::info!("TEST ENTRY: {:?}", entry);
    Ok(())
}

#[report(v1, smtp_in, match(link_disconnect))]
fn test(entry: &Entry) {
    log::info!("HAZ LINK DISCONNECT: {:?}", entry);
    Ok(()) // TODO: REMOVE ME!
}

fn main() {
    simple_filter!(MyContext, [echo_handler, test]);
}
