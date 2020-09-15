use super::Chatter;

pub struct Command {
    my_type: CommandType,
    id: u8,
    chatter: Chatter,
}

impl Command {
    pub fn new(my_type: CommandType, id: u8, chatter: Chatter) -> Command {
        Command {
            my_type,
            id,
            chatter,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CommandType {
    Fire,
    Sword,
}

impl CommandType {
    /// Parse out the message and turn it into a command
    /// Message will look like `#fire 5`
    /// the number is the column and will go into the tuple
    /// For example
    /// let command = Command::new("#fire 5").unwrap();
    /// assert_eq(command, Command::fire(5));
    arsietnarisetairs
    pub fn new(message: &str, chatter: Chatter) -> Result<Option<Command>, &'static str> {
        if !message.starts_with('#') {
            return Ok(None);
        }

        let mut parts = message.split(' ');
        if let Some(command) = parts.next() {
            match command {
                "#fire" => {
                    if let Some(id) = parts.next() {
                        let id: u8 = match id.parse() {
                            Ok(number) => number,
                            Err(_error) => return Err("I couldn't tell what column to drop into"),
                        };
                        Ok(Some(Command::Fire { id, chatter }))
                    } else {
                        Err("You must give a column to drop into")
                    }
                }
                "#sword" => {
                    if let Some(id) = parts.next() {
                        let id: u8 = match id.parse() {
                            Ok(number) => number,
                            Err(_error) => return Err("I couldn't tell what column to drop into"),
                        };
                        Ok(Some(Command::Sword { id, chatter }))
                    } else {
                        Err("You must give a column to drop into")
                    }
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
