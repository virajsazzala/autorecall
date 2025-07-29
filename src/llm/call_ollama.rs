use anyhow::Result;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;

pub async fn send_ollama_request(client: Ollama, model: &str, prompt: &str) -> Result<String> {
    let model = model.to_string();
    let prompt = prompt.to_string();

    let req = GenerationRequest::new(model.clone(), prompt);
    let res = client.generate(req).await?;

    Ok(res.response.to_lowercase())
}

pub fn create_prompt(text: &str) -> String {
    let prompt = format!(
    r#"You are an assistant that generates flashcard content for Anki.
    
    Given the following sentence or passage, do the following:

    - Identify all facts, technical terms, definitions, numbers, dates, and specific details a learner might need to memorize. For each, create a question–answer pair suitable for an Anki flashcard.
    - Identify important concepts or terms that would benefit from a simple explanation. For each, create a flashcard asking for a simple explanation and provide a concise answer.
    - If you believe a question is important or highly relevant but the answer is NOT DIRECTLY in the text, answer it only if you can using your own general knowledge. If you cannot confidently answer, do NOT create a flashcard for that question.
    - Be thorough! Extract as many meaningful question-answer pairs as possible, but generate a card ONLY when you can provide a clear, useful answer.
    - Do not create cards for trivial, self-evident, unimportant, or unanswerable information.

    Format your output strictly as a JSON array of objects. Each object must contain:
    - "question" (front of the card)
    - "answer" (back of the card)

    Answers should be concise: a phrase or a brief clear explanation. If a concise, correct answer is not possible, skip that question entirely (do NOT include a card for it).

    Input Sentence: "{}"
    
    Example output:
    [
        {{ "question": "How long is the Great Wall of China?", "answer": "Over 13,000 miles" }},
        {{ "question": "Where is the Great Wall of China located?", "answer": "Northern China" }},
        {{ "question": "What is the Great Wall of China?", "answer": "A historic fortification built to protect China from northern invaders." }}
    ]

    Please output only the JSON array with no extra text."#,
        text.to_string()
    );

    return prompt;
}
