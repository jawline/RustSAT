use std::vec::Vec;
use clause::Clause;
use variable::Variable;

pub struct CNF<'a> {
	items: Vec<Clause<'a>>
}

impl <'a>CNF<'a> {
	pub fn new() -> CNF<'a> {
		CNF{
			items: Vec::new()
		}
	}

	pub fn add(&mut self, item: Clause<'a>) {
		self.items.push(item)
	}

	pub fn can_satisfy(&self, variables: &[&Variable]) -> bool {
		let mut current_allocation = Vec::<bool>::new();

		for var in variables {
			current_allocation.push(false);
		}

		false
	}
}