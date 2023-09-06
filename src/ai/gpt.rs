use crate::helpers::helpers;

struct ChatData {
    model: String,
    messages: Vec<Message>,
}

struct Message {
    role: String,
    content: String,
}

enum Role {
    System,
    User,
}
enum AiTool {
    Chat,
    Back,
}

pub fn gpt(instruction: &str) {
    println!("GPT MODEL RUNNING....");
    print!("");
    println!("New AI: {}", instruction);

    let user_command: String = helpers::get_user_input();

    let command: &str = user_command.trim();

    let command: AiTool = match command {
        "chat" => AiTool::Chat,
        "back" => AiTool::Back,
        _ => panic!("Invalid command"),
    };

    match command {
        AiTool::Chat => {
            println!("Activating Chat!");
        }
        AiTool::Back => {
            println!("Going back!");
        }
    }
}
