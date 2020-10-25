# Get the Streamer Game

![screenshot](./screenshot.png)

## Setup

### Install Rust

Install Rust Stable at [rust-lang.org](https://www.rust-lang.org/).

### Set up your .env file

1. Copy the [`.env_example`](./.env_example) to `.env`
1. _TWITCH_NAME_ is the twitch user name (you will need a Twitch account)
1. _TWITCH_TOKEN_ is the oauth token, you can get one by visiting [https://twitchapps.com/tmi/](https://twitchapps.com/tmi/) and creating a new application. _Note, if you are streaming while setting this up, hide your screen during this step as your oauth token is essentially your password_
1. _TWITCH_CHANNEL_ is the Twitch channel name that you want the game to listen to

### Run the Game!

```sh
cargo run
```

## Want to help? New ideas?

New ideas and feature requests are done within the [issues page](https://github.com/brooks-builds/get_the_streamer_game/issues).

If you want to add a feature please feel free to open a pull request.
