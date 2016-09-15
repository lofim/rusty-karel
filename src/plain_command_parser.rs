use program::Program;
use program::CommandType;

pub fn parse_program(program_contents: &str) -> Result<Program, String>{
    let lines = program_contents.lines();
    let mut commands: Vec<CommandType> = vec![];

    for line in lines {

        match parse_command_line(line) {
            Some(command) => commands.push(command),
            None => ()
        }
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
