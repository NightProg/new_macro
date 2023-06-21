# new_macro
The Rust macro derives a new function with  attribute for simplified struct instantiation. It eliminates repetitive code and enhances readability.

## Usage

To use the `new_macro` macro derive in your project, follow these steps:

1. Import the `new_macro` crate in your code:

```rust
use new_macro::New;
```
2. Annotate your struct with the New attribute:
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
The macro will generate a new function for your struct that accepts arguments:
```rust
#[derive(New)]
let instance = MyStruct::new(34, true, 3); // Create an instance 
```

Please note that the arguments passed to the new function should match the order and type of the struct's attributes.

