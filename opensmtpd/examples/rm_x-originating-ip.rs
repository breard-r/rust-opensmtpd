use opensmtpd::{return_data_line, run_filter, Filter, FilterEntry};
use opensmtpd_derive::register;

pub const HEADER_NAME: &str = "x-originating-ip:";
pub const HEADER_LEN: usize = 17;

struct RmXOriginatingIp {}

impl Filter for RmXOriginatingIp {
    #[register]
    fn on_filter_data_line(&mut self, entry: &FilterEntry, data_line: &[u8]) {
        if data_line.len() >= HEADER_LEN {
            let head_start = data_line[..HEADER_LEN].to_vec();
            if let Ok(s) = String::from_utf8(head_start) {
                if s.to_lowercase() == HEADER_NAME {
                    return;
                }
            }
        }
        return_data_line(entry, data_line);
    }
}

fn main() {
    let mut my_filter = RmXOriginatingIp {};
    run_filter(&mut my_filter);
}
