use variable::Variable;

pub type Clause<'a> = (&'a Variable, &'a Variable, &'a Variable);

pub fn one_clause<'a>(item: &'a Variable) -> Clause {
	(item, item, item)
}