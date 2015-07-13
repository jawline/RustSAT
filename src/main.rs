pub mod variable;
pub mod clause;

fn main() {
	let a = variable::Variable::new("a");
	let clause = clause::one_clause(&a);
}