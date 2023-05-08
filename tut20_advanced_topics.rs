fn main() {

    //--------------Unsafe Rust--------------
    // 1.Dereference a raw pointer
    // 2.Call an unsafe function or method
    // 3.Access or modify a mutable static variable
    // 4.Implement an unsafe trait
    // 5.Access fields of unions
    
    //--------------Dereferencing a Raw Pointer--------------
    // Different from references and smart pointers, raw pointers:
    // 1.Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    // 2.Aren’t guaranteed to point to valid memory
    // 3.Are allowed to be null
    // 4.Don’t implement any automatic cleanup
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    
    //--------------Calling an Unsafe Function or Method--------------
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
    
    //--------------Creating a Safe Abstraction over Unsafe Code--------------
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    // only safe rust
    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();
    //     assert!(mid <= len);
    //     (&mut values[..mid], &mut values[mid..])
    // } // error: cannot borrow `*values` as mutable more than once at a time
    // unsafe rust 
    use std::slice;
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();
        assert!(mid <= len);
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    
    //--------------Using extern Functions to Call External Code--------------
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    
    //--------------Accessing or Modifying a Mutable Static Variable--------------
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);
    static mut COUNTER: u32 = 0;
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    
    //--------------Implementing an Unsafe Trait--------------
    unsafe trait Foo {
        // methods go here
    }
    unsafe impl Foo for i32 {
        // method implementations go here
    }
    
    //--------------Accessing Fields of a Union--------------
    // The final action that works only with unsafe is accessing fields of a union. A union is similar to a struct, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code
    
    
    //--------------Specifying Placeholder Types in Trait Definitions with Associated Types--------------
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            // --snip--
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    
    //--------------Specifying Placeholder Types in Trait Definitions with Associated Types--------------
    use std::ops::Add;
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
    
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    trait Add<Rhs=Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    use std::ops::Add;
    struct Millimeters(u32);
    struct Meters(u32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    //--------------Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name--------------
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    let person = Human;
    person.fly(); // When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    
    //--------------Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name--------------
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    impl OutlinePrint for Point {}
    use std::fmt;
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    let p = Point { x: 1, y: 3 };
    p.outline_print();
    
    //--------------Using the Newtype Pattern to Implement External Traits on External Types--------------
    use std::fmt;
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    //--------------Creating Type Synonyms with Type Aliases--------------
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        // --snip--
    }
    fn returns_long_type() -> Thunk {
        // --snip--
    }
    use std::fmt;
    type Result<T> = std::result::Result<T, std::io::Error>;
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;
    
        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
    
    //--------------The Never Type that Never Returns--------------
    fn bar() -> ! {
        panic!();
    }
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        // --snip--
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        // --snip--
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    enum Option<T> {
        Some(T),
        None,
    }
    use crate::Option::*;
    
    impl<T> Option<T> {
        pub fn unwrap(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }
    print!("forever ");
    loop {
        print!("and ever ");
    }
    
    //--------------Dynamically Sized Types and the Sized Trait--------------
    fn generic<T>(t: T) {
        // --snip--
    }
    fn generic<T: Sized>(t: T) {
        // --snip--
    }
    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
    
    //--------------Function Pointers--------------
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    
    //--------------Returning Closures--------------
    // fn returns_closure() -> dyn Fn(i32) -> i32 { // error: return type cannot have an unboxed trait object
    //     |x| x + 1
    // }
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    
    //--------------Macros--------------
    // 1.Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
    // 2.Attribute-like macros that define custom attributes usable on any item
    // 3.Function-like macros that look like function calls but operate on the tokens specified as their argument
    
    //--------------The Difference Between Macros and Functions--------------
    // Another important difference between macros and functions is that you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.
    
    //--------------Declarative Macros with macro_rules! for General Metaprogramming-------------
    let v: Vec<u32> = vec![1, 2, 3];
    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    }
    
    //--------------Procedural Macros for Generating Code from Attributes-------------
    use proc_macro;
    #[some_attribute]
    pub fn some_name(input: TokenStream) -> TokenStream {
    }
    
    //--------------How to Write a Custom derive Macro-------------
    // use hello_macro::HelloMacro;
    // use hello_macro_derive::HelloMacro;
    // #[derive(HelloMacro)]
    // struct Pancakes;
    // fn main() {
    //     Pancakes::hello_macro();
    // }
    // it works
    use hello_macro::HelloMacro;
    struct Pancakes;
    impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Pancakes!");
        }
    }
    Pancakes::hello_macro();
    // Filename: hello_macro_derive/Cargo.toml
    [lib]
    proc-macro = true
    [dependencies]
    syn = "1.0"
    quote = "1.0"
    // use proc_macro::TokenStream;
    // use quote::quote;
    // use syn;
    // #[proc_macro_derive(HelloMacro)]
    // pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    //     // Construct a representation of Rust code as a syntax tree
    //     // that we can manipulate
    //     let ast = syn::parse(input).unwrap();
    //     // Build the trait implementation
    //     impl_hello_macro(&ast)
    // }
    // define
    DeriveInput {
        // --snip--
        ident: Ident {
            ident: "Pancakes",
            span: #0 bytes(95..103)
        },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Unit,
                semi_token: Some(
                    Semi
                )
            }
        )
    }
    //func
    fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let gen = quote! {
            impl HelloMacro for #name {
                fn hello_macro() {
                    println!("Hello, Macro! My name is {}!", stringify!(#name));
                }
            }
        };
        gen.into()
    }
    // path
    hello_macro = { path = "../hello_macro" }
    hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
    
    //--------------Attribute-like macros-------------
    #[route(GET, "/")]
    fn index() {
    //or
    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    
    //--------------Function-like macros-------------
    let sql = sql!(SELECT * FROM posts WHERE id=1);
    //or
    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {
  
}
