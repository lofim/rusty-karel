use program::CommandType;
use program::Program;

pub fn parse_program(program_contents: &str) -> Result<Program, String> {
    let lines = program_contents.lines();
    let commands = lines
        .filter_map(parse_command_line)
        .collect::<Vec<CommandType>>();

    if commands.is_empty() {
        return Err("Program does not contains any valid commands!".to_owned());
    }

    Ok(Program::new(commands))
}

fn parse_command_line(program_line: &str) -> Option<CommandType> {
    match program_line.to_lowercase().trim() {
        "turn" => Some(CommandType::TurnLeft),
        "move" => Some(CommandType::Move),
        "put" => Some(CommandType::PutBeeper),
        "pick" => Some(CommandType::PickBeeper),
        _ => None,
    }
}
