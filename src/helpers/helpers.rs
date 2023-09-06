pub fn get_user_input(message: &str) -> String {
    println!("{}", message);
    let mut user_command: String = String::new();

    std::io::stdin()
        .read_line(&mut user_command)
        .expect("Command Failed");

    user_command
}
