use trpl::{Either, Html};

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
    })
    
}

