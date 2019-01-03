use env_logger::{Builder, Env};
use opensmtpd::SmtpIn;

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    SmtpIn::new().run();
}
