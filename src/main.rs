use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let head = Html::parse(&response_text)
        .select_first("head")
        .map(|title_element| title_element.inner_html());
    (url, head)
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let web_url_one = page_title(&arguments[1]);
        let web_url_two = page_title(&arguments[2]);

        let (url, maybe_head) = match trpl::race(web_url_one, web_url_two).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first!");
        match maybe_head {
            Some(head) => println!("head tag of given url is: \n{head}"),
            None => println!("it's head tag could not be parsed"),
        }
    });
}
