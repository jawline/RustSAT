pub mod variable;
pub mod clause;
pub mod cnf;

fn main() {
	let a = variable::Variable::new("a");
	let b = variable::Variable::new("b");
	let c = variable::Variable::new("c");
	let clause_one = clause::clause(&a, &b, &a);
	let clause_two = clause::clause(&b, &c, &a);

	let mut cnf = cnf::CNF::new();
	cnf.add(clause_one);
	cnf.add(clause_two);
}