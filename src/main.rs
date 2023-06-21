
use new_macro::New;

#[derive(New)]
struct MyStruct {
    a: u32,
    b: bool,
    c: i32
    // Other attributes...
}


fn main() {
    let instance = MyStruct::new(12, true, 32);
}