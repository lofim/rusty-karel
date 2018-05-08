use world::World;

use std::{thread, time};

#[derive(Clone, Debug)]
pub enum CommandType {
    Move,
    TurnLeft,
    PickBeeper,
    PutBeeper,
}

#[derive(Clone, Debug)] 
pub enum CommandError {
    CommandError(CommandType),
}

#[derive(Debug)]
pub struct Program {
    commands: Vec<CommandType>,
}

impl Program {
    pub fn new(commands: Vec<CommandType>) -> Program {
        Program { 
            commands,
        }
    }

    pub fn start(&self, world: &mut World) {
        println!("Executing program...");
        self.render(world);

        for command in &self.commands {
            // set timer
            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);

            if let Err(err) = self.execute(command, world) {
                println!("There was an error in program execution {:?}", err);
            }
            self.render(world);
        }

        println!("Program execution done.");
    }

    fn render(&self, world: &mut World) {
        println!("karel info {:?}", world.get_robot().info());
        println!("tiles {}", world.render());
    }

    fn execute(&self, command: &CommandType , world: &mut World) -> Result<(), CommandError> {
        let owned_command = command.to_owned();
        
        let command_successful = match owned_command {
            CommandType::Move => world.move_robot(),
            CommandType::TurnLeft => world.get_robot().turn_left(),
            CommandType::PickBeeper => world.pick_beeper(),
            CommandType::PutBeeper => world.put_beeper(),
        };

        if !command_successful {
            return Err(CommandError::CommandError(owned_command));
        }

        Ok(())
    }
}
