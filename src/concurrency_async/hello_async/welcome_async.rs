use trpl::Html;

pub async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    
    // Another way to write this
    let response_text_v2 = trpl::get(url).await.text().await;
    let n = response_text_v2.as_bytes().len().min(100);
    let mut buffer = [0u8; 100];
    buffer[0..n].copy_from_slice(&response_text_v2.as_bytes()[0..n]);
    println!("100 first words of the response text: {:#?}", buffer);
    
    return Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
}