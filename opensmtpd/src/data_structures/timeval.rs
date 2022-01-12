#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeVal {
	pub sec: i64,
	pub usec: i64,
}

impl ToString for TimeVal {
	fn to_string(&self) -> String {
		format!("{}.{}", self.sec, self.usec)
	}
}
