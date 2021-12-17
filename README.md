# User Input with autocomplete

This crate provides a way to get user input with autocomplete optional auto complete functionality. Very simple to use, and parsing the input is done in a very simple way.

## Usage
To use this crate you need to add the following to your `Cargo.toml`:
```toml
[dependencies]
user_input_with_autocomplete = "*" # or just copy the current crate version number istead of *
```

To use it in the code, you need to import the `user_input_with_autocomplete` crate:
```rust
use user_input_with_autocomplete::input::UserInput;
```
Then you can use the `UserInput` struct to get user input:
```rust
// Example
fn main() {
    let a = UserInput::new("Enter a number: ", None); // Here the first argument to the constructor of the struct is the prompt, and the second argument is to use auto-complete or not.
    // To use the auto-complete feature instead of None, you need to provide a function of the following type
    // ```Some(fn(String) -> String)```
    let b = a.to_string();
    let c: i32 = a.parse().unwrap(); // It's simple to parse the input and you can even parse it as a vector of strings or specified type, seperated by spaces.

}
```

## Features
* `autocomplete` - This feature enables the auto-complete feature.

## License
MIT

## Contributing
Feel free to open an issue, submit a pull request or make a suggestion.