mod api;
mod dict;

use api::WiktionaryApi;

pub async fn get_word(word: &str) -> Result<dict::Word, Box<dyn std::error::Error>> {
    let api = WiktionaryApi::new();
    api.get_word(word).await
}
