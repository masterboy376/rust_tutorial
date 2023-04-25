fn main() {

    //--------------memory allocation--------------
    fn a(){
        let x:&str = "Hello"; //stack
        let y:i32 = 32; //stack
        b();
    }
    fn b(){
        let x:String = String::from("world"); //heap
    }
    
    //--------------ownership rule--------------
    // 1. Each value in Rust has a variable that is known an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    { // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward -> stored in stack
        let s = String::from("hello"); // -> stored in heap
        // do stuff with s
    } // this scope is now over, and s is no longer valid
    
    //--------------shallow copy--------------
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // error
    
    //--------------clone or copy--------------
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // note: integer, float, boolean, character is always copied
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    
    //--------------taking ownership--------------
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
                                    
     //--------------giving ownership--------------
     let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
                                        
    //--------------need for refrences--------------
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    
    //--------------using refrences--------------
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);
    
    //--------------mutable refrence--------------
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
    let mut s = String::from("hello");
    change(&mut s);
    
    //--------------only one mutable refrence--------------
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // error
    
    //--------------can't have mutable refrence when immutable references extists--------------
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // use this
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
    
    //--------------dangling refrences--------------
    // let reference_to_nothing = dangle();
    
    //--------------string slicing--------------
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    // let hello = &s[..5]; // valid
    let world = &s[6..11];
    // let world = &s[6..]; // valid
    let word = first_word(&s); // word will get the value 5
    // s.clear(); // ERROR:  it requires mutable ref which is not possible as we already have immutable ref in word
    // println!("the first word is: {}", word);
    
    //--------------string litreal--------------
    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    
    //--------------array slicing--------------
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // error
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
