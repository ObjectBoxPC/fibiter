use basic_lucas_seq::BasicLucasSeqIterator;

#[derive(Debug, Clone)]
pub struct FibonacciIterator {
	basic_lucas: BasicLucasSeqIterator,
}

impl FibonacciIterator {
	pub fn new() -> FibonacciIterator {
		FibonacciIterator { basic_lucas: BasicLucasSeqIterator::new(1, 1) }
	}
}

impl Iterator for FibonacciIterator {
	type Item = i64;

	fn next(&mut self) -> Option<i64> {
		self.basic_lucas.next()
	}
}

#[cfg(test)]
mod tests {
#[test]
	fn gives_correct_values() {
		let f = super::FibonacciIterator::new();
		let expected = [1, 1, 2, 3, 5, 8, 13, 21];
		for (e, a) in expected.iter().zip(f) {
			assert_eq!(*e, a);
		}
	}
}