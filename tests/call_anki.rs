use anki_bridge::AnkiClient;
use uuid::Uuid;

use autorecall::api::call_anki::create_card;

#[test]
fn test_create_card_success() {
    let client = AnkiClient::default();

    let uuid = Uuid::new_v4().to_string();

    let question = format!("What is the UUID? {}", uuid);
    let answer = "Unique ID".to_string();
    let tags = vec!["test".to_string()];
    let deck_name = "autorecall-test".to_string();

    let result = create_card(&client, &deck_name, &question, &answer, &tags);

    match result {
        Ok(note_id) => {
            println!("Card created: ID = {}", note_id);
            assert!(note_id > 0);
        }
        Err(err) => panic!("Card creation failed: {}", err),
    }
}

#[test]
fn test_create_card_duplicate() {
    let client = AnkiClient::default();

    let uuid = Uuid::new_v4().to_string();

    let question = format!("What is the UUID? {}", uuid);
    let answer = "Unique ID".to_string();
    let tags = vec!["test".to_string()];
    let deck_name = "autorecall-test".to_string();

    let _ = create_card(&client, &deck_name, &question, &answer, &tags);

    // recall with same que to trigger duplication
    let recall_result = create_card(&client, &deck_name, &question, &answer, &tags);

    match recall_result {
        Ok(note_id) => println!("Card created: ID = {}", note_id),
        Err(err) => {
            println!("Card creation failed: {}", err.to_string());
            let msg = err.to_string();
            assert!(msg.contains("duplicate"));
        }
    }
}
