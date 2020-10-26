use rand::Rng;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use twitch_chat_wrapper::chat_message::ChatMessage;
use twitchchat::messages::Privmsg;
use twitchchat::FromIrcMessage;

pub fn run(
  send_incoming_chat_message: Sender<ChatMessage>,
  num_bot_users: u64,
  initial_commands_delay: Duration,
  min_command_interval_ms: u64,
  max_command_interval_ms: u64,
) -> () {
  thread::spawn(move || {

    //The commands we'll be selecting from and the number of times each will be in the selection pool
    let command_occurences = [("fire", 2), ("sword", 5), ("snek", 3), ("heart", 1)];
    
    //Generate command pool
    let command_pool = command_occurences.iter().fold(Vec::<&str>::new(), |mut v, c|{ v.extend(vec![c.0;c.1].iter()); return v;});
        
    let mock_user_names: Vec<String> = (1..num_bot_users)
      .map(|u| format!("test_bot{}", u))
      .collect();
    let mut rng = rand::thread_rng();

    thread::sleep(initial_commands_delay);

    loop {
      let input = format!(
        ":{mock_user}!user@host PRIVMSG #mock_channel :#{command} {column}\r\n",
        mock_user = mock_user_names[rng.gen_range(0, mock_user_names.len())],
        command = command_pool[rng.gen_range(0, command_pool.len())],
        column = rng.gen_range(0, 10)
      );
      let (_pos, msg_a) = twitchchat::irc::parse_one(&input).unwrap();

      let test_msg: Privmsg = Privmsg::from_irc(msg_a).unwrap();
      send_incoming_chat_message
        .send(ChatMessage::new(test_msg))
        .unwrap();

      thread::sleep(Duration::from_millis(
        rng.gen_range(min_command_interval_ms, max_command_interval_ms),
      ));
    }
  });
}
