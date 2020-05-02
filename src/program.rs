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
        Program { commands }
    }

    pub fn start(&self, world: &mut World) {
        let one_second = time::Duration::from_secs(1);
        let five_seconds = time::Duration::from_secs(5);

        self.clean_terminal();
        self.render_start_info();
        self.render(world);

        for (index, command) in self.commands.iter().enumerate() {
            let step_number = index + 1;
            // set timer
            thread::sleep(one_second);

            if let Err(err) = self.execute(command, world) {
                println!("There was an error in program execution on step {}, {:?}, continuing...", step_number, err);
                thread::sleep(five_seconds);
                continue;
            }

            self.clean_terminal();
            self.render_header(step_number);
            self.render(world);
        }

        println!("Program execution done.");
    }

    fn render_start_info(&self) {
        println!("Program loaded, executing...");
    }

    fn render_header(&self, step_number: usize) {
        println!("Executing step {}", step_number);
    }

    fn render(&self, world: &mut World) {
        println!("karel info {:?}", world.get_robot().info());
        println!("tiles {}", world.render());
    }

    fn clean_terminal(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn execute(&self, command: &CommandType, world: &mut World) -> Result<(), CommandError> {
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
