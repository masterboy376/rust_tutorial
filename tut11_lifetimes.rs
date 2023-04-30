fn main() {

    //--------------validating references with lifetime--------------
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r); // x is out of scope by his time
    //fix
    // let x = 5;            // ----------+-- 'b
    //                       //           |
    // let r = &x;           // --+-- 'a  |
    //                       //   |       |
    // println!("r: {}", r); //   |       |
    //                       // --+       |
    //                       // ----------+
    
    //--------------generic lifetime functions--------------
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // } // error: Rust can’t tell whether the reference being returned refers to x or y
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    
    //--------------lifetime anotation syntax--------------
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime
    
    //--------------lifetime anotation in function signature--------------
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {// We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // valid
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // not valid 
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    
    //--------------thinking in terms of lifetime--------------
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
    fn longest<'a>(x: &str, y: &str) -> &'a str {// error: the return value lifetime is not related to the lifetime of the parameters at all
        let result = String::from("really long string");
        result.as_str()
    }
    
    //--------------lifetime annotation with struct defination--------------
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    //--------------lifetime elision--------------
    fn first_word(s: &str) -> &str { // in older version it would have given error but in newer version of rust it's ok
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    
    //--------------lifetime annotation in methods--------------
    impl<'a> ImportantExcerpt<'a> { // it's ok due to first elision rule
        fn level(&self) -> i32 {
            3
        }
    }
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for
    
    //--------------static lifetime--------------
    let s: &'static str = "I have a static lifetime.";// valid for entire duration of program

    //--------------generic type parameters, trait bounds, and lifetimes together--------------
    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

}
