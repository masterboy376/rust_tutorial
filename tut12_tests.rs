fn main() {

    //--------------write test--------------
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }
    
        #[test]
        fn another() {
            panic!("Make this test fail");
        }
    }
    
    //--------------checking result with assert! macro--------------
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn larger_can_hold_smaller() {
            // --snip--
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };
    
            assert!(larger.can_hold(&smaller));
        }
        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };
            assert!(!smaller.can_hold(&larger));
        }
    }
    
    //--------------checking result with assert_eq! and assert_ne! macro--------------
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }
    
    //--------------custom failure message--------------
    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn greeting_contains_name() {
            let result = greeting("Carol");
            assert!(
                result.contains("Carol"),
                "Greeting did not contain name, value was `{}`",
                result
            );
        }
    }
    
    //--------------custom failure message--------------
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        #[should_panic]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
    // another example
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }
    
            Guess { value }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        #[should_panic(expected = "less than or equal to 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
    
    //--------------using Result<T,E> in tests--------------
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("two plus two does not equal four"))
            }
        }
    }
    
    //--------------running tests in parallel--------------
    // $ cargo test -- --test-threads=1 // if all test modify the same file on disk then run tests serially
    // by default all test run in parallel
    
    //--------------showing function output--------------
    // $ cargo test -- --show-output
    
    //--------------running tests by name--------------
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn add_two_and_two() {
            assert_eq!(4, add_two(2));
        }
        #[test]
        fn add_three_and_two() {
            assert_eq!(5, add_two(3));
        }
        #[test]
        fn one_hundred() {
            assert_eq!(102, add_two(100));
        }
    }
    // $ cargo test one_hundred
    
    //--------------filtering to runn multiple tests--------------
    // $ cargo test add // supply some part of test name
    //   Compiling adder v0.1.0 (file:///projects/adder)
    //     Finished test [unoptimized + debuginfo] target(s) in 0.61s
    //      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
    // running 2 tests
    // test tests::add_three_and_two ... ok
    // test tests::add_two_and_two ... ok
    // test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
    
    //--------------running tests by module name--------------
    // $ cargo test tests::

    //--------------ignoring some tests until specifically requested--------------
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
    // $ cargo test : test will be ignored
    // $ cargo test -- --ignored : test will be run
    
    //--------------for more information--------------
    // $ cargo test --help
    // $ cargo test -- --help
    
    //--------------unit tests--------------
    // so far we discussed unit test, it exists in the same file as product code
    
    //--------------testing private functions, as child module has access to all item of the parent module--------------
    pub fn add_two(a: i32) -> i32 {
        internal_adder(a, 2)
    }
    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn internal() {
            assert_eq!(4, internal_adder(2, 2));
        }
    }
    
    //--------------integration tests--------------
    // adder
    // ├── Cargo.lock
    // ├── Cargo.toml
    // ├── src
    // │   └── lib.rs
    // └── tests
    //     └── integration_test.rs
    // Filename: tests/integration_test.rs : 
    use adder;
    #[test]
    fn it_adds_two() {
    // only call public functions
        assert_eq!(4, adder::add_two(2));
    }
    //to run all tests
    // $ cargo test
    //to run only integration tests
    // $ cargo test --test integration_test
    
    //--------------submodules in integration tests--------------
    // Filename: tests/common.rs : 
    pub fn setup() {
        // setup code specific to your library's tests would go here
    }// not like this
    // instead
    // ├── Cargo.lock
    // ├── Cargo.toml
    // ├── src
    // │   └── lib.rs
    // └── tests
    //     ├── common
    //     │   └── mod.rs
    //     └── integration_test.rs
    // Filename: tests/common.rs : 
    pub fn setup() {
        // setup code specific to your library's tests would go here
    }
    // Filename: tests/integration_test.rs : 
    use adder;
    mod common; // will look for either common.rs or common/mod.rs
    #[test]
    fn it_adds_two() {
        common::setup();
        assert_eq!(4, adder::add_two(2));
    }
    
}
