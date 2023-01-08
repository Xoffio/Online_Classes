# 02.01 to 02.04

In section two of the class we learn more about testing in Rust.

Rusts tests help you check if a program is working as expected.

Rust test functions are defines using the `#[test]` attribute inside the test module.
You can run tests with `cargo test`
You can have multiple test functions.

Example
```rust
#[cfg(test)]
mod tests {
    use super::*;
	
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
	
    #[test]
    fn it_fails() {
        let result = add(2, 2);
        assert_eq!(result, 0);
    }
}
```

```terminal
running 2 tests
test tests::it_fails ... FAILED
test tests::it_works ... ok
```

## Useful `macros` to test code

### `assert!` Macro
- Evaluates a single Boolean argument 
- If `true`, do nothing
- If `false` call the `panic!` macro
- Can have an extra argument for the message that will be shown when `panic`

### `assert_eq!` Macro
- Compares tow arguments for equality.
- If *equal*, do nothing
- If *not equal* call the `panic!` macro
- Can have an extra argument for the message that will be shown when `panic`

### `assert_ne!` Macro
- Compares tow arguments for inequality.
- If *not equal*, do nothing
- If *equal* call the `panic!` macro
- Can have an extra argument for the message that will be shown when `panic`

> [!Important]
> In the background the `assert_eq` and `assert_ne` macros use `==` and `!=` operators for comparison. The arguments must implement the `PartialOrd` trait to be compared. Arguments must implement the `Debug` trait to be displayed. If you defined your own custom types then you have to implement those traits yourself or add the derive attribute to your custom types to derive a default implementation `#derive(PartialEq, Debug`)


