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

	fn clause_satisfied(&self, clause: &Clause<'a>, variables: &[&Variable], allocation: &Vec<bool>) -> bool {
		let &(ref one, ref two, ref three) = clause;
		one.variable.uid;
		true
	}

	fn is_satisfied(&self, variables: &[&Variable], allocation: &Vec<bool>) -> bool {
		
		for clause in &self.items {
			if !self.clause_satisfied(clause, variables, allocation) {
				return false;
			}
		}

		true
	}

	fn satisfy_from(&self, i: usize, variables: &[&Variable], allocation: &mut Vec<bool>) -> bool {
		
		if i == allocation.len() {
			self.is_satisfied(variables, allocation)
		} else {

			if self.satisfy_from(i + 1, variables, allocation) {
				return true;
			}

			allocation[i] = true;

			if self.satisfy_from(i + 1, variables, allocation) {
				return true;
			}

			allocation[i] = false;
			false
		}
	}

	pub fn can_satisfy(&self, variables: &[&Variable]) -> bool {
		
		let mut current_allocation = Vec::new();

		for var in variables {
			current_allocation.push(false);
		}

		return self.satisfy_from(0, variables, &mut current_allocation);
	}
}