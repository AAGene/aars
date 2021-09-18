# ArcheAge.RS

ArcheAge.RS is a Rust implementation/emulation of the server for the MMORPG ArcheAge.
AARS aims to be cross compatible with another (and currently, only) other ArcheAge emulator, [AAEmu](https://github.com/AAEmu/AAEmu)


## Project hierarchy

| Project Name  | AAEmu equivalent  | Progress  |
| ------------- |-------------------| ---------:|
| Thiol         | AAEmu             | N/A       |
| Shatigon      | AAEmu.Commons     | N/A       |
| Haje          | AAEmu.Login       | WIP       |
| Nui           | AAEmu.Game        | TODO      |


## Haje

Haje is an authentication server. It handles accounts and registering game servers.

Game Server registration is done via network packets over the "internal" network. 

Accounts should be logged in by token. Endpoints are exposed over HTTP(s) returning a JWT, which the game client can then pass in as a password. This differs from AAEmu (which simply requires the password) making launchers incompatible between the two. The JWT approach is favored because it is also valid for website authentication.

Accounts are solely managed by the auth server, and no outside source (website) should modify them. Should you want to store information about an account, you can expand the auth server itself.

Haje also supports some commands via the CLI, to make using it in local with friends easier.

### Commands

`register <account name> <password> [accessLevel=0] [email='']` - Creates a new account with the specified username and password. accessLevel is optional and defaults to 0 (player access). Same for email.

`changepw <account name> <new password>` - Changes the password of the specified account

`changeaccess <account name> <access level>` - Changes the access level of the specified account 

`ban <account name> [duration=-1]` - Bans the specified account for `duration` minutes. Default is -1 which is infinite

`unban <account name>` - Unbans the specified account

`display <account name>` - Displays all known information (by the auth server) about the specified account

## Nui

// TODO

## Shatigon

Represents the shared code (packet serialization for example) between Nui and Haje.
