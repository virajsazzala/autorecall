use serde::Deserialize;

use anki_bridge::AnkiClient;
use ollama_rs::Ollama;

use autorecall::api::call_anki::create_card;
use autorecall::llm::call_ollama::{create_prompt, send_ollama_request};

#[derive(Debug, Deserialize)]
struct Card {
    question: String,
    answer: String,
    #[serde(default)]
    id: Option<i64>,
}

#[tokio::main]
async fn main() {
    let host = "http://localhost";
    let port = 11434;
    let model = "llama3:latest";
    let deck_name = "autorecall-test".to_string();

    let client = AnkiClient::default();
    let ollama = Ollama::new(host, port);

    let prompt = create_prompt(
        "suppose u want to find the  tree. can we use compression for this? let's take zip for example, how zip works is it find repeated patters, it'll create a reference and point to it. so, if we have two genome, like human and chimpanzee. we compress them, take the length of them. then we concat them. then take that concatanted length. so, if concat length is closer to one of them, it means they are similar as the size reduced. so, we can use this for finiding similarity. it can be applied to languages, etc",
    );

    let res = send_ollama_request(ollama, model, prompt.as_str())
        .await
        .unwrap();

    let mut cards: Vec<Card> = serde_json::from_str(&res).unwrap();

    for card in &mut cards {
        let card_response = create_card(
            &client,
            &deck_name,
            &card.question,
            &card.answer,
            &vec!["testing".to_string()],
        );

        match card_response {
            Ok(id) => {
                card.id = Some(id);
            }
            Err(e) => {
                eprintln!("[ERROR (NOTE CREATION)] {}", e);
                continue;
            }
        };
    }

    println!("{:#?}", cards);
}
