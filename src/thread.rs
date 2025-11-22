use scraper::{Html, Selector};

use crate::file::File;

/// Represents a board on the website
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Board {
    Soy,
    QuestionsAndAnswers,
    Raid,
    ShartyStation,
    Requests,
    Soy2,
    TheDollHouse,
    Minecraft,
    International,
    Politics,
    Anime,
    AnimalsAndNature,
    AllSports,
    MediaAndInterests,
    R9K,
    Technology,
    VidyaGames,
    Paranormal,
    Cacaborea,
    Cado,
    Gigachads,
    Jaks,
    Sneed,
    Sude,
    Nonsense,
    Meta,
    News,
    Archive,
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
                Board::Minecraft => "craft",
                Board::International => "int",
                Board::Politics => "pol",
                Board::Anime => "a",
                Board::AnimalsAndNature => "an",
                Board::AllSports => "asp",
                Board::MediaAndInterests => "mtv",
                Board::R9K => "r9k",
                Board::Technology => "tech",
                Board::VidyaGames => "v",
                Board::Paranormal => "x",
                Board::Cacaborea => "caca",
                Board::Cado => "cado",
                Board::Gigachads => "giga5",
                Board::Jaks => "jak",
                Board::Sneed => "sneed",
                Board::Sude => "sude",
                Board::Nonsense => "yyyyyyy",
                Board::Meta => "q",
                Board::News => "news",
                Board::Archive => "chive",
            }
        )
    }
}

/// Represents a thread with a board and id
#[derive(Debug)]
pub struct Thread {
    board: Board,
    id: u64,
    html: Html,
}

impl Thread {
    /// Fetches a thread from the specified board and id
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

    /// Fetches the images of a given thread into a vector of files
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
