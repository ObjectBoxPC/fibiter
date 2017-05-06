use basic_lucas_seq::BasicLucasSeqIterator;

/// Iterator that gives Lucas numbers
#[derive(Debug, Clone)]
pub struct LucasIterator {
	basic_lucas: BasicLucasSeqIterator,
}

impl LucasIterator {
	pub fn new() -> LucasIterator {
		LucasIterator { basic_lucas: BasicLucasSeqIterator::new(2, 1) }
	}
}

impl Iterator for LucasIterator {
	type Item = i64;

	fn next(&mut self) -> Option<i64> {
		self.basic_lucas.next()
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn gives_correct_values() {
		let f = super::LucasIterator::new();
		let expected = [2, 1, 3, 4, 7, 11, 18, 29];
		for (e, a) in expected.iter().zip(f) {
			assert_eq!(*e, a);
		}
	}
}