fn main() {

    //--------------enum--------------
        enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    //--------------enum with value--------------
    // struct QuitMessage; // unit struct
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // tuple struct
    // struct ChangeColorMessage(i32, i32, i32); // tuple struct
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            println!("message called");
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    //--------------option enum--------------
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // error
    
    //--------------match control flow on enum--------------
    // all cases must be covered under match
    enum Coin1 {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin1) -> u8 {
        match coin {
            Coin1::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin1::Nickel => 5,
            Coin1::Dime => 10,
            Coin1::Quarter => 25,
        }
    }
    // enum nesting
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents_new(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    // matching options
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    //--------------matching all cases--------------
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    
    //--------------if let--------------
    //example 1
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    // example 2
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

}


