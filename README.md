# KDMP

A music player in development that focuses on being purely keyboard driven. Designed to work well with tiling window managers such as Sway and i3.

Works in client/server architecture, where the server daemon runs in the background waiting for the client to send requests. The user can then open a dmenu inspired client, which will then issue a music oriented command to the server, such as playing or pausing a song. Once the command has been sent, the client will immediately close, getting out of the users way.
