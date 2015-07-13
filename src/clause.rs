use variable::Variable;

pub type Clause<'a> = (&'a Variable, &'a Variable, &'a Variable);

pub fn clause<'a>(a: &'a Variable, b: &'a Variable, c: &'a Variable) -> Clause<'a> {
	(a, b, c)
}