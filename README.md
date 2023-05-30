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
#[derive(New)]
let instance = MyStruct::new(34); // Create an instance with default value
```
a and b is already defined with value 32 and true

Please note that the arguments passed to the new function should match the order and type of the struct's attributes.

### Modifiers
The `new_macro` crate comes with a set of modifiers that can be used to customize the generated function.

Example:
```rust
let instance = MyStruct::new(12)
        .with_a(34)
        .with_b(false);
```
The `a` and `b` attributes are now set to 34 and false respectively.

You can use new_without_default to generate a instance without default value
```rust
let instance = MyStruct::new_without_default(34, false, 12)
```


## Contribution
Contributions are welcome! If you want to improve the new_macro crate, please follow these steps:

Fork the project

Create a new branch (`git checkout -b feature/YourFeature`)

Make your modifications

Commit your changes (`git commit -m 'Add YourFeature'`)

Push the branch (`git push origin feature/YourFeature`)

Open a pull request
