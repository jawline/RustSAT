use variable::Variable;

pub struct Item<'a> {
	variable: &'a Variable,
	not: bool
}

impl <'a>Item<'a> {
	pub fn new(variable: &'a Variable, not: bool) -> Item<'a> {
		Item{variable: variable, not: not}
	}
}

pub type Clause<'a> = (Item<'a>, Item<'a>, Item<'a>);

pub fn clause<'a>(a: Item<'a>, b: Item<'a>, c: Item<'a>) -> Clause<'a> {
	(a, b, c)
}