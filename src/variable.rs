use std::string::ToString;

static mut LAST_UID : usize = 0;

pub struct Variable {
	pub uid: usize,
	pub name: String
}

impl Variable {
	pub fn new(name: &str) -> Variable {
		unsafe {
			LAST_UID = LAST_UID + 1;
			Variable {
				uid: LAST_UID,
				name: format!("{}", name)
			}
		}
	}
}

impl ToString for Variable {
	fn to_string(&self) -> String {
		format!("{}", self.name)
	}
}