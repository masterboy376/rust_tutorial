fn main() {

// ------------------------variable------------------------
// `let` will create a new variable binding
let foo = 1;
// bindings are immutable by default
// foo = 2; // ERROR: cannot assign; `foo` not mutable
let mut bar = 1; // create mutable binding
bar = 2; // OK to mutate
let baz = 'a'; // use single quotes to create a character
let baz = "ok"; // use double quotes to create a string
// variables can be shadowed, so these lines have been valid
let baz = 42; // `baz` is now an integer; 'a' and "ok" no longer accessible
// Rust infers types, but you can use annotations as well
let foo: i32 = 50; // set `foo` to i32
let foo: u8 = 100; // set `foo` to u8
// let foo: u8 = 256; // ERROR: 256 too large to fit in u8
let bar = 14.5_f32; // underscore can be used to set numeric type...
let bar = 99_u8;
let bar = 1_234_567; // ...and also to make it easier to read long numbers
let baz; // variables can start uninitialized, but they must be set before usage
// let foo = baz; // ERROR: possibly uninitialized.
baz = 0; // `baz` is now initialized
// baz = 1; // ERROR: didn't declare baz as mutable
// naming convention:
let use_snake_case_for_variables = ();

// ------------------------variable------------------------
// `const` will create a new constant value
const PEACE: char = '☮'; // type annotations are required
const MY_CONST: i32 = 4; // naming conventions is SCREAMING_SNAKE_CASE
// const UNINIT_CONST: usize; // ERROR: must have initial value for constants
// use `once_cell` crate if you need lazy initialization of a constant
use once_cell::sync::OnceCell;
const HOME_DIR: OnceCell<String> = OnceCell::new();
// use .set to set the value (can only be done once)
HOME_DIR.set(std::env::var("HOME").expect("HOME not set"));
// use .get to retrieve the value
HOME_DIR.get().unwrap();

}
