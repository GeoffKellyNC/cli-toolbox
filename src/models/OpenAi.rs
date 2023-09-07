use reqwest;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Deserialize, Debug)]
pub struct AiUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    index: i32,
    pub message: Message,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct AiResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: HashMap<AiUsage, u64>,
}

enum AiRoles {
    System,
    User,
}

impl AiRoles {
    fn as_str(role: &AiRoles) -> String {
        match role {
            AiRoles::System => String::from("system"),
            AiRoles::User => String::from("user"),
        }
    }
}

pub struct OpenAi {
    api_key: String,
    engine: String,
    base_url: String,
    context: Vec<HashMap<String, String>>,
    tokens_used: u64,
    price_rate: HashMap<String, f64>,
    headers: HashMap<String, String>,
}

impl OpenAi {
    pub fn new(api_key: String, engine: String) -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("Authorization"), format!("Bearer {}", api_key));

        Self {
            api_key,
            engine,
            base_url: String::from("https://api.openai.com/v1"),
            context: vec![{
                let mut map = HashMap::new();
                map.insert(String::from("role"), String::from("system"));
                map.insert(
                    String::from("content"),
                    String::from("You are an AI here to help me answer basic questions."),
                );
                map
            }],
            tokens_used: 0,
            price_rate: [(String::from("gpt-3.5-turbo"), 0.000002)]
                .iter()
                .cloned()
                .collect(),
            headers,
        }
    }

    pub fn get_engine(&self) -> &str {
        &self.engine
    }

    pub fn add_to_context(&mut self, response: HashMap<String, String>) {
        self.context.push(response);
    }

    pub fn get_context(&self) -> &Vec<HashMap<String, String>> {
        &self.context
    }

    pub async fn ask_chat_model(
        &mut self,
        user_text: String,
    ) -> Result<AiResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let user_message: HashMap<String, String> = {
            let mut map = HashMap::new();
            map.insert(String::from("role"), AiRoles::as_str(&AiRoles::User));
            map.insert(String::from("content"), user_text);
            map
        };

        self.add_to_context(user_message);

        let payload = json!({
            "model": &self.engine,
            "messages": &self.get_context()
        });

        let response = client
            .post(format!("{}/chat/completions", &self.base_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            // Print the raw response
            let raw_response = response.text().await?;
            println!("Raw Response: {}", raw_response);

            // Now attempt to deserialize
            let ai_response: AiResponse = serde_json::from_str(&raw_response)?;
            Ok(ai_response)
        } else {
            panic!("Error: {}", response.status());
        }
    }
}
