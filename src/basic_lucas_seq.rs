/// Iterator that gives the numbers in a "basic" Lucas sequence
///
/// A basic Lucas sequence is a sequence of integers defined by:
///
/// a(1) = (arbitrary), a(2) = arbitrary, a(n) = a(n-1) + a(n-2) for n > 2
///
/// These are particular cases of Lucas sequences in which P = 1 and Q = -1.
/// Fibonacci numbers and Lucas numbers are both basic Lucas sequences.
#[derive(Debug, Clone)]
pub struct BasicLucasSeqIterator {
	current: i64,
	next: i64,
}

impl BasicLucasSeqIterator {
	/// Create a new basic Lucas sequence iterator with
	/// the given starting values.
	pub fn new(first: i64, second: i64) -> BasicLucasSeqIterator {
		BasicLucasSeqIterator { current: second - first, next: first }
	}
}

impl Iterator for BasicLucasSeqIterator {
	type Item = i64;

	fn next(&mut self) -> Option<i64> {
		let new_next = self.current + self.next;
		self.current = self.next;
		self.next = new_next;
		Some(self.current)
	}
}