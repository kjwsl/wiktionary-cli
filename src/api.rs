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
            "https://en.wiktionary.org/w/api.php?action=query&titles={}&prop=revisions&rvprop=content&format=json", word
        );

        let client = Client::new();
        let response = client.get(&url).send().await?;

        let response: QueryResponse = response.json().await?;
        let page = response.query.pages.values().next().unwrap();

        Ok(Word {
            word: page.title.clone(),
            extract: page.revisions[0].content.clone(),
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
    revisions: Vec<Revision>,
}

#[derive(Deserialize, Debug)]
struct Revision {
    #[serde(rename = "*")]
    content: String,
}
