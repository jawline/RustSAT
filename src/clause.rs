use variable::Variable;

pub struct Item<'a> {
	pub variable: &'a Variable,
	pub not: bool
}

impl <'a>Item<'a> {
	pub fn new(variable: &'a Variable, not: bool) -> Item<'a> {
		Item {
			variable: variable,
			not: not
		}
	}

	pub fn clone(&self) -> Item<'a> {
		Item {
			variable: self.variable,
			not: self.not
		}
	}
}

pub type Clause<'a> = (Item<'a>, Item<'a>, Item<'a>);

pub fn item<'a>(variable: &'a Variable, not: bool) -> Item<'a> {
	Item::new(variable, not)
}

pub fn one<'a>(a: Item<'a>) -> Clause<'a> {
	(a.clone(), a.clone(), a)
}

pub fn two<'a>(a: Item<'a>, b: Item<'a>) -> Clause<'a> {
	(a.clone(), b, a)
}

pub fn three<'a>(a: Item<'a>, b: Item<'a>, c: Item<'a>) -> Clause<'a> {
	(a, b, c)
}