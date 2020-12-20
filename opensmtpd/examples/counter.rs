use log;
use opensmtpd::{run_filter, Address, Filter, ReportEntry};
use opensmtpd_derive::register;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

pub const DEFAULT_LOG_FILE: &str = "/tmp/counter.log";

#[derive(Default)]
struct MyCounter {
    nb_connected: u64,
    nb_total: u64,
}

impl Filter for MyCounter {
    #[register]
    fn on_report_link_connect(
        &mut self,
        _entry: &ReportEntry,
        _rdns: &str,
        _fcrdns: &str,
        _src: &Address,
        _dest: &Address,
    ) {
        self.nb_connected += 1;
        self.nb_total += 1;
        log::info!(
            "New client (connected: {}, total: {})",
            self.nb_connected,
            self.nb_total
        );
    }

    #[register]
    fn on_report_link_disconnect(&mut self, _entry: &ReportEntry) {
        self.nb_connected -= 1;
        log::info!(
            "Client left (connected: {}, total: {})",
            self.nb_connected,
            self.nb_total
        );
    }
}

fn main() {
    let log_file = std::env::var("LOG_FILE").unwrap_or(String::from(DEFAULT_LOG_FILE));
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create(&log_file).unwrap(),
    )
    .unwrap();
    let mut my_counter: MyCounter = Default::default();
    run_filter(&mut my_counter);
}
