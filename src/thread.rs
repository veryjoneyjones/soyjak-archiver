use scraper::{Html, Selector};

use crate::file::File;

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Board {
    Soy,
    QuestionsAndAnswers,
    Raid,
    ShartyStation,
    Requests,
    Soy2,
    TheDollHouse,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Board::Soy => "soy",
                Board::QuestionsAndAnswers => "qa",
                Board::Raid => "raid",
                Board::ShartyStation => "ss",
                Board::Requests => "r",
                Board::Soy2 => "soy2",
                Board::TheDollHouse => "tdh",
            }
        )
    }
}

#[derive(Debug)]
pub struct Thread {
    board: Board,
    id: u64,
    html: Html,
}

impl Thread {
    pub async fn fetch(board: Board, id: u64) -> Self {
        let thread_url = format!("https://soyjak.st/{}/thread/{}.html", board, id);

        let raw_html = reqwest::get(thread_url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let html = Html::parse_document(&raw_html);

        Self { board, id, html }
    }

    pub fn get_images(&self) -> Vec<File> {
        let target_div_selector = Selector::parse("body > form:nth-child(13)").unwrap();

        let mut vec = Vec::new();

        if let Some(div) = self.html.select(&target_div_selector).next() {
            let anchor_selector = Selector::parse("a").unwrap();

            for element in div.select(&anchor_selector) {
                if element.children().any(|e| {
                    if let Some(element) = e.value().as_element() {
                        element.name() == "img"
                    } else {
                        false
                    }
                }) {
                    vec.push(File::new(format!(
                        "https://soyjak.st{}",
                        element.attr("href").unwrap_or_default().to_owned()
                    )));
                }
            }
            vec
        } else {
            vec![]
        }
    }
}
