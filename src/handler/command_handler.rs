use std::net::TcpStream;
use crate::command::command::{Command, SubCommands};
use crate::command::db_command::DBCommand;

pub fn command_handler(command_args: Vec<String>, stream: TcpStream) {
    let mut subcommands: Vec<SubCommands> = Vec::new();
    for arg in command_args {
        println!("{}", arg);
        let start_index = arg.find("(");
        let end_index = arg.rfind(")");
        match (start_index, end_index) {
            (Some(start_index), Some(end_index)) => {
                let command: String = arg[..start_index].to_string() + &arg[end_index + 1..];
                let args = &arg[start_index + 1..end_index];
                let sub_command = SubCommands {
                    command: command.clone(),
                    args: args.to_string()
                };
                subcommands.push(sub_command);
                println!("Command: {}", command);
                println!("Args: {}", args);
            }
            _ => {
                println!("Invalid command: {}", arg);
            }
        }
    }
    let _command = subcommands.get(0).unwrap();
    if _command.command.eq_ignore_ascii_case("db") {
        DBCommand::handle_command(_command.command.clone(), _command.args.clone(), subcommands, stream);
    }
}