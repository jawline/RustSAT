pub mod variable;
pub mod clause;
pub mod cnf;

use variable::Variable;
use clause::Item;

fn main() {
	
	let a = Variable::new("a");
	let b = Variable::new("b");
	let c = Variable::new("c");

	let clause_one = clause::two(Item::new(&a, true), Item::new(&b, false));
	let clause_two = clause::two(Item::new(&b, true), Item::new(&c, false));

	let mut cnf = cnf::CNF::new();
	cnf.add(clause_one);
	cnf.add(clause_two);

	println!("{}", cnf.can_satisfy());
}