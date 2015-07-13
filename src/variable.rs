use std::string::ToString;

pub struct Variable {
	name: String
}

impl ToString for Variable {
	fn to_string(&self) -> String {
		format!("{}", self.name)
	}
}