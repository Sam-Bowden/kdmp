# KDMP

A music player in development that focuses on being purely keyboard driven. Designed to work well with tiling window managers such as Sway and i3.

KDMP will operate in a client/server architecture. The user will open a dmenu inspired client, which will issue a music oriented command to the server daemon, such as playing or pausing a song. Once the command has been sent, the client will immediately close, getting out of the users way.
