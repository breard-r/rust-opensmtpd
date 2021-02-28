#[derive(Clone, Debug)]
pub struct SmtpStatusCode {
	pub number: usize,
	pub text: String,
}

impl SmtpStatusCode {
	pub fn from_number(error_number: usize) -> Self {
		match error_number {
			211 => SmtpStatusCode {
				number: 211,
				text: String::from("System status"),
			},
			220 => SmtpStatusCode {
				number: 220,
				text: String::from("Service ready"),
			},
			250 => SmtpStatusCode {
				number: 250,
				text: String::from("Requested mail action okay, completed"),
			},
			251 => SmtpStatusCode {
				number: 251,
				text: String::from("User not local; will forward"),
			},
			252 => SmtpStatusCode {
				number: 252,
				text: String::from(
					"Cannot verify the user, but it will try to deliver the message anyway",
				),
			},
			354 => SmtpStatusCode {
				number: 354,
				text: String::from("Start mail input"),
			},
			421 => SmtpStatusCode {
				number: 421,
				text: String::from("Service is unavailable because the server is shutting down"),
			},
			450 => SmtpStatusCode {
				number: 450,
				text: String::from("Requested mail action not taken: mailbox unavailable"),
			},
			451 => SmtpStatusCode {
				number: 451,
				text: String::from("Requested action aborted: local error in processing"),
			},
			452 => SmtpStatusCode {
				number: 452,
				text: String::from("Requested action not taken: insufficient system storage"),
			},
			455 => SmtpStatusCode {
				number: 455,
				text: String::from("Server unable to accommodate parameters"),
			},
			500 => SmtpStatusCode {
				number: 500,
				text: String::from("Syntax error, command unrecognized"),
			},
			501 => SmtpStatusCode {
				number: 501,
				text: String::from("Syntax error in parameters or arguments"),
			},
			502 => SmtpStatusCode {
				number: 502,
				text: String::from("Command not implemented"),
			},
			503 => SmtpStatusCode {
				number: 503,
				text: String::from("Bad sequence of commands"),
			},
			504 => SmtpStatusCode {
				number: 504,
				text: String::from("Command parameter is not implemented"),
			},
			521 => SmtpStatusCode {
				number: 521,
				text: String::from("Server does not accept mail"),
			},
			523 => SmtpStatusCode {
				number: 523,
				text: String::from("Encryption Needed"),
			},
			550 => SmtpStatusCode {
				number: 550,
				text: String::from("Requested action not taken: mailbox unavailable"),
			},
			552 => SmtpStatusCode {
				number: 552,
				text: String::from("Requested mail action aborted: exceeded storage allocation"),
			},
			553 => SmtpStatusCode {
				number: 553,
				text: String::from("Requested action not taken: mailbox name not allowed"),
			},
			554 => SmtpStatusCode {
				number: 554,
				text: String::from("Transaction has failed"),
			},
			556 => SmtpStatusCode {
				number: 556,
				text: String::from("Domain does not accept mail"),
			},
			nb => SmtpStatusCode {
				number: nb,
				text: String::new(),
			},
		}
	}
}

impl ToString for SmtpStatusCode {
	fn to_string(&self) -> String {
		format!("{} {}", self.number, self.text)
	}
}
