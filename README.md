get_the_streamer_game

# Todo

- [x] Integrate twitchchat wrapper crate
- [x] Interface
  - [x] Title
  - [x] Partition off instruction area
  - [x] Show flame command
  - [x] Show flame graphic
  - [x] Partition off drop zones
  - [x] Label each drop zone (a - z)
- [x] Implement flame on flame command
  - [x] when receiving `#flame-1` spawn a flame entity at position 1
  - [x] Have flame enitity drop
  - [x] Have flame entity stop when it hits the ground
  - [x] Have flame entity dissapear after 5 seconds
- [ ] Implement player
  - [x] create player entity + graphic
  - [x] player can move left and right
  - [ ] player slows down to a halt when not holding a move key down
  - [ ] player can jump
  - [ ] player can stand on ground
- [ ] game is over after player is hit by an entity spawned by chat
- [ ] Create a second entity that chat can spawn
- [ ] Create a third entity that chat can spawn
- [ ] Create a fourth entity that chat can spawn
- [ ] Polish
  - [ ] When quitting the app, exit all processes
  - [ ] Scale up the fire sprite
  - [ ] Cut the player sprites so that we have left run, right run, and stand
  - [ ] Switch player to have multiple sprites based on which direction they are moving (can mirror sprites by scale using a negative number [from #BareDoodah])
- [ ] Clean up code
  - [ ] Make command.rs nicer
  - [ ] Extracting command in lib/update is not great :(
  - [ ] Not using children in game objects so remove them
