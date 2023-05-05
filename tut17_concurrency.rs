fn main() {

    //--------------Using Threads to Run Code Simultaneously--------------
    // 1.Race conditions, where threads are accessing data or resources in an inconsistent order
    // 2.Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
    // 3.Bugs that happen only in certain situations and are hard to reproduce and fix reliably
    
    //--------------Creating a New Thread with spawn--------------
    use std::thread;
    use std::time::Duration;
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // when main thread ends spawned thread stops running
    
    //--------------Waiting for All Threads to Finish Using join Handles--------------
    use std::thread;
    use std::time::Duration;
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // handle.join().unwrap(); // blocking the main thread until spawned thread stops executing
    
    //--------------Using move Closures with Threads--------------
    // use std::thread;
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v); // error: closure may outlive the current function, but it borrows `v`, which is owned by the current function
    // });
    // handle.join().unwrap();
    // OR
    // use std::thread;
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });
    // drop(v); // oh no!
    // handle.join().unwrap();
    //good
    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    // use std::thread;
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });
    // drop(v); // error: v was moved into the closure
    // handle.join().unwrap();
    
    //--------------Using Message Passing to Transfer Data Between Threads--------------
    use std::sync::mpsc;
    use std::thread;
    let (tx, rx) = mpsc::channel(); // only one reciver
     thread::spawn(move || { // it is advisable not to share sender between threads
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    
    //--------------Channels and Ownership Transference--------------
    use std::sync::mpsc;
    use std::thread;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // error: val was moved earlier
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    
    //--------------Sending Multiple Values and Seeing the Receiver Waiting--------------
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
    
    //--------------Creating Multiple Producers by Cloning the Transmitter--------------
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
    
    //--------------Shared-State Concurrency--------------
    // In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time
    
    //--------------Using Mutexes to Allow Access to Data from One Thread at a Time--------------
    // 1.You must attempt to acquire the lock before using the data.
    // 2.When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
    
    //--------------The API of Mutex<T>--------------
    use std::sync::Mutex;
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
    
    //--------------Sharing a Mutex<T> Between Multiple Threads--------------
    // 1.
    // use std::sync::Mutex;
    // use std::thread;
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap()); // error: counter already moved
    // 2.
    // use std::rc::Rc;
    // use std::sync::Mutex;
    // use std::thread;
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());// error: Rc is not concurrency safe
    
    //--------------Atomic Reference Counting with Arc<T>--------------
    use std::sync::{Arc, Mutex};
    use std::thread;
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
    
    // You might have noticed that counter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides interior mutability, as the Cell family does. In the same way we used RefCell<T> in Chapter 15 to allow us to mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>.
    
    //--------------Extensible Concurrency with the Sync and Send Traits--------------
    // 1. Allowing Transference of Ownership Between Threads with Send
    // 2. Allowing Access from Multiple Threads with Sync
    // 3. Implementing Send and Sync Manually Is Unsafe

}
