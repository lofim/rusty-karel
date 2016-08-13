use helpers::Size;

#[derive(Clone, Debug)]
pub enum CommandType {
    Move,
    TurnLeft,
    PickBeeper,
    PutBeeper,
}

pub struct Program {
    commands: Vec<CommandType>,
    counter: Size,
}

impl Program {
    pub fn new(commands: Vec<CommandType>) -> Program {
        Program { 
            commands: Vec::new(),
            counter: 0,
        }
    }

    pub fn execute(&mut self) {
        // TODO: impl
    }
}
