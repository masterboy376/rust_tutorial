fn main() {

    //--------------Objects Contain Data and Behavior--------------
    // Rust is object-oriented: structs and enums have data, and impl blocks provide methods on structs and enums
    
    //--------------Encapsulation that Hides Implementation Details--------------
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }
    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }
    
        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }
    
        pub fn average(&self) -> f64 {
            self.average
        }
    
        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
    
    //--------------Inheritance as a Type System and as Code Sharing--------------
    // trairs
    
    //--------------Polymorphism--------------
    // Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called bounded parametric polymorphism.
    
    //--------------Trait--------------
    pub trait Draw {
        fn draw(&self);
    }
    // pub struct Screen<T: Draw> {
    //     pub components: Vec<T>,
    // }
    // impl<T> Screen<T>
    // where
    //     T: Draw,
    // {
    //     pub fn run(&self) {
    //         for component in self.components.iter() {
    //             component.draw();
    //         }
    //     }
    // }
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    
    //--------------Implementing the Trait--------------
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            // code to actually draw a button
        }
    }
    // selectbox
    use gui::Draw;
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
        }
    }
    // Using trait objects to store values of different types that implement the same trait
    use gui::{Button, Screen};
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
    //  Error: Attempting to use a type that doesn’t implement the trait object’s trait
    // use gui::Screen;
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };
    // screen.run();
    
    //--------------Trait Objects Perform Dynamic Dispatch--------------
    // The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time. This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
    // When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call
    
    //--------------object safety--------------
    // 1. return type is not self
    // 2. no generic parameters
    // note: only use static dispatch when there is object safety otherwise use dynamic dispatch
    
    //--------------Implementing an Object-Oriented Design Pattern--------------
    // Filename: src/main.rs : 
    use blog::Post;
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    // Filename: src/lib.rs : 
    pub struct Post {
        content: String,
    }
    pub struct DraftPost {
        content: String,
    }
    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }
        pub fn content(&self) -> &str {
            &self.content
        }
    }
    impl DraftPost {
        // --snip--
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }
    pub struct PendingReviewPost {
        content: String,
    }
    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
    

}
