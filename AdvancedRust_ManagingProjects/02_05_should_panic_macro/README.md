# 02.05

### `#[should_panic]`
Tells the Rust compiler that the next test function will pass the test if the function panics.
```rust
// Test pass because function should panic.
#[test]
#[should_panic]
fn person_panic_test() {
	let new_person = Person::new("".to_string());
	assert_eq!(new_person.name, "Steve".to_string());
}
```