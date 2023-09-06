mod ai;

use ai::gpt;


#[derive(Debug)]
enum ActiveTool {
    Ai,
    SmartDevice,
    Pokemon,
    Settings,
    Exit
}

fn main(){
    loop {
        println!("Welcome to Custom Tools!");
        println!("");
        print!("");

        let mut user_command: String = String::new();

        std::io::stdin().read_line(&mut user_command).expect("Command Failed");

        let user_command: &str = &user_command.trim();

        let command: ActiveTool = match user_command { 
            "gpt" => ActiveTool::Ai,
            "device" => ActiveTool::SmartDevice,
            "poke" => ActiveTool::Pokemon,
            "settings" => ActiveTool::Settings,
            "exit" => ActiveTool::Exit,
            _ => {
                println!("Unknown Command");
                continue
            }
        };

        match command {
            ActiveTool::Ai => {

                let mut ai_action: String = String::new();

                println!("");

                println!("Ai Instruction: ");

                std::io::stdin().read_line(&mut ai_action).expect("Ai Action Failed");

                let ai_action: &str = &ai_action;

                gpt::gpt(ai_action);

                break;
            }
            ActiveTool::Pokemon => {
                println!("Pokemon Chosen!");
                break;
            }
            ActiveTool::SmartDevice => {
                println!("Smart Devices Chosen!");
                break
            }
            ActiveTool::Settings => {
                println!("Settings Active");
                break;
            }
            ActiveTool::Exit => {
                println!("Exiting Program!");
                break;
            }
        }


    }
}
