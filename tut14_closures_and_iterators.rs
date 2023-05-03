fn main() {

    //--------------capturing environment with closure--------------
    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }
    struct Inventory {
        shirts: Vec<ShirtColor>,
    }
    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }
        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;
    
            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    
    //--------------closure type and annotation--------------
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    // Attempting to call a closure whose types are inferred with two different types will throw an error
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
    
    //--------------capturing reference and moving ownership in closure--------------
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    // mutable list
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
    // Using move to force the closure for the thread to take ownership of list
    use std::thread;
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
        
    //--------------moving captured value out of closures and the function trait--------------
    //FnOnce
    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }
    // sorting -> FnMut closure
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    fn main() {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];
    
        list.sort_by_key(|r| r.width);
        println!("{:#?}", list);
    }
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    fn main() {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];
        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("{:#?}, sorted in {num_sort_operations} operations", list);
    }
    
    //--------------processing a series of items with iterator--------------
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
     
    //--------------the iterator trait and next method--------------
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
        // methods with default implementations elided
    }
    #[cfg(test)]
    mod tests {
        #[test]
        fn iterator_demonstration() {
            let v1 = vec![1, 2, 3];
    
            let mut v1_iter = v1.iter();
    
            assert_eq!(v1_iter.next(), Some(&1));
            assert_eq!(v1_iter.next(), Some(&2));
            assert_eq!(v1_iter.next(), Some(&3));
            assert_eq!(v1_iter.next(), None);
        }
    }
    #[cfg(test)]
    mod tests {
        #[test]
        fn iterator_sum() {
            let v1 = vec![1, 2, 3];
    
            let v1_iter = v1.iter();
    
            let total: i32 = v1_iter.sum();
    
            assert_eq!(total, 6);
        }
    } // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.
    
    //--------------Methods that Produce Other Iterators--------------
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    
    //--------------Using closures that capture their environment--------------
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 13,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ];
            let in_my_size = shoes_in_size(shoes, 10);
    
            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 10,
                        style: String::from("sneaker")
                    },
                    Shoe {
                        size: 10,
                        style: String::from("boot")
                    },
                ]
            );
        }
    }
    
    //--------------Iterators are slightly faster than loops--------------
    // test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
    // test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
    // In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;
    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
                                     .zip(&buffer[i - 12..i])
                                     .map(|(&c, &s)| c * s as i64)
                                     .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }

}
