use env_logger::{Builder, Env};

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    opensmtpd::run();
}
