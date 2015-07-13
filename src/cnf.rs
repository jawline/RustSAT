use std::vec::Vec;
use clause::Clause;

pub struct CNF<'a> {
	items: Vec<Clause<'a>>
}

impl <'a>CNF<'a> {
	pub fn new() -> CNF<'a> {
		CNF{items: Vec::new()}
	}

	pub fn add(&mut self, item: Clause<'a>) {
		self.items.push(item)
	}

	pub fn can_satisfy(&mut self) -> bool {
		true
	}
}