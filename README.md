# new_macro
The Rust macro derives a new function with optional default attribute for simplified struct instantiation. It eliminates repetitive code and enhances readability.

## Usage

To use the `new_macro` macro derive in your project, follow these steps:

1. Import the `new_macro` crate in your code:

```rust
use new_macro::New;
```
Annotate your struct with the New attribute, specifying the attributes that should have default values:
```rust
#[derive(New)]
struct MyStruct {
    #[default = 32]
    a: u32,
    #[default = true]
    b: bool,
    c: i32
    // Other attributes...
}
```
The macro will generate a new function for your struct that accepts optional arguments:
```rust
let instance = MyStruct::new(Some(32), None, 34); // Create an instance with custom values
```
In the example above, the new function is called with specific arguments to provide custom values for the attributes. The first attribute a is set to 32

Please note that the arguments passed to the new function should match the order and type of the struct's attributes.

## Contribution
Contributions are welcome! If you want to improve the new_macro crate, please follow these steps:

Fork the project
Create a new branch (`git checkout -b feature/YourFeature`)
Make your modifications
Commit your changes (`git commit -m 'Add YourFeature'`)
Push the branch (`git push origin feature/YourFeature`)
Open a pull request
