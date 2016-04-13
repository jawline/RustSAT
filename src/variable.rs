use std::string::ToString;

pub struct Variable {
	pub uid: usize,
	pub name: String
}

impl Variable {
	pub fn new(id: usize, name: &str) -> Variable {
		Variable {
			uid: id,
			name: format!("{}", name)
		}
	}
}

impl ToString for Variable {
	fn to_string(&self) -> String {
		format!("{}", self.name)
	}
}