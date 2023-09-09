use std::net::TcpStream;

pub(crate) trait Command {
    fn handle_command(
        command: String,
        command_arg: String,
        subcommands: Vec<SubCommands>,
        stream: TcpStream,
    );
}

pub(crate) struct SubCommands {
    pub(crate) command: String,
    pub(crate) args: String,
}
