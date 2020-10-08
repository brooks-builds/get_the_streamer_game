#!/bin/bash

echo "Please enter your Twitch username: "
read username

echo "Please enter your oauth token. You can get one at https://twitchapps.com/tmi/ (if you are streaming, please hide your screen as this is effectively a password): "
read token

echo "Please enter the Twitch channel you would like the game to listen to: "
read channel

echo "TWITCH_NAME=${username}" > .env
echo "TWITCH_TOKEN=${token}" >> .env
echo "TWITCH_CHANNEL=${channel}" >> .env

echo "\n.env file has been created!"