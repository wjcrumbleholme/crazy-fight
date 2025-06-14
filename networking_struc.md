# Networking Structure Initial Ideas

## Web client
- Only able to create rooms and connect to them / direct connect


## Local client
- Able to both create rooms and connect to them and also host for direct connections

## Server side
### Central (public)
- Needs to have a matchmaking server in front of it

### Private (self-hosted)



# Potential file structure
- main.rs - handles what state the application is lauched in 
- server
    - server_common.rs - All of the functions common to both servers
    - central.rs - Allow creating of rooms
    - private.rs - Just create one room and allow connections to it
    - matchmaking.rs - Handle the room listing and connection to a room

- client - Handles all of the ui elements

- game_logic - Handles the underlying logic


# Potential implementation
- Have the client connect to the matchmaking server if looking for a room (open websocket)
    - When client chooses a room to connect to, close the websocket to the matchmaking server and open a websocket to the game server
- If joining from a code, connect the client to the matchmaking server, look for a room with that code then if found disconect and connect to the relavent game server
- If direct connecting to a local server - just connect to server (open websocket)