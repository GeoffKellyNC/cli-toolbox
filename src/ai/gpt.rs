use crate::helpers::helpers;
use crate::models::OpenAi::{AiResponse, OpenAi};
use std::env;

enum AiTool {
    Chat,
    Back,
}

pub async fn gpt(instruction: &str) {
    println!("GPT MODEL RUNNING....");
    print!("");
    println!("New AI: {}", instruction);

    let user_command: String = helpers::get_user_input("Enter Ai Command:");

    let command: &str = user_command.trim();

    let command: AiTool = match command {
        "chat" => AiTool::Chat,
        "back" => AiTool::Back,
        _ => panic!("Invalid command"),
    };

    match command {
        AiTool::Chat => {
            println!("Activating Chat!");
            print!("");

            let api_key: String = env::var("OPENAI_API_KEY").unwrap();
            let engine: String = String::from("gpt-3.5-turbo");

            let mut open_ai: OpenAi = OpenAi::new(api_key, engine);

            let user_text: String = helpers::get_user_input("Qestion: ");

            let response_result = open_ai.ask_chat_model(user_text).await;

            match response_result {
                Ok(response) => {
                    println!("Model: {}", response.model);
                    if let Some(choice) = response.choices.get(0) {
                        if let Some(text) = choice.get("text") {
                            println!("Response: {}", text);
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
        AiTool::Back => {
            println!("Going back!");
        }
    }
}
