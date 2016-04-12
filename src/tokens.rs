pub struct Tokens {
	pub src: String
}

impl Tokens {
	pub fn new(src: &str) {
		Tokens{
			src: src.to_string()
		}
	}
}