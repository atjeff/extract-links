use scraper::{Html, Selector};
use tokio::io::{self as async_io, AsyncReadExt};
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let mut input = String::new();
    let timeout_duration = Duration::from_secs(3);

    // Set a timeout for reading from stdin
    let result = time::timeout(
        timeout_duration,
        async_io::stdin().read_to_string(&mut input),
    )
    .await;

    match result {
        Ok(Ok(_)) => parse_html(&input), // Successfully read input
        Ok(Err(e)) => println!("Failed to read input: {}", e),
        Err(_) => {
            println!(
                "No input provided within timeout ({} seconds)",
                timeout_duration.as_secs_f32()
            );

            // Exit program
            std::process::exit(1);
        }
    }
}

fn parse_html(input: &str) {
    let document = Html::parse_document(input);

    if document.tree.nodes().len() == 0 && !document.errors.is_empty() {
        println!("Failed to parse HTML, errors: {:?}", document.errors);

        return;
    }

    let selector = Selector::parse("a").expect("Failed to find any links in provided HTML");

    for element in document.select(&selector) {
        if let Some(possible_link) = element.value().attr("href") {
            if possible_link.contains("https://") {
                println!("{}", possible_link);
            }
        }
    }
}
