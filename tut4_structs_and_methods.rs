fn main() {

    //--------------struct--------------
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // mutable
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    // snip
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    
    //--------------tuple struct--------------
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let randPoint = Point(2, 0, 0);
    println!("{}", randPoint.0);
    
    //--------------printing struct--------------
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    
    //--------------method syntax--------------
    #[derive(Debug)]
    struct Rectangle1 {
        width: u32,
        height: u32,
    }
    impl Rectangle1 {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle1) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    let rect1 = Rectangle1 {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle1 {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle1 {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle1::square(5);
    println!("rect1 is {:#?}", rect4);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    impl Rectangle1 {
    fn square(size: u32) -> Rectangle1 {
        Rectangle1 {
            width: size,
            height: size,
        }
    }
}
    
    
    // functions
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }
    fn build_user_sh(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

