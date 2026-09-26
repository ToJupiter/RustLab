use std::thread;
use std::time::Duration;
use std::sync::mpsc;

/*
 * Here are the topics we’ll cover in this chapter:
 
     How to create threads to run multiple pieces of code at the same time
     Message-passing concurrency, where channels send messages between threads
     Shared-state concurrency, where multiple threads have access to some piece of data
     The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

* The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread
 */
pub fn thread_spawn() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned threads", i);
            thread::sleep(Duration::from_millis(1));
        } 
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}


pub fn join_control() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi with number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi with number {} from the spawned thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

/*
 * Rust can’t tell how long the spawned thread will run, so it doesn’t know whether the reference to v will always be valid.
 */
pub fn thread_problems() {
    let v = vec![10, 20, 30, 40];

    // may outlive borrowed value `v`, causing errors
    // let handle = thread::spawn(|| {
    //     println!("The vector v is: {:?}", v);
    // });
    // handle(v).join().unwrap();

    let handle = thread::spawn(move || {
        println!("Here is the vector v: {:?}", v);
    });
    handle.join().unwrap();
}

/*
 * Transfer Data Between Threads with Message Passing.
 * Concept: channel (directional). 
 * The send function takes ownership of its parameter, and when the value is moved the receiver takes ownership of it. 
 */
pub fn message_passing_channel() {
    let (tx, rx) = mpsc::channel();
    println!("{:?}", tx);

    thread::spawn(move || {
       let val = String::from("hi");
       tx.send(val).unwrap();
       let val2 = String::from("Hello");
       tx.send(val2).unwrap();
    });

    // recv() blocks the main thread completely
    let received = rx.recv().unwrap();
    println!("Messaged received: {}", received);  // hi
    let received = rx.try_recv().unwrap();
    println!("Messaged received: {}", received);  // Hello


    let (tx2, rx2) = mpsc::channel::<i32>();
    let (tx3, rx3): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        let list_of_messages = vec![
            String::from("Hello"),
            String::from("Rust"),
            String::from("programming"),
            String::from("language")
        ];
       
       for val in list_of_messages {
           tx3.send(val).unwrap();
           thread::sleep(Duration::from_millis(1000));
       } 
    });

    for received_value in &rx3 {
        println!("Received: {}", received_value);
    }

    // After calling recv() 4 times, channel is closed permanently. 
    // Error when receiving: receiving on a closed channel!
    match &rx3.try_recv() {
        Ok(val) => println!("Received: {:?}", val),
        Err(e) => println!("Error when receiving: {}!", e)
    }


    thread::spawn(move || {
        let bread_count = 10;
        let apple_count = 20;
        let banana_count = 5;
        let bread_apple_banana = vec![bread_count, apple_count, banana_count];

        for food_count in bread_apple_banana {
            tx2.send(food_count).unwrap();
        }
    });

    // An error occured: receiving on an empty channel!
    match &rx2.try_recv() {
        Ok(value) => println!("Received from tx2: {:?}", value),
        Err(e) => println!("An error occured: {e}!")
    }

    // Received from tx2: 10
    match &rx2.recv() {
        Ok(value) => println!("Received from tx2: {:?}", value),
        Err(e) => println!("An error occured: {e}!")
    }
}