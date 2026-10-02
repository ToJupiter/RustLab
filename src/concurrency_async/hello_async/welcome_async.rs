use std::time::Duration;

use trpl::{Either, Html, Receiver};

pub async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    
    // Another way to write this
    let response_text_v2 = trpl::get(url).await.text().await;
    let n = response_text_v2.as_bytes().len().min(100);
    let mut buffer = [0u8; 100];
    buffer[0..n].copy_from_slice(&response_text_v2.as_bytes()[0..n]);
    println!("100 first words of the response text: {:#?}", buffer);
    
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    return (url, title);
}

/*
 * `main` function is not allowed to be `async`. Because we need a runtime to start the async process 
 */
pub async fn work_with_args() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await.1 {
        Some(title) => println!("The title for {} was {}", url, title),
        None => println!("{} had no title", url)
    }
}

pub fn start_without_async() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await.1 {
            Some(title) => println!("The title for {} was {}", url, title),
            None => println!("{} has no title", url)
        }
    });

    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{} returned first", url);
        match maybe_title {
            Some(title) => println!("Its page title is {}", title),
            None => println!("No title")
        }
    });

    // Comparable to thread API
    /*
     * This version stops as soon as the for loop in the body of the main async block finishes, because the task spawned by spawn_task is shut down when the main function ends.
     */
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }

            for i in 1..5 {
                println!("Hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        match handle.await {
            Ok(async_output) => println!("The async output is: {:?}", async_output),
            Err(e) => println!("An error occured with threaded async {e}")
        }

        // Joining futures together
        let fut1 = async {
            for i in 1..10 {
                println!("Hi number {} from the first task", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };


        let fut2 = async {
            for i in 1..5 {
                println!("Hi number {i} from the second task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
        
    });

    // Calling async message passing
    trpl::block_on(async_message_passing());
    
}

/* This part needs fixing */
pub async fn async_message_passing() {
    let (tx, mut rx) = trpl::channel::<i32>();

    let val: i32 = 10;
    let received = rx.recv().await.unwrap();
    println!("We received {}", received);
    

    let (tx2, mut rx2) = trpl::channel::<String>();
    let vals_2 = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];

    for val in vals_2 {
        tx2.send(val).unwrap();
        trpl::sleep(Duration::from_micros(500)).await;
    }

    while let Some(value) = rx.recv().await {
        println!("Received {}", value);
    }
}



