# Escape the Dark Castle 

Project to learn the Bevy game engine in rust using the Escape the Dark Castle game as a template. This project is for educational purposes only and if you want to play Escape the dark castle go purchase a copy from ThemeBorne. :) 

https://www.ultraboardgames.com/escape-the-dark-castle/game-rules.php

# Architecture

## Game Server

To support multiple games running the game server will be spawning lightweight bevy apps on game creation. These will live for the lifetime of a game and also automatically shutdown after a period of time so the server doesn't get overloaded. The bevy apps will be "headless" as in they won't have any rendered UI server side outside of logs. Usage of bevy is to leverage its Entity / Component system as well as its game loop mechanics. Even though bevy is still in beta these pieces are already well in place and simple to use.

```mermaid

```

## Command / Event Transport

Since the server is headless it will need an API of sorts to facilitate game actions with the UI implementing these messages. These will need to transcend bevy as one of the actions will actually be to spawn a new game.

```mermaid
```

## Frontend UI

The frontend will be fully decoupled from the game server with its only requirement being that it implements the commands and events that the server communicates with. This allows a web based interface to be built, CLI, or any other method you can think of.

## Configuration

This is a side project so the configuration will have a max amount of concurrent games running to keep costs low. 

## Assets

Resources used to create sprites: https://www.piskelapp.com/
