# Retrospective

This is going to be a retrospective on the Get the Streamer game. Recorded live on Twitch on 2020-10-28 at [https://twitch.tv/brookzerker](https://twitch.tv/brookzerker)

## Start

What am I not doing that I would like to start doing for the next project? This could also apply to any more work done on this project. Why? What benefit would I get from starting this?

- Running a design sprint before coding. I would have identified how the game would work earlier, which would lead to less re-writing of the codebase.
  - The sprite system assumed that all images being used were spritesheets that had the same type of character in it. This led to problems when I brought in the player because it had front, backwards, and left all in the same spritesheet. The heart had just one image, so treating it like a spritesheet was a bit overkill as well.
- Using a config file that would be loaded on start would be really helpful in the game for tweaking the feel, and adding / changing resources.
- Find time to work on the project during other parts of the day
  - Maybe find more times to stream during the day (after work?)
  - next stream show off what I did
  - keep commits really tight to the feature working on
- Create tools that can be used in future projects as opposed to just this one
  - system to create / load config files
  - design system for interfaces
- Try Bevy game engine
  - ECS system for organizing code
- Learn how to edit videos from multiple streams
- Play / explore more Rust-based games featured in the Rust Gamedev newsletter
- Advertise the stream and games I'm making
  - Rust Gamedev newsletter
  - Reddit weekly `what are you working on thread`
  - Reddit Rust gamedev
  - Twitter/Discord: Screenshots
- Participate in the Rust community more
- Learn how to separate code that interfaces with the engine from core game logic. This would allow me to write tests.
- Set up CI/CD system
  - auto build for different platforms (Mac, Windows, Linux)
  - auto run tests
  - auto lint using clippy
  - run rust fmt
  - if any of these fail, don't allow merging into master

## Stop

What am I doing that I would like to stop doing from now on? Why? Should we iterate on this and start doing something slightly different?

- Trying to work on multiple projects at the same time. They ended up being too many different things. With only an hour or so before work to work on the project I was feeling that I taking a long time to finish features. There was some pressure internally to switch to new features quickly, and therefore code quality suffered to make that happen.
- VODS based on streams, and focus on features at a time
  - VOD would be based on a feature, named after that feature and therefore be shorter

## Continue

What am I doing that I want to ensure that I continue doing?

- Working only on one stream project at a time
- Uploading full stream archives
- Track milestones
  - This requires knowing what done means MVP-wise
  - Creating issues to track work to be done on GitHub

## Observations

What have I noted about this project that is interesting?

- There was a lot of feature creep in the project. This may be due to the lack of design.
- This was my second MVP game that has been completed.
- I considered creating a config-file driven game, but rejected because I assumed the game was not going to be that complex and would be finished in 2 weeks.
  - This assumption caused me to make implementation decisions that made the game take longer because I thought the game was simpler than it was.
- Began coding on August 26, declared MVP done on October 27.
- I was working on the game Monday - Thursday. Friday was working on learning web assembly, and Sunday I was creating an introduction to Rust tutorial series.
- 99% of the project was done live on stream
- Used Component pattern
  - but not for everything
- My programming skills have improved a lot more through these projects than just the learning projects from before

## Actions

What will I commit to doing for the next project?

- [ ] Create design sprint rules for myself
  - [ ] Determining what means success for the project
  - [ ] What metrics to collect that will show me if I'm successful?
- [ ] Create list of projects to work on
  - [ ] Set up CI/CDs system for Rust-based projects
  - [ ] Set up metric collecting and displaying for tracking success of future projects
  - [ ] Share with viewers, and get feedback on which of these projects you'd like to see
- [ ] Set up retro cadence
