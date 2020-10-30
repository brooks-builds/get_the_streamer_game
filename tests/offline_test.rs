#[test]
fn main() {
    get_the_streamer_game::run_game(Some(get_the_streamer_game::RunConfig{
        test_bot_chatters: 5,
        test_command_occurences: &[("fire", 1), ("sword", 1), ("snake", 1), ("heart", 1)],
        attach_to_twitch_channel: false
    }));
}
