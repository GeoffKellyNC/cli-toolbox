use crate::helpers::helpers;
use crate::models::OpenAi::{AiResponse, OpenAi};
use std::env;
use std::error::Error;

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

            let response_result: Result<AiResponse, Box<dyn Error>> =
                open_ai.ask_chat_model(user_text).await;

            match response_result {
                Ok(response) => {
                    println!("Response: {}", response.choices[0].message.content);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        AiTool::Back => {
            println!("Going back!");
        }
    }
}
