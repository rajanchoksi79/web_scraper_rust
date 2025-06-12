use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let response_text = trpl::get(url).await.text().await;
        Html::parse(&response_text)
            .select_first("title")
            .map(|title_element| title_element.inner_html())
    }
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let web_url = &arguments[1];
        match page_title(&web_url).await {
            Some(title) => println!("The title of {web_url} is {title}"),
            None => println!("{web_url} has no title"),
        }
    });
}
