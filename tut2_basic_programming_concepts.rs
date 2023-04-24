fn main() {

    //--------------variable--------------
    let x = 5;
    println!("{x}");
    // x = 7; // error
    println!("{x}");
    let mut y = 5;
    println!("{y}");
    y = 7; // valid
    println!("{y}");
    
    //--------------constant--------------
    let MY_AGE:u32 = 18; // must be asigned a contant value and type must be mentiones
    
    //--------------shadowing--------------
    let z = 5;
    println!("{z}");
    let z = 7;
    println!("{z}");
    
    //--------------scaler datatypes--------------
    // integer
    let a = 92_888; // decimal
    let b = 0xff; // hex
    let c = 0o77; // octal
    let d = 0b1111_0000; // binary
    let e = b'A'; // byte (u8 only)
    let f:u8 = 255;
    // floating point number
    let g:f64 = 2.0;
    let h:f32 = 3.0;
    let sum = 5+10;
    let sub = 90.8-7.6;
    let mul = 6*7;
    let div = 8.7/4.35;
    let rem = 40%9;
    // boolean
    let t:bool = true;
    let f:bool = false;
    // character
    let ch1:char = '2';
    let ch2:char = 'Z';
    
    //--------------compound datatypes--------------
    // tuple
    let tup:(&str, i32) = ("hello world", 100_000);
    let str1 = tup.0;
    let i1 = tup.1;
    let (str2, i2) = tup;
    // array
    let ar = [1, 2, 3, 4, 5];
    let firstTerm = ar[1];
    let bytes = [0;8];
    
    //--------------function call--------------
    let ans:i32 = my_func(1,2);
    
    //--------------conditionals--------------
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    //--------------loop--------------
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    
    //--------------while loop--------------
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    //--------------for loop--------------
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("the value is: {element}");
    }
    // loop over range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    
    //--------------comments--------------
    // single line comment
    /*
    multilinr comment
    */
}

//--------------functions--------------
fn my_func(x:i32,y:i32)->i32{
    println!("another function {x} {y}");
    // return x+y;
    x+y
}
