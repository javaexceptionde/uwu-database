use std::net::TcpStream;

pub fn handle_db_command(db_command: String, remaining_args: Vec<String>, stream: TcpStream) {
    if !db_command.starts_with("db(") {
        println!("Invalid command: {}", db_command);
        return;
    }
    let mut db_command = db_command.replace("db(", "");
    db_command = db_command.replace(")", "");
    if !db_command.starts_with("\"") || !db_command.ends_with("\"") {
        println!("Invalid command: {}", db_command);
        return;
    }
    db_command = db_command.replace("\"", "");
    if !is_valid_database_name(&db_command) {
        println!("Invalid database name: {}", db_command);
        return;
    }

}

fn is_valid_database_name(database: &String) -> bool {
    return database.chars().all(|c| c.is_alphanumeric() || c == '_');
}