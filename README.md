# Pong
A simple clone of the game Pong, made using the Bevy game engine. 

This game can be played locally or in your browser via [WebAssembly](https://amkillam.github.io/pong/).

# Purpose
The code for this game was written purely to familiarize myself with the Bevy game engine and basic ECS concepts. As such, the code was written fairly haphazardly in the span of a few hours. 

# Notes
As a result of rushed implementation, there are unsurprisingly several bugs and quirks left in the game. 

- On game initialization, both paddles spawn within the dotted line, then quickly are moved to their respective sides
- The game has no audio
- The "P1/P2 wins!" screen looks rather crowded
- The ball does not bounce off of the tops and bottoms of the paddles at the angle expected - the ball changes direction as if it hit the front of the paddle
