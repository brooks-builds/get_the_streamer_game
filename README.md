get_the_streamer_game

# Todo

- [x] Integrate twitchchat wrapper crate
- [ ] Interface
  - [x] Title
  - [x] Partition off instruction area
  - [x] Show flame command
  - [ ] Show flame graphic
  - [ ] Partition off drop zones
  - [ ] Label each drop zone (a - z)
- [ ] Implement flame on flame command
  - [ ] when receiving `#flame-a` spawn a flame entity at position a
  - [ ] Have flame enitity drop
  - [ ] Have flame entity stop when it hits the ground
  - [ ] Have flame entity dissapear after 5 seconds
- [ ] Implement player
  - [ ] create player entity + graphic
  - [ ] player can move left and right
  - [ ] player can jump
- [ ] game is over after player is hit by an entity spawned by chat
- [ ] Create a second entity that chat can spawn
- [ ] Create a third entity that chat can spawn
- [ ] Create a fourth entity that chat can spawn
