pub mod variable;
pub mod clause;
pub mod cnf;

use cnf::CNF;
use variable::Variable;
use clause::item;

fn main() {
	
	let a = Variable::new("a");
	let b = Variable::new("b");
	let c = Variable::new("c");

	let clause_one = clause::two(item(&a, true), item(&b, false));
	let clause_two = clause::two(item(&b, true), item(&c, false));

	let mut cnf = CNF::new();

	cnf.add(clause_one);
	cnf.add(clause_two);

	println!("{}", if cnf.can_satisfy(&[&a, &b, &b]) { "satisfiable" } else { "not satisfiable" });
}
