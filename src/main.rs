pub mod variable;
pub mod clause;
pub mod cnf;

use cnf::CNF;
use variable::Variable;
use clause::item;

fn main() {
	
	let mut a = Variable::new("a");
	let mut b = Variable::new("b");
	let mut c = Variable::new("c");

	let mut cnf = CNF::new();
	
	let mut initial = cnf.initial_allocation(&mut [&mut a, &mut b, &mut c]);

	let clause_one = clause::two(item(&a, true), item(&b, false));
	let clause_two = clause::two(item(&b, false), item(&c, false));

	cnf.add(clause_one);
	cnf.add(clause_two);

	println!("{}", if cnf.can_satisfy(&[&a, &b, &c], &mut initial) { "satisfiable" } else { "not satisfiable" });
}
