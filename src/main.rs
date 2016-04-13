pub mod variable;
pub mod clause;
pub mod cnf;

use cnf::CNF;
use variable::Variable;
use clause::item;

fn main() {
	
	let a = Variable::new(0, "a");
	let b = Variable::new(1, "b");
	let c = Variable::new(2, "c");

	let clause_one = clause::two(item(&a, true), item(&b, false));
	let clause_two = clause::two(item(&b, false), item(&c, false));

	let mut cnf = CNF::new();

	cnf.add(clause_one);
	cnf.add(clause_two);

	println!("{}", if cnf.can_satisfy(&[&a, &b, &c]) { "satisfiable" } else { "not satisfiable" });
}
