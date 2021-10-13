#[derive(Debug, Default, Clone, Copy)]
struct Counter(u8);

impl Iterator for Counter {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		if self.0 >= 11 {
			None
		} else {
			self.0 += 1;
			Some(self.0)
		}
	}
}

fn main() {
	dbg!(iter::<20>());
}

fn iter<const N: usize>() -> Vec<u8> {
	let mut output = Vec::new();
	let counter = Counter::default();
	for i in 0..=(N / 5) {
		output.extend(counter.skip(i * 5).take(5));
	}

	output
}
