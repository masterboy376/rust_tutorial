fn main() {

    //--------------customize builds with release profile--------------
    // $ cargo build
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.0s
    // $ cargo build --release
    //     Finished release [optimized] target(s) in 0.0s
    // Filename: Cargo.toml : 
    [profile.dev]
    opt-level = 0
    [profile.release]
    opt-level = 3
    // Filename: Cargo.toml : over riding
    [profile.dev]
    level = 1
    
    //--------------making useful documentation comments--------------
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    // running cargo doc --open will build the HTML for your current crate’s documentation (as well as the documentation for all of your crate’s dependencies) and open the result in a web browser. Navigate to the add_one function and you’ll see how the text in the documentation comments is rendered
    
    //--------------documentation comments as test--------------
       Doc-tests my_crate
    running 1 test
    test src/lib.rs - add_one (line 5) ... ok
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
    
    //--------------//!--------------
    //! # My Crate
    //!
    //! `my_crate` is a collection of utilities to make performing certain
    //! calculations more convenient.
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    //--------------Exporting a Convenient Public API with pub use--------------
    // Filename: src/lib.rs :
    //! # Art
    //!
    //! A library for modeling artistic concepts.
    pub mod kinds {
        /// The primary colors according to the RYB color model.
        pub enum PrimaryColor {
            Red,
            Yellow,
            Blue,
        }
        /// The secondary colors according to the RYB color model.
        pub enum SecondaryColor {
            Orange,
            Green,
            Purple,
        }
    }
    pub mod utils {
        use crate::kinds::*;
    
        /// Combines two primary colors in equal amounts to create
        /// a secondary color.
        pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
            // --snip--
        }
    }
    // Filename: src/main.rs : 
    use art::kinds::PrimaryColor;
    use art::utils::mix;
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
    // Filename: src/lib.rs : 
    //! # Art
    //!
    //! A library for modeling artistic concepts.
    pub use self::kinds::PrimaryColor;
    pub use self::kinds::SecondaryColor;
    pub use self::utils::mix;
    pub mod kinds {
        // --snip--
        /// The primary colors according to the RYB color model.
        pub enum PrimaryColor {
            Red,
            Yellow,
            Blue,
        }
        /// The secondary colors according to the RYB color model.
        pub enum SecondaryColor {
            Orange,
            Green,
            Purple,
        }
    }
    pub mod utils {
        // --snip--
        use crate::kinds::*;
    
        /// Combines two primary colors in equal amounts to create
        /// a secondary color.
        pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
            SecondaryColor::Orange
        }
    }
    // Filename: src/main.rs : 
    use art::mix;
    use art::PrimaryColor;
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);

    //--------------Setting up cargo.io account--------------
    // $ cargo login abcdefghijklmnopqrstuvwxyz012345

    //--------------Adding metadata to a crate--------------
    //Filename: Cargo.toml : 
    [package]
    name = "guessing_game"
    version = "0.1.0"
    edition = "2021"
    description = "A fun game where you guess what number the computer has chosen."
    license = "MIT OR Apache-2.0"
    [dependencies]
    
    //--------------Publishing to crate.io or Publishing a New Version of an Existing Crate--------------
    $ cargo publish
        Updating crates.io index
       Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
       Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
       Compiling guessing_game v0.1.0
    (file:///projects/guessing_game/target/package/guessing_game-0.1.0)
        Finished dev [unoptimized + debuginfo] target(s) in 0.19s
       Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
       
    //--------------Deprecating Versions from Crates.io with cargo yank--------------
    $ cargo yank --vers 1.0.1
        Updating crates.io index
            Yank guessing_game@1.0.1
    // to undo
    $ cargo yank --vers 1.0.1 --undo
        Updating crates.io index
          Unyank guessing_game@1.0.1
    
    //--------------Creating a Workspace--------------
    $ mkdir add
    $ cd add
    // Filename: Cargo.toml : 
    [workspace]
    members = [
        "adder",
    ]
    // create adder binary crate
    $ cargo new adder
         Created binary (application) `adder` package
    // at this point we can build a new binary crate
    cargo build
    // add directory looks like this
    //     ├── Cargo.lock
    //     ├── Cargo.toml
    //     ├── adder
    //     │   ├── Cargo.toml
    //     │   └── src
    //     │       └── main.rs
    //     └── target
    // Creating the Second Package in the Workspace
    [workspace]
    members = [
        "adder",
        "add_one",
    ]
    // generate new library crate add-one
    $ cargo new add_one --lib
         Created library `add_one` package
    // add directory looks like this
    // ├── Cargo.lock
    // ├── Cargo.toml
    // ├── add_one
    // │   ├── Cargo.toml
    // │   └── src
    // │       └── lib.rs
    // ├── adder
    // │   ├── Cargo.toml
    // │   └── src
    // │       └── main.rs
    // └── target
    // Filename: add_one/src/lib.rs :
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    // Filename: adder/Cargo.toml :
    [dependencies]
    add_one = { path = "../add_one" }
    // Filename: adder/src/main.rs :
    use add_one;
    fn main() {
        let num = 10;
        println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
    }
    $ cargo build
       Compiling add_one v0.1.0 (file:///projects/add/add_one)
       Compiling adder v0.1.0 (file:///projects/add/adder)
        Finished dev [unoptimized + debuginfo] target(s) in 0.68s
    $ cargo run -p adder
        Finished dev [unoptimized + debuginfo] target(s) in 0.0s
         Running `target/debug/adder`
    Hello, world! 10 plus one is 11!
    
    //--------------Depending on an External Package in a Workspace--------------
    // Filename: add_one/Cargo.toml : 
    [dependencies]
    rand = "0.8.5" // this will not be available in adder crate
    $ cargo build
        Updating crates.io index
      Downloaded rand v0.8.5
       --snip--
       Compiling rand v0.8.5
       Compiling add_one v0.1.0 (file:///projects/add/add_one)
    warning: unused import: `rand`
     --> add_one/src/lib.rs:1:5
      |
    1 | use rand;
      |     ^^^^
      |
      = note: `#[warn(unused_imports)]` on by default
    
    warning: `add_one` (lib) generated 1 warning
       Compiling adder v0.1.0 (file:///projects/add/adder)
        Finished dev [unoptimized + debuginfo] target(s) in 10.18s
        
    //--------------Adding a Test to a Workspace--------------
    // Filename: add_one/src/lib.rs :
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn it_works() {
            assert_eq!(3, add_one(2));
        }
    }
    // run: $ cargo test or$ cargo test -p add_one
    
    //--------------Installing Binaries with cargo install--------------
    $ cargo install ripgrep
    // All binaries installed with cargo install are stored in the installation root’s bin folder. If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be $HOME/.cargo/bin. Ensure that directory is in your $PATH to be able to run programs you’ve installed with cargo install.
    
    //--------------Extending Cargo with Custom Commands--------------
    // $PATH is named cargo-something can be run as $ cargo something
    $ cargo --list
    $ cargo install
    
}
