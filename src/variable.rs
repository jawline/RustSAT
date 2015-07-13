use std::string::ToString;

pub struct Variable {
	name: String
}

impl Variable {
	pub fn new(name: &str) -> Variable {
		Variable{name: format!("{}", name)}
	}
}

impl ToString for Variable {
	fn to_string(&self) -> String {
		format!("{}", self.name)
	}
}