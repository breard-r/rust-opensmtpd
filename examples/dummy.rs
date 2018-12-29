fn main() {
    match opensmtpd::dispatch() {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e.as_str()),
    }
}
