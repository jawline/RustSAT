use std::vec::Vec;
use clause::Clause;
use variable::Variable;

pub struct CNF<'a> {
	items: Vec<Clause<'a>>
}

impl <'a>CNF<'a> {
	pub fn new() -> CNF<'a> {
		CNF {
			items: Vec::new()
		}
	}

	pub fn add(&mut self, item: Clause<'a>) {
		self.items.push(item)
	}

	pub fn is_satisfied(&self, variables: &[&Variable], allocation: &Vec<bool>) -> bool {
		true
	}

	pub fn satisfy_from(&self, i: usize, variables: &[&Variable], allocation: &mut Vec<bool>) -> bool {
		
		for j in (i+1)..allocation.len() {
			allocation[j] = true;
			
			if self.is_satisfied(variables, allocation) {
				return true;
			}

			allocation[j] = false;
		}

		false
	}

	pub fn can_satisfy(&self, variables: &[&Variable]) -> bool {
		
		let mut current_allocation = Vec::new();

		for var in variables {
			current_allocation.push(false);
		}

		if self.is_satisfied(variables, &current_allocation) {
			return true;
		}

		for i in 0..current_allocation.len() {
			current_allocation[i] = true;

			if self.is_satisfied(variables, &current_allocation) {
				return true;
			}

			if self.satisfy_from(i, variables, &mut current_allocation) {
				return true;
			}
		}

		false
	}
}