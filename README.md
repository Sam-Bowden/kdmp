# KDMP

https://github.com/Samuel-Bowden/kdmp/assets/91887909/ae21fd81-8cf3-47b2-99f8-bb49a548ff41

A music player in development that focuses on being purely keyboard driven. The main focus is to allow a user to control playback of music in minimal time, allowing them to focus on their work. It is designed and intended to be used with a tiling window manager such as Sway or i3.

KDMP operates in a client/server architecture. The user opens a dmenu inspired client, which issues a music oriented command to the KDMP daemon, such as playing or pausing a song, which runs in the background without a client needing to be open.

## Compatibility

KDMP is written with only Linux support in mind.

## Compilation and Installation

A Rustup installation is required to compile and install KDMP. Instructions for doing this can be found at https://www.rust-lang.org/tools/install.

Once Rustup is installed, compile and install KDMP using Cargo with the following command:
```bash
cargo install --git https://github.com/Samuel-Bowden/kdmp kdmp-client kdmp-daemon
```

## Setup with Sway and i3

After installing KDMP, open up your Sway/i3 config.

Firstly, make Sway/i3 start the KDMP daemon when loaded:
```
exec kdmp-daemon
```

Then, make Sway/i3 start the KDMP client when your chosen key combination is pressed:
```
bindsym <KEY_BINDING> exec kdmp-client
# E.g. bindsym $mod+Shift+t exec kdmp-client
```

Finally, reload the Sway/i3 configuration file to start using KDMP.

## Using KDMP

Once you have setup up KDMP in your Sway/i3 config, press your chosen key combination to start the client.

Now you will be presented with the KDMP client, where you will be able to enter the following commands to control your music:

| Operation | Command |
| --- | --- |
| Play | pl <LOCATION> |
| Stop | s |
| Pause | ps |
| Resume | r |
| Next | n |
