use std::collections::HashMap;

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
            base_url: String::from("https://api.openai.com/v1/"),
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
}
