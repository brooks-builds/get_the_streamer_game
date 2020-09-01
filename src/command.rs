use ggez::GameResult;

#[derive(PartialEq, Debug)]
pub enum Command {
    Fire(u8),
}

impl Command {
    /// Parse out the message and turn it into a command
    /// Message will look like `#fire 5`
    /// the number is the column and will go into the tuple
    /// For example
    /// let command = Command::new("#fire 5").unwrap();
    /// assert_eq(command, Command::fire(5));
    pub fn new(message: &str) -> Result<Option<Command>, &'static str> {
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
                            Err(error) => return Err("I coundn't tell what column to drop into"),
                        };
                        Ok(Some(Command::Fire(id)))
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
