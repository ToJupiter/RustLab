use std::{thread, time::Duration, vec};

use trpl::{Either, Html, Receiver, StreamExt};

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

}

pub fn calling_async_exec() {
    // Calling async message passing
    trpl::block_on(async_message_passing());

    // Calling async message passing with 2 producers and 1 consumer
    trpl::block_on(multiple_send_single_recv());

    // Continue execution pipeline
    trpl::block_on(fix_sync_block());

    // Executing streaming with async and demonstrate the StreamExt trait (upper level of Iterator and Future trait)
    trpl::block_on(stream_and_iter());
}

/* This part needs fixing */
pub async fn async_message_passing() {
    let (tx, mut rx) = trpl::channel::<i32>();

    let val: i32 = 10;
    tx.send(val).unwrap();
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

    drop(tx2);

    while let Some(value) = rx2.recv().await {
        println!("Received {}", value);
    }
}

pub async fn multiple_send_single_recv() {
    let (tx, mut rx) = trpl::channel();

    let tx1 = tx.clone();
    let tx1_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("rust"),
            String::from("programming"),
            String::from("language")
        ];
        for val in vals {
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("Received this value: {}", value);
        }
    };

    let tx_fut = async move {
        let vals = vec![
            String::from("This"),
            String::from("is"),
            String::from("another"),
            String::from("welcoming"),
            String::from("message")
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::join!(tx1_fut, tx_fut, rx_fut);
}

/* Runtime handling for slow operations inside async block */
fn slow_operation(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{} slept for {}", name, ms);
}

async fn fix_sync_block() {
    let one_ms = Duration::from_millis(1);
    
    let a = async {
        println!("Started a");
        slow_operation("a", 20);
        trpl::sleep(one_ms).await;
        slow_operation("a", 30);
        trpl::sleep(one_ms).await;
        slow_operation("a", 50);
        trpl::sleep(one_ms).await;
        slow_operation("a", 100);
        trpl::sleep(one_ms).await;
    };

    let b = async {
        println!("Started b");
        slow_operation("b", 20);
        trpl::sleep(one_ms).await;
        slow_operation("b", 30);
        trpl::sleep(one_ms).await;
        slow_operation("b", 50);
        trpl::sleep(one_ms).await;
        slow_operation("b", 100);
        trpl::sleep(one_ms).await;
    };

    trpl::select(a, b).await;
}

/* Testing timeout for a website crawl for example.
 * The implementation of trpl::select is not fair: it always polls arguments in the order in which they are passed 
 */
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration
) -> Result<F::Output, Duration> {
    // Prioritize future_to_try over sleep
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time)
    }
}

pub async fn stream_and_iter() {
    let values = vec![10;20];
    let iter = values.iter().map(|n| n * 2);
    let mut stream = trpl::stream_from_iter(iter);

    while let Some(value) = stream.next().await {
        println!("The value was {}", value);
    }
}