# Haje

## Main goals

### Game Server 'lobby'

Haje needs to act as a game server 'lobby' of sorts. Game Servers can send a packet to Haje with their information and the secret key (defined via the env var `SERVER_KEY`).
Once the packet is received, Haje will keep the socket to the Game Server open, and reply with a succesful registration packet.

Afterwards, when an account requests servers, Haje will provide it with the list of servers. Upon choosing a server, Haje sends a packet to Nui to inform it about an account joining.

### Account manager

Haje needs to be able to manage accounts: Login, Register, Change password, Get info, Ban, Delete

These endpoints should be exposed via the public REST API :
* `/register`
* `/login`
* `/changepassword`
* `/account/{id}` (get info)

These endpoints should be exposed via the admin REST API :
* `/admin/ban/{id}?d={duration}`
* `/admin/delete/{id}`

These endpoints should be exposed via the TCP API :
* GetAccountPacket
* BanPacket
