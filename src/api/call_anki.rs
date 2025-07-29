use std::collections::HashMap;

use anki_bridge::{
    AnkiClient, AnkiRequestable,
    prelude::{AddNoteEntry, AddNoteOptions, AddNoteRequest},
};


pub fn create_card(
    client: &AnkiClient<'_>,
    deck_name: &String,
    question: &String,
    answer: &String,
    tags: &Vec<String>,
) -> Result<i64, anki_bridge::Error> {
    let model_name = "Basic".to_string();

    let mut fields = HashMap::new();
    fields.insert("Front".to_string(), question.to_string());
    fields.insert("Back".to_string(), answer.to_string());

    let options = AddNoteOptions::default();

    let note = AddNoteEntry {
        deck_name: deck_name.to_string(),
        model_name: model_name.to_string(),
        fields: fields,
        options: options,
        tags: tags.to_vec(),
        audio: vec![],
        video: vec![],
        picture: vec![],
    };

    let request = AddNoteRequest { note };
    let note_id = client.request(request);

    return note_id;
}
