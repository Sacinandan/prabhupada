use reqwest::{Client, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env::var;

#[derive(Serialize, Deserialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<Conversation>,
}

#[derive(Serialize, Deserialize)]
struct Conversation {
    role: String,
    content: String,
}

pub async fn gpt(text: String) -> String {
    let openai_key = var("OPENAI_API_KEY").unwrap();
    let url = "https://api.openai.com/v1/chat/completions";
    let content = format!("I want you to act as Bhaktivedanta Swami Prabhupada,\
    an Indian Gaudiya Vaishnava guru and spiritual teacher who founded ISKCON,\
    commonly known as the \"Hare Krishna movement\".\
    You will provide conversation with ISKCON member.\
    Answers have to be based only on Bhaktivedanta Swami Prabhupada teachings with quotes from his books.\
    You can answer with sanskrit quotes, which Bhaktivedanta Swami Prabhupada used.\
    Answer as first person and don't use phrases like \"As Bhaktivedanta Swami Prabhupada\", \"As Spiritual Teacher\" and etc.\
    Prefer to speak language of interlocutor.\
    \
    ISKCON member message:\
    {}", text);

    let request = OpenAIChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Conversation {
            role: "user".to_string(),
            content,
        }],
    };

    let client = Client::new();
    let response = client
        .post(url)
        .header(AUTHORIZATION, format!("Bearer {}", openai_key))
        .json(&request)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let json_response = response.json::<Value>().await;
                match json_response {
                    Ok(json) => {
                        json["choices"][0]["message"]["content"]
                            .as_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "Invalid response from OpenAI".to_string())
                    }
                    Err(err) => format!("Error parsing JSON response: {}", err),
                }
            } else {
                format!("Request failed with status code: {}", response.status())
            }
        }
        Err(err) => format!("Request failed: {}", err),
    }
}