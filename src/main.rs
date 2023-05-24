use std::io::{self, Read};

use scraper::{Html, Selector};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let document = Html::parse_document(&input);

    if document.tree.nodes().len() == 0 && document.errors.len() > 0 {
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
