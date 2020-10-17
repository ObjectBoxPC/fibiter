use iterator::RecursiveSequenceIterator;

/// Returns an iterator over the Fibonacci numbers.
/// This iterator skips the initial zero. For an iterator that includes it,
/// use `fibonacci_zero()`.
pub fn fibonacci() -> RecursiveSequenceIterator {
	RecursiveSequenceIterator::new(1, 1, 1, -1)
}

/// Returns an iterator over the Fibonacci numbers, including the initial zero.
pub fn fibonacci_zero() -> RecursiveSequenceIterator {
	RecursiveSequenceIterator::new(0, 1, 1, -1)
}

/// Returns an iterator over the Lucas numbers.
pub fn lucas_numbers() -> RecursiveSequenceIterator {
	RecursiveSequenceIterator::new(2, 1, 1, -1)
}

/// Returns an iterator over the Pell numbers.
pub fn pell() -> RecursiveSequenceIterator {
	RecursiveSequenceIterator::new(0, 1, 2, -1)
}

/// Returns an iterator over the Jacobsthal numbers.
pub fn jacobsthal() -> RecursiveSequenceIterator {
	RecursiveSequenceIterator::new(0, 1, 1, -2)
}

/// Returns an iterator over the Lucas sequence of the first kind
/// with the given values of P and Q.
pub fn lucas_first(p: i64, q: i64) -> RecursiveSequenceIterator {
	RecursiveSequenceIterator::new(0, 1, p, q)
}

/// Returns an iterator over the Lucas sequence of the second kind
/// with the given values of P and Q.
pub fn lucas_second(p: i64, q: i64) -> RecursiveSequenceIterator {
	RecursiveSequenceIterator::new(2, p, p, q)
}

#[cfg(test)]
mod tests {
	use std::iter::Iterator;

	#[test]
	fn fibonacci_gives_correct_values() {
		assert_correct_values(
			&[1, 1, 2, 3, 5, 8, 13, 21, 34, 55],
			super::fibonacci(),
		);
	}

	#[test]
	fn fibonacci_zero_gives_correct_values() {
		assert_correct_values(
			&[0, 1, 1, 2, 3, 5, 8, 13, 21, 34],
			super::fibonacci_zero(),
		);
	}

	#[test]
	fn lucas_numbers_gives_correct_values() {
		assert_correct_values(
			&[2, 1, 3, 4, 7, 11, 18, 29, 47, 76],
			super::lucas_numbers(),
		);
	}

	#[test]
	fn pell_gives_correct_values() {
		assert_correct_values(
			&[0, 1, 2, 5, 12, 29, 70, 169, 408, 985],
			super::pell(),
		);
	}

	#[test]
	fn jacobsthal_gives_correct_values() {
		assert_correct_values(
			&[0, 1, 1, 3, 5, 11, 21, 43, 85, 171],
			super::jacobsthal(),
		);
	}

	#[test]
	fn lucas_first_gives_correct_values() {
		assert_correct_values(
			&[0, 1, 1, 2, 3, 5, 8, 13, 21, 34],
			super::lucas_first(1, -1),
		);
		assert_correct_values(
			&[0, 1, 2, 5, 12, 29, 70, 169, 408, 985],
			super::lucas_first(2, -1),
		);
	}

	#[test]
	fn lucas_second_gives_correct_values() {
		assert_correct_values(
			&[2, 1, 3, 4, 7, 11, 18, 29, 47, 76],
			super::lucas_second(1, -1),
		);
		assert_correct_values(
			&[2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786],
			super::lucas_second(2, -1),
		);
	}

	fn assert_correct_values<T>(expected: &[i64], actual: T)
		where T: Iterator<Item = i64> {
		for (e, a) in expected.iter().zip(actual) {
			assert_eq!(*e, a);
		}
	}
}