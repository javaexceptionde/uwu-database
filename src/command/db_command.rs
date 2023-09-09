use std::io::Write;
use std::net::TcpStream;
use crate::command::command::{Command, SubCommands};
use crate::database::database::{create_collection, get_collection, get_database};

struct DBCommand {
}
impl Command for DBCommand {
    fn handle_command(command: String, commandArg: String, subcommands: Vec<SubCommands>, mut stream: TcpStream) {
        if subcommands.len() < 1 {
            stream.write(b"ERR wrong number of arguments for 'db' command").unwrap();
            return;
        }
        let database_name = commandArg;
        if !is_valid_database_name(&database_name) {
            stream.write(b"ERR invalid database name").unwrap();
            return;
        }
        let database = get_database(database_name);
        if database.is_none() {
            stream.write(b"ERR database not found").unwrap();
            return;
        }
        let first_subcommand = &subcommands[0].command;
        if first_subcommand.eq_ignore_ascii_case("createCollection") {
            CreateCollectionCommand::handle_command(command, database.unwrap().name, subcommands, stream);
        } else {
            stream.write(b"ERR unknown subcommand for 'db' command").unwrap();
        }
    }
}

struct CreateCollectionCommand {
}

impl Command for CreateCollectionCommand {
    fn handle_command(command: String, commandArg: String, subcommands: Vec<SubCommands>, mut stream: TcpStream) {
        let database_name = commandArg;
        let collection_name = subcommands[0].args.clone();
        let collection = get_collection(&database_name, collection_name);
        if collection.is_some() {
            stream.write(b"ERR collection already exists").unwrap();
            return;
        }
        unsafe {
            create_collection(database_name, collection.unwrap().name)
                .expect("TODO: panic message");
        }
        stream.write(b"OK").unwrap();
    }
}

fn is_valid_database_name(database: &String) -> bool {
    return database.chars().all(|c| c.is_alphanumeric() || c == '_');
}