use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;

use crate::dict::Word;
pub struct WiktionaryApi;

impl WiktionaryApi {
    pub fn new() -> Self {
        WiktionaryApi
    }

    pub async fn get_word(&self, word: &str) -> Result<Word, Box<dyn std::error::Error>> {
        let url = format!(
            "https://en.wiktionary.org/w/api.php?action=query&prop=extracts&format=json&titles={}",
            word
        );

        let client = Client::new();
        let response = client.get(&url).send().await?;

        let response: QueryResponse = response.json().await?;
        println!("{:?}", response);
        let page = response.query.pages.values().next().unwrap();

        Ok(Word {
            word: page.title.clone(),
            extract: page.extract.clone().unwrap_or_default(),
        })
    }
}

#[derive(Deserialize, Debug)]
struct QueryResponse {
    query: Query,
}

#[derive(Deserialize, Debug)]
struct Query {
    pages: HashMap<String, Page>,
}

#[derive(Deserialize, Debug)]
struct Page {
    title: String,
    extract: Option<String>,
}
