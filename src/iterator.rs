/// Iterator that gives the values in a particular form of recursive sequence.
///
/// Specifically, this iterator supports sequences of the form:
///
/// a(1) = (arbitrary)
/// a(2) = (arbitrary)
/// a(n) = p * a(n-1) - q * a(n-2), for n > 2
///
/// This is the form of Lucas sequences.
#[derive(Debug, Clone)]
pub struct RecursiveSequenceIterator {
	current: i64,
	next: i64,
	p: i64,
	q: i64,
}

impl RecursiveSequenceIterator {
	/// Create a new recursive sequence iterator with the given starting values
	/// and coefficients.
	pub fn new(first: i64, second: i64, p: i64, q: i64)
		-> RecursiveSequenceIterator {
		RecursiveSequenceIterator {
			current: first,
			next: second,
			p: p,
			q: q,
		}
	}
}

impl Iterator for RecursiveSequenceIterator {
	type Item = i64;

	fn next(&mut self) -> Option<i64> {
		let current = self.current;
		self.current = self.next;
		self.next = self.p * self.current - self.q * current;
		Some(current)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn gives_initial_values() {
		assert_correct_initial_values(0, 1, 1, 2);
		assert_correct_initial_values(0, 1, 3, 4);
		assert_correct_initial_values(2, 1, 1, 2);
		assert_correct_initial_values(2, 3, 3, 4);
	}

	fn assert_correct_initial_values(first: i64, second: i64, p: i64, q: i64) {
		let expected = [
			first,
			second,
			p * second - q * first,
			(p * p - q) * second - p * q * first,
		];
		let iter = super::RecursiveSequenceIterator::new(first, second, p, q);
		for (e, a) in expected.iter().zip(iter) {
			assert_eq!(*e, a);
		}
	}
}