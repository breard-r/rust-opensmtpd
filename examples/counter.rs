use log;
use opensmtpd::{register, run_filter, Address, Filter, ReportEntry};
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

#[derive(Default)]
struct MyCounter {
    nb_connected: u64,
    nb_total: u64,
}

impl Filter for MyCounter {
    register!(has_report_link_connect);
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

    register!(has_report_link_disconnect);
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
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Stderr).unwrap();
    let mut my_counter: MyCounter = Default::default();
    run_filter(&mut my_counter);
}