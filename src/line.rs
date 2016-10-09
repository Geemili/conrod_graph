
use conrod::color;

pub struct Line<X> {
	pub color: color::Color,
	pub function: Box<Fn(X) -> X>,
}

impl<X> Line<X> {

	pub fn from_fn(f: Box<Fn(X) -> X>) -> Self {
		Line {
			color: color::BLUE,
			function: f,
		}
	}

	pub fn set_color(mut self, color: color::Color) -> Self {
		self.color = color;
		self
	}

}
