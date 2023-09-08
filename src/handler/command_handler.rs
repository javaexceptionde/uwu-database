pub fn command_handler(command_args: Vec<String>) {
    for arg in command_args {
        println!("{}", arg);
        let start_index = arg.find("(");
        let end_index = arg.rfind(")");
        match (start_index, end_index) {
            (Some(start_index), Some(end_index)) => {
                let command = arg[..start_index].to_string() + &arg[end_index + 1..];
                let args = &arg[start_index + 1..end_index];
                println!("Command: {}", command);
                println!("Args: {}", args);
            }
            _ => {
                println!("Invalid command: {}", arg);
            }
        }
    }
}