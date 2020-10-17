# fibiter

Simple iterators in Rust that give Fibonacci numbers and other Lucas sequences

## Examples

```rust
let f = fibiter::fibonacci();

for x in f.take(5) {
	println!("{}", x);
}
```

Output:

```
1
1
2
3
5
```

```rust
let l = fibiter::lucas_numbers();

for x in l.take(5) {
	println!("{}", x);
}
```

Output:

```
2
1
3
4
7
```