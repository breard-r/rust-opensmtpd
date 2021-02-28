use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Address {
	Ip(SocketAddr),
	UnixSocket(PathBuf),
}

impl ToString for Address {
	fn to_string(&self) -> String {
		match self {
			Address::Ip(a) => a.to_string(),
			Address::UnixSocket(a) => match a.clone().into_os_string().into_string() {
				Ok(s) => s,
				Err(_) => String::new(),
			},
		}
	}
}
