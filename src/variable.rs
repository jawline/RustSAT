use std::string::ToString;

pub struct Variable {
	pub tid: usize,
	pub name: String
}

impl Variable {
	pub fn new(name: &str) -> Variable {
		Variable {
			tid: 0,
			name: format!("{}", name)
		}
	}
}

impl ToString for Variable {
	fn to_string(&self) -> String {
		format!("{}", self.name)
	}
}