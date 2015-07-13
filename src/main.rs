pub mod variable;
pub mod clause;
pub mod cnf;

use clause::Item;

fn main() {
	
	let a = variable::Variable::new("a");
	let b = variable::Variable::new("b");
	let c = variable::Variable::new("c");

	let clause_one = clause::clause(Item::new(&a, true), Item::new(&b, false), Item::new(&a, true));
	let clause_two = clause::clause(Item::new(&b, true), Item::new(&c, false), Item::new(&b, true));

	let mut cnf = cnf::CNF::new();
	cnf.add(clause_one);
	cnf.add(clause_two);

	println!("{}", cnf.can_satisfy());
}