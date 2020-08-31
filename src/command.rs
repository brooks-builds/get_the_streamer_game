use ggez::GameResult;

#[derive(PartialEq)]
pub enum Command {
    Flame(u8),
}

impl Command {
    /// Parse out the message and turn it into a command
    /// Message will look like `#flame 5`
    /// the number is the column and will go into the tuple
    /// For example
    /// let command = Command::new("#flame 5").unwrap();
    /// assert_eq(command, Command::Flame(5));
    pub fn new(message: &str) -> Result<Option<Command>, &'static str> {
        if !message.starts_with('#') {
            return Ok(None);
        }

        let mut parts = message.split(' ');
        if let Some(command) = parts.next() {
            match command {
                "#flame" => {
                    if let Some(id) = parts.next() {
                        let id: u8 = match id.parse() {
                            Ok(number) => number,
                            Err(error) => return Err("I coundn't tell what column to drop into"),
                        };
                        Ok(Some(Command::Flame(id)))
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
