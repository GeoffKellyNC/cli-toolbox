pub fn get_user_input() -> String {
    let mut user_command: String = String::new();

    std::io::stdin()
        .read_line(&mut user_command)
        .expect("Command Failed");

    user_command
}
