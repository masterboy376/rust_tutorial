fn main() {

    //--------------smart pointers - box--------------
    // 1.When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // 2.When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // 3.When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    let b = Box::new(5);
    println!("b = {}", b);
    
    //--------------Enabling Recursive Types with Boxes--------------
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    // let list = Cons(1, Cons(2, Cons(3, Nil))); // error: recursive type `List` has infinite size
    enum List { // linked list
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    //--------------Treating Smart Pointers Like Regular References with the Deref Trait--------------
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("{}",y);
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("{}",y);
    
    //--------------Defining Our Own Smart Pointer--------------
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y); //error
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));
    
    //--------------Implicit Deref Coercions with Functions and Methods--------------
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]); // The code we would have to write if Rust didn’t have deref coercion
    
    //--------------How Deref Coercion Interacts with Mutability--------------
    // 1.From &T to &U when T: Deref<Target=U>
    // 2.From &mut T to &mut U when T: DerefMut<Target=U>
    // 3.From &mut T to &U when T: Deref<Target=U>
    
    //--------------Running Code on Cleanup with the Drop Trait--------------
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    
    //--------------Dropping a Value Early with std::mem::drop--------------
    // let c = CustomSmartPointer { // error:explicit destructor call not allowed
    //     data: String::from("some data"),
    // };
    // println!("CustomSmartPointer created.");
    // c.drop();
    // println!("CustomSmartPointer dropped before the end of main.");
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    //--------------Using Rc<T> to Share Data--------------
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }
    // use crate::List::{Cons, Nil};
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // error: use of moved value 'a'
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use crate::List::{Cons, Nil};
    use std::rc::Rc;
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a)); // this increase the reference count of a rather than making a deep copy
    
    //--------------Cloning an Rc<T> Increases the Reference Count--------------
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    //--------------RefCell<T> and the Interior Mutability Pattern--------------
    // Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing
    
    //--------------Enforcing Borrowing Rules at Runtime with RefCell<T>--------------
    // Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
    // 1.Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    // 2.Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
    // 3.Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
    
    //--------------Interior Mutability: A Mutable Borrow to an Immutable Value--------------
    // let x = 5;
    // let y = &mut x; // error : x is not mutable
    
    //--------------A Use Case for Interior Mutability: Mock Objects--------------
    pub trait Messenger {
        fn send(&self, msg: &str);
    }
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }
        pub fn set_value(&mut self, value: usize) {
            self.value = value;
    
            let percentage_of_max = self.value as f64 / self.max as f64;
    
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }pub trait Messenger {
        fn send(&self, msg: &str);
    }
    
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    
    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }
    
        pub fn set_value(&mut self, value: usize) {
            self.value = value;
    
            let percentage_of_max = self.value as f64 / self.max as f64;
    
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }
    // #[cfg(test)] 
    // mod tests {
    //     use super::*;
    //     struct MockMessenger {
    //         sent_messages: Vec<String>,
    //     }
    //     impl MockMessenger {
    //         fn new() -> MockMessenger {
    //             MockMessenger {
    //                 sent_messages: vec![],
    //             }
    //         }
    //     }
    //     impl Messenger for MockMessenger {
    //         fn send(&self, message: &str) {
    //             self.sent_messages.push(String::from(message));
    //         }
    //     }
    //     #[test]
    //     fn it_sends_an_over_75_percent_warning_message() {
    //         let mock_messenger = MockMessenger::new();
    //         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    //         limit_tracker.set_value(80);
    //         assert_eq!(mock_messenger.sent_messages.len(), 1);
    //     }
    // } // error: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
    #[cfg(test)]
    mod tests { // Using RefCell<T> to mutate an inner value while the outer value is considered immutable
        use super::*;
        use std::cell::RefCell;
        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }
        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }
        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message));
            }
        }
        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
    }
    
    //--------------Keeping Track of Borrows at Runtime with RefCell<T>--------------
    // impl Messenger for MockMessenger { // error : Creating two mutable references in the same scope to see that RefCell<T> will panic
    //     fn send(&self, message: &str) {
    //         let mut one_borrow = self.sent_messages.borrow_mut();
    //         let mut two_borrow = self.sent_messages.borrow_mut();
    //         one_borrow.push(String::from(message));
    //         two_borrow.push(String::from(message));
    //     }
    // }
    
    //--------------Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>--------------
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }
    use crate::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    
    //--------------Creating a Reference Cycle--------------
    use crate::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
    
    //--------------Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>--------------
    // When you call Rc::downgrade, you get a smart pointer of type Weak<T>. Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1. The Rc<T> type uses weak_count to keep track of how many Weak<T> references exist, similar to strong_count. The difference is the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.
    
    //--------------Creating a Tree Data Structure: a Node with Child Nodes--------------
    use std::cell::RefCell;
    use std::rc::Rc;
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    //--------------Adding a Reference from a Child to Its Parent--------------
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    //--------------Visualizing Changes to strong_count and weak_count--------------
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

}
