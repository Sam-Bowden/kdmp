# KDMP

A music player in development that focuses on being purely keyboard driven. Its main focus is to allow the user to control playback of music in minimal time, allowing them to focus on their work. It is designed and intended to be used with a tiling window manager such as Sway or i3.

KDMP will operate in a client/server architecture. The user will open a dmenu inspired client, which will issue a music oriented command to the server daemon, such as playing or pausing a song. Once the command has been sent, the client will immediately close.

## Compatibility

KDMP is written with only Linux support in mind, however it will likely run on other Unix-like operating systems such as MacOS or FreeBSD.

## Compilation

As the client relies on GTK4, an installation of the GTK4 development package is required along with build essentials. As described on the gtk-rs website, these can be installed with the following commands on these major distributions.

Fedora based:
```bash
sudo dnf install gtk4-devel gcc
```

Debian based:
```bash
sudo apt install libgtk-4-dev build-essential
```

Arch based:
```bash
sudo pacman -S gtk4 base-devel
```

A rustup installation is also required. The instructions for doing this can be found at https://www.rust-lang.org/tools/install.

Once these are both installed, clone this repository and build using cargo:
```bash
git clone https://github.com/Sam-Bowden/kdmp
cd kdmp
cargo build --release
```
The client and the server daemon can then be started using:
```bash
cargo run --release -p daemon
cargo run --release -p client
```
The compiled binaries can be found in "target/release".

## Setup with Sway and i3

After compiling KDMP, open up your Sway/i3 config.

Firstly, make Sway/i3 start the KDMP daemon when loaded:
```
exec <DAEMON_BINARY_PATH>
```

Then, make Sway/i3 start the KDMP client when your chosen key combination is pressed:
```
bindsym <KEY_BINDING> exec <CLIENT_BINARY_PATH>
```

Finally, reload the Sway/i3 configuration file to start using KDMP.

## Using KDMP

Once you have setup up KDMP in your Sway/i3 config, press your chosen key combination to start the client.

The client will fail to load at first because the default music directory is not set. Navigate to your XDG_CONFIG_HOME directory (typically ~/.config/), then edit kdmp/kdmp.conf. Add your desired music directory and save.

Now you will be presented with the KDMP client, where you will be able to enter the following commands to control your music:

| Operation | Command |
| --- | --- |
| Begin Track | b <NAME_OF_Track> |
| Stop | s |
| Pause | p |
| Resume | r |
